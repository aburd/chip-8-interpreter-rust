use rand::Rng;
use std::path::Path;
mod instructions;

use instructions::Instruction;

/// MEMORY: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
pub type Opcode = u16;
const OPCODE_SIZE: u16 = 2;
const USERSPACE_START: u16 = 0x200;
const USERSPACE_END: u16 = 0xFFF;

/// REGISTERS - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.2
const VF: usize = 0x0F;

enum ProgramCounterChange {
    Next,
    Skip,
    Jump(u16),
}

impl ProgramCounterChange {
    fn skip_if(cond: bool) -> Self {
        if cond {
            Self::Skip
        } else {
            Self::Next
        }
    }
}

fn init_pc_register() -> u16 {
    USERSPACE_START.clone()
}

/// KEYBOARD - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.3
/*
 * Layout:
 * 1 2 3 C
 * 4 5 6 D
 * 7 8 9 E
 * A 0 B F
 */
type Keys = [bool; 4 * 4];

/// DISPLAY - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.4
/*
*
* The original implementation of the Chip-8 language used a 64x32-pixel monochrome display with this format:

  (0,0)	(63,0)
  (0,31)	(63,31)

* The graphics of the Chip 8 are black and white and the screen has a total of 2048 pixels (64 x 32).
* This can easily be implemented using an array that hold the pixel state (1 or 0):
*/
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
type Gfx = [bool; SCREEN_WIDTH * SCREEN_HEIGHT];

/// SOUND - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.5
/**
* The original implementation of the Chip-8 language used a 64x32-pixel monochrome display with this format:

  (0,0)	(63,0)
  (0,31)	(63,31)
*/

/// FONTSET
const CHIP8_FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // ZERO
    0x20, 0x60, 0x20, 0x20, 0x70, // ONE
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // TWO,
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // THREE
    0x90, 0x90, 0xF0, 0x10, 0x10, // FOUR
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // FIVE
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // SIX
    0xF0, 0x10, 0x20, 0x40, 0x40, // SEVEN
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // EIGHT
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // NINE
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Cpu {
    memory: [u8; 4096],
    v: [u8; 16],
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    i: I,
    pixels: Gfx,
    keys: Keys,
    sound_timer: u8,
    delay_timer: u8,
    pub draw_flag: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: [0; 4096],
            v: [0; 16],
            pc: init_pc_register(),
            stack: [0; 16],
            sp: 0x00,
            i: u16,
            pixels: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            keys: Keys::default(),
            sound_timer: 0,
            delay_timer: 0,
            draw_flag: false,
        }
    }

    pub fn initialize(&mut self) {
        // Reset all pertinent memory
        self.memory = [0; 4096];
        self.v = [0; 16];
        self.pc = init_pc_register();
        self.stack = [0; 16];
        self.sp = 0x00;
        i: 0x0000,
        self.keys = Keys::default();
        self.draw_flag = false;

        // Reset screen
        self.execute(Instruction::Clear);

        // Load fontset
        for i in 0..80 {
            self.memory[i] = CHIP8_FONTSET[i];
        }

        // Reset timers
    }

    pub fn load_rom(&mut self, path_str: &str) -> std::io::Result<()> {
        let rom_path = Path::new(path_str);
        let file = std::fs::read(rom_path)?;
        for (i, byte) in file.iter().enumerate() {
            self.memory[(USERSPACE_START + i as u16) as usize] = *byte;
        }
        Ok(())
    }

    pub fn get_memory(&self, idx: usize) -> u8 {
        let mem_slice: Vec<u8> = self.memory[..].iter().map(|val| val.clone()).collect();
        mem_slice.get(idx).unwrap().clone()
    }
}

impl Cpu {
    /// Every cycle, the method emulateCycle is called which emulates one cycle of the Chip 8 CPU.
    /// During this cycle, the emulator will Fetch, Decode and Execute one opcode.
    pub fn emulate_cycle(&mut self) -> Result<(), String> {
        // Fetch Opcode
        let byte1 = self.memory[self.pc as usize];
        let byte2 = self.memory[(self.pc + 1) as usize];
        let opcode: Opcode = (byte1 as Opcode) << 8 | byte2 as Opcode;

        // Wrap if loaded Rom gets to empty memory
        if opcode == 0x0 {
            self.pc = USERSPACE_START;
            return Ok(());
        }

        // Increment Program Counter Register and wrap if it reaches the end of memory
        self.pc += 2;
        if self.pc == USERSPACE_END + 1 {
            self.pc = USERSPACE_START;
        }
        let instruction = instructions::decode_opcode(opcode)?;
        self.execute(instruction);

        // Update timers

        Ok(())
    }

    fn execute(&mut self, instruction: instructions::Instruction) {
        let pc_change: ProgramCounterChange = match instruction {
            Instruction::Call(_nnn) => ProgramCounterChange::Next,
            Instruction::Clear => {
                // Clears the screen.
                for pixel in self.pixels.iter_mut() {
                    *pixel = false;
                }
                ProgramCounterChange::Next
            }
            Instruction::SubReturn => {
                // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
                ProgramCounterChange::Next
            }
            Instruction::Jump(nnn) => ProgramCounterChange::Jump(nnn),
            Instruction::CallSubroutine(nnn) => {
                // *(0xNNN)() 	Calls subroutine at NNN.
                self.stack[self.sp as usize] = self.pc + OPCODE_SIZE;
                self.sp += 1;
                ProgramCounterChange::Jump(nnn)
            }
            Instruction::SkipEq(x, nn) => ProgramCounterChange::skip_if(self.v[x] == nn),
            Instruction::SkipNeq(x, nn) => ProgramCounterChange::skip_if(self.v[x] != nn),
            Instruction::SkipRegEq(x, y) => ProgramCounterChange::skip_if(self.v[x] == self.v[y]),
            Instruction::Set(x, nn) => {
                self.v[x] = nn;
                ProgramCounterChange::Next
            }
            Instruction::AddNoCarry(x, nn) => {
                let vx_val = self.v[x] as u16;
                let res = vx_val + nn as u16;
                self.v[x] = res as u8;
                ProgramCounterChange::Next
            }
            Instruction::Assign(x, y) => {
                self.v[x] = self.v[y];
                ProgramCounterChange::Next
            }
            Instruction::AssignOr(x, y) => {
                self.v[x] = self.v[x] | self.v[y];
                ProgramCounterChange::Next
            }
            Instruction::AssignAnd(x, y) => {
                self.v[x] = self.v[x] & self.v[y];
                ProgramCounterChange::Next
            }
            Instruction::AssignXor(x, y) => {
                self.v[x] = self.v[x] ^ self.v[y];
                ProgramCounterChange::Next
            }
            Instruction::AddCarry(x, y) => {
                let vx_val = self.v[x] as u16;
                let vy_val = self.v[y] as u16;
                let res = vx_val + vy_val;
                self.v[x] = res as u8;
                self.v[VF] = if res > 0xFF { 1 } else { 0 };
                ProgramCounterChange::Next
            }
            Instruction::SubLeft(x, y) => {
                self.v[VF] = if self.v[y] > self.v[x] { 1 } else { 0 };
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                ProgramCounterChange::Next
            }
            Instruction::LeastSig(x) => {
                self.v[VF] = self.v[x] & 0x0F;
                self.v[x] >>= 1;
                ProgramCounterChange::Next
            }
            Instruction::SubRight(x, y) => {
                self.v[VF] = if self.v[x] > self.v[y] { 0 } else { 1 };
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                ProgramCounterChange::Next
            }
            Instruction::MostSig(x) => {
                self.v[VF] = (self.v[x] & 0xF0) >> 4;
                self.v[x] <<= 1;
                ProgramCounterChange::Next
            }
            Instruction::CondNeq(x, y) => ProgramCounterChange::skip_if(self.v[x] != self.v[y]),
            Instruction::SetI(nnn) => {
                self.i = nnn;
                ProgramCounterChange::Next
            }
            Instruction::JumpV0NNN(nnn) => {
                self.pc = self.v[0] as u16 + nnn;
                ProgramCounterChange::Next
            }
            Instruction::RandX(x, nn) => {
                let mut rng = rand::thread_rng();
                let random_u8: u8 = rng.gen();
                self.v[x] = random_u8 & nn;
                ProgramCounterChange::Next
            }
            Instruction::DrawSprite(x, y, n) => {
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                // The interpreter reads n bytes from memory, starting at the address stored in I. 
                // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). 
                // Sprites are XORed onto the existing screen. 
                // If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. 
                // If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. 
                // See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
                for i in 0..n {
                    let pixel = self.memory[self.i + i];
                    self.pixels[(self.v[x] * SCREEN_WIDTH + self.v[y] + i)] = pixel;
                }
                ProgramCounterChange::Next
            }
            Instruction::KeyPressed(x) => {
                println!("KeyPressed, x: {:X}", x);
                ProgramCounterChange::Next
            }
            Instruction::KeyUnpressed(x) => {
                println!("KeyUnpressed, x: {:X}", x);
                ProgramCounterChange::Next
            }
            Instruction::SetXDelayTimer(x) => {
                println!("SetXDelayTimer, x: {:X}", x);
                ProgramCounterChange::Next
            }
            Instruction::AwaitKeyPress(x) => {
                println!("AwaitKeyPress, x: {:X}", x);
                ProgramCounterChange::Next
            }
            Instruction::SetDelayTimer(x) => {
                println!("SetDelayTimer, x: {:X}", x);
                ProgramCounterChange::Next
            }
            Instruction::SetSoundTimer(x) => {
                println!("SetSoundTimer, x: {:X}", x);
                ProgramCounterChange::Next
            }
            Instruction::AddVxToI(x) => {
                println!("AddVxToI, x: {:X}", x);
                ProgramCounterChange::Next
            }
            Instruction::SetIWithChar(x) => {
                println!("SetIWithChar, x: {:X}", x);
                ProgramCounterChange::Next
            }
            Instruction::SetBCD(x) => {
                println!("SetBCD, x: {:X}", x);
                ProgramCounterChange::Next
            }
            Instruction::RegDump(x) => {
                println!("RegDump, x: {:X}", x);
                ProgramCounterChange::Next
            }
            Instruction::RegLoad(x) => {
                println!("RegLoad, {:X}", x);
                ProgramCounterChange::Next
            }
        };

        match pc_change {
            ProgramCounterChange::Next => self.pc += OPCODE_SIZE,
            ProgramCounterChange::Skip => self.pc += OPCODE_SIZE * 2,
            ProgramCounterChange::Jump(nnn) => self.pc = nnn,
        }
    }

    pub fn draw_graphics(&self) {
        println!("TODO: Draw graphics");
    }

    pub fn set_keys(&mut self) {
        println!("TODO: Set keys");
    }
}

#[test]
fn test_keys_initialize_to_false() {
    let keys = Keys::default();
    for key in keys.iter() {
        assert_eq!(*key, false);
    }
}

#[test]
fn test_initial_mem_location_is_0x200() {
    let pc_reg = init_pc_register();
    assert_eq!(pc_reg, 0x200);
}

#[test]
fn test_opens_rom_correctly() -> std::io::Result<()> {
    let mut interpreter = Cpu::new();
    interpreter.load_rom("roms/puzzle.ch8")?;

    let first_byte = interpreter.get_memory(USERSPACE_START as usize);
    assert_eq!(first_byte, 0x00);
    let second_byte = interpreter.get_memory((USERSPACE_START + 1) as usize);
    assert_eq!(second_byte, 0xE0);
    let third_byte = interpreter.get_memory((USERSPACE_START + 2) as usize);
    assert_eq!(third_byte, 0x6C);
    let fourth_byte = interpreter.get_memory((USERSPACE_START + 3) as usize);
    assert_eq!(fourth_byte, 0x00);
    let third_byte = interpreter.get_memory((USERSPACE_START + 4) as usize);
    assert_eq!(third_byte, 0x4C);

    Ok(())
}

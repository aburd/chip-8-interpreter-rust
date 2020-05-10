use std::path::Path;
mod instructions;

use instructions::Instruction;

/**
 * ======
 * MEMORY - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
 * ======
 */
type Memory = [u8; 4096];
pub type Opcode = u16;
const USERSPACE_START: u16 = 0x200;
const USERSPACE_END: u16 = 0xFFF;

/**
 * =========
 * REGISTERS - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.2
 * =========
 */

// V0 - VF
const VF: usize = 15;
// const DELAY_TIMER_REGISTER
// const SOUND_TIMER_REGISTER
type GeneralRegisters = [u8; 16];
type PCRegister = u16; // Program Counter Register
type SPRegister = u8; // Stack Pointer Register
type I = u16;
type Stack = [u16; 16];

fn init_pc_register() -> PCRegister {
    USERSPACE_START.clone()
}

/**
 * ========
 * KEYBOARD - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.3
 * ========
 *
 * Layout:
 * 1 2 3 C
 * 4 5 6 D
 * 7 8 9 E
 * A 0 B F
 */

type Keys = [bool; 4 * 4];

/**
* =======
* DISPLAY - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.4
* =======
*
* The original implementation of the Chip-8 language used a 64x32-pixel monochrome display with this format:

   (0,0)	(63,0)
   (0,31)	(63,31)

* The graphics of the Chip 8 are black and white and the screen has a total of 2048 pixels (64 x 32).
* This can easily be implemented using an array that hold the pixel state (1 or 0):
*/
type Gfx = [bool; 64 * 32];

/**
* ==============
* TIMERS & SOUND - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.5
* ==============
*
* The original implementation of the Chip-8 language used a 64x32-pixel monochrome display with this format:

   (0,0)	(63,0)
   (0,31)	(63,31)
*/

/**
 * =======
 * FONTSET
 * =======
 */
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
    memory: Memory,
    v: GeneralRegisters,
    pc: PCRegister,
    stack: Stack,
    sp: SPRegister,
    keys: Keys,
    pub draw_flag: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let memory: Memory = [0; 4096];
        Cpu {
            memory,
            v: GeneralRegisters::default(),
            pc: init_pc_register(),
            stack: Stack::default(),
            sp: SPRegister::default(),
            keys: Keys::default(),
            draw_flag: false,
        }
    }

    pub fn initialize(&mut self) {
        // Reset all pertinent memory
        self.memory = [0; 4096];
        self.v = GeneralRegisters::default();
        self.pc = init_pc_register();
        self.stack = Stack::default();
        self.sp = SPRegister::default();
        self.keys = Keys::default();
        self.draw_flag = false;

        // Load fontset
        for i in 0..80 {
            self.memory[i] = CHIP8_FONTSET[i];
        }

        // Reset timers
    }

    pub fn load_rom(&mut self, path: &Path) -> std::io::Result<()> {
        let file = std::fs::read(path)?;
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

        // Decode Opcode
        let instruction = instructions::decode_opcode(opcode)?;

        // Execute Opcode
        self.execute_instruction(instruction);

        // Update timers

        Ok(())
    }

    fn execute_instruction(&mut self, instruction: instructions::Instruction) {
        match instruction {
            Instruction::Call(nnn) => println!("Call, nnn: {:X}", nnn),
            Instruction::Clear => println!("Clear screen"),
            Instruction::SubReturn => println!("SubReturn"),
            Instruction::Jump(nnn) => println!("Jump, nnn: {:X}", nnn),
            Instruction::CallSubroutine(nnn) => println!("CallSubroutine, nnn: {:X}", nnn),
            Instruction::SkipEq(x, nn) => println!("SkipEq, x: {:X}, nn: {:X}", x, nn),
            Instruction::SkipNeq(x, nn) => println!("SkipNeq, x: {:X}, nn: {:X}", x, nn),
            Instruction::SkipRegEq(x, y) => println!("SkipRegEq, x: {:X}, y: {:X}", x, y),
            Instruction::Set(x, nn) => println!("Set, x: {:X}, nn: {:X}", x, nn),
            Instruction::AddNoCarry(x, nn) => println!("AddNoCarry, x: {:X}, nn: {:X}", x, nn),
            Instruction::Assign(x, y) => println!("Assign, x: {:X}, y: {:X}", x, y),
            Instruction::AssignOr(x, y) => println!("AssignOr, x: {:X}, y: {:X}", x, y),
            Instruction::AssignAnd(x, y) => println!("AssignAnd, x: {:X}, y: {:X}", x, y),
            _ => unimplemented!(),
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
fn test_registers_initialize_to_zero() {
    let regs = GeneralRegisters::default();
    for reg in regs.iter() {
        assert_eq!(*reg, 0);
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
    let path = Path::new("roms/puzzle.ch8");
    interpreter.load_rom(path)?;

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

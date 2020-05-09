use std::path::Path;

/**
 * ======
 * MEMORY - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
 * ======
 */
type Memory = [u8; 4096];
const USERSPACE_START: u16 = 0x200;

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
// Program Counter Register
type PCRegister = u16;
// Stack Pointer Register
type SPRegister = u8;
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
* ============
* INSTRUCTIONS - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
* ============
*
* Standard Instructions
* =====================
   0NNN 	Call 		Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
   00E0 	Display 	disp_clear() 	Clears the screen.
   00EE 	Flow 	    return; 	Returns from a subroutine.
   1NNN 	Flow 	    goto NNN; 	Jumps to address NNN.
   2NNN 	Flow 	    *(0xNNN)() 	Calls subroutine at NNN.
   3XNN 	Cond 	    if(Vx==NN) 	Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block)
   4XNN 	Cond 	    if(Vx!=NN) 	Skips the next instruction if VX doesn't equal NN. (Usually the next instruction is a jump to skip a code block)
   5XY0 	Cond 	    if(Vx==Vy) 	Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block)
   6XNN 	Const 	    Vx = NN 	Sets VX to NN.
   7XNN 	Const 	    Vx += NN 	Adds NN to VX. (Carry flag is not changed)
   8XY0 	Assign 	    Vx=Vy 	Sets VX to the value of VY.
   8XY1 	BitOp 	    Vx=Vx|Vy 	Sets VX to VX or VY. (Bitwise OR operation)
   8XY2 	BitOp 	    Vx=Vx&Vy 	Sets VX to VX and VY. (Bitwise AND operation)
   8XY3[a]  BitOp 	    Vx=Vx^Vy 	Sets VX to VX xor VY.
   8XY4 	Math 	    Vx += Vy 	Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
   8XY5 	Math 	    Vx -= Vy 	VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
   8XY6[a]  BitOp 	    Vx>>=1 	Stores the least significant bit of VX in VF and then shifts VX to the right by 1.[b]
   8XY7[a]	Math 	    Vx=Vy-Vx 	Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
   8XYE[a]	BitOp 	    Vx<<=1 	Stores the most significant bit of VX in VF and then shifts VX to the left by 1.[b]
   9XY0 	Cond 	    if(Vx!=Vy) 	Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block)
   ANNN 	MEM 	    I = NNN 	Sets I to the address NNN.
   BNNN 	Flow 	    PC=V0+NNN 	Jumps to the address NNN plus V0.
   CXNN 	Rand 	    Vx=rand()&NN 	Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
   DXYN 	Disp 	    draw(Vx,Vy,N) 	Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen
   EX9E 	KeyOp 	    if(key()==Vx) 	Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block)
   EXA1 	KeyOp 	    if(key()!=Vx) 	Skips the next instruction if the key stored in VX isn't pressed. (Usually the next instruction is a jump to skip a code block)
   FX07 	Timer 	    Vx = get_delay() 	Sets VX to the value of the delay timer.
   FX0A 	KeyOp 	    Vx = get_key() 	A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)
   FX15 	Timer 	    delay_timer(Vx) 	Sets the delay timer to VX.
   FX18 	Sound 	    sound_timer(Vx) 	Sets the sound timer to VX.
   FX1E 	MEM 	    I +=Vx 	Adds VX to I. VF is set to 1 when there is a range overflow (I+VX>0xFFF), and to 0 when there isn't.[c]
   FX29 	MEM 	    I=sprite_addr[Vx] 	Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
   FX33 	BCD 	    set_BCD(Vx);
   *(I+0)=BCD(3);
   *(I+1)=BCD(2);
   *(I+2)=BCD(1);
       Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
   FX55 	MEM 	    reg_dump(Vx,&I) 	Stores V0 to VX (including VX) in memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
   FX65 	MEM 	    reg_load(Vx,&I) 	Fills V0 to VX (including VX) with values from memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
*
* Super Chip-48 Instructions
* ==========================
* - 00Cn - SCD nibble
* - 00FB - SCR
* - 00FC - SCL
* - 00FD - EXIT
* - 00FE - LOW
* - 00FF - HIGH
* - Dxy0 - DRW Vx, Vy, 0
* - Fx30 - LD HF, Vx
* - Fx75 - LD R, Vx
* - Fx85 - LD Vx, R
*/

type Opcode = u16;

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

pub struct Chip8Interpreter {
    memory: Memory,
    v: GeneralRegisters,
    pc: PCRegister,
    stack: Stack,
    sp: SPRegister,
    keys: Keys,
    pub draw_flag: bool,
}

impl Chip8Interpreter {
    pub fn new() -> Self {
        let memory: Memory = [0; 4096];
        Chip8Interpreter {
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
        let mem_slice: Vec<u8> = self.memory[..].iter()
            .map(|val| val.clone())
            .collect();
        mem_slice.get(idx).unwrap().clone()
    }
}

impl Chip8Interpreter {
    /// Every cycle, the method emulateCycle is called which emulates one cycle of the Chip 8 CPU.
    /// During this cycle, the emulator will Fetch, Decode and Execute one opcode.
    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode
        let byte1 = self.memory[self.pc as usize];
        self.pc += 1;
        let byte2 = self.memory[self.pc as usize];
        self.pc += 1;
        let opcode: Opcode = (byte1 as Opcode) << 8 | byte2 as Opcode;

        // Decode Opcode

        // Execute Opcode

        // Update timers

        unimplemented!();
    }

    pub fn draw_graphics(&self) {
        unimplemented!();
    }

    pub fn set_keys(&mut self) {
        unimplemented!();
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
    let mut interpreter = Chip8Interpreter::new();
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
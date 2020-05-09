/**
 * ======
 * MEMORY - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
 * ======
 */
type Memory = [u8; 4096];

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
 * - 00E0 - CLS
 * - 00EE - RET
 * - 0nnn - SYS addr
 * - 1nnn - JP addr
 * - 2nnn - CALL addr
 * - 3xkk - SE Vx, byte
 * - 4xkk - SNE Vx, byte
 * - 5xy0 - SE Vx, Vy
 * - 6xkk - LD Vx, byte
 * - 7xkk - ADD Vx, byte
 * - 8xy0 - LD Vx, Vy
 * - 8xy1 - OR Vx, Vy
 * - 8xy2 - AND Vx, Vy
 * - 8xy3 - XOR Vx, Vy
 * - 8xy4 - ADD Vx, Vy
 * - 8xy5 - SUB Vx, Vy
 * - 8xy6 - SHR Vx {, Vy}
 * - 8xy7 - SUBN Vx, Vy
 * - 8xyE - SHL Vx {, Vy}
 * - 9xy0 - SNE Vx, Vy
 * - Annn - LD I, addr
 * - Bnnn - JP V0, addr
 * - Cxkk - RND Vx, byte
 * - Dxyn - DRW Vx, Vy, nibble
 * - Ex9E - SKP Vx
 * - ExA1 - SKNP Vx
 * - Fx07 - LD Vx, DT
 * - Fx0A - LD Vx, K
 * - Fx15 - LD DT, Vx
 * - Fx18 - LD ST, Vx
 * - Fx1E - ADD I, Vx
 * - Fx29 - LD F, Vx
 * - Fx33 - LD B, Vx
 * - Fx55 - LD [I], Vx
 * - Fx65 - LD Vx, [I]
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
            pc: PCRegister::default(),
            stack: Stack::default(),
            sp: SPRegister::default(),
            keys: Keys::default(),
            draw_flag: false,
        }
    }
}

impl Chip8Interpreter {
    pub fn emulate_cycle(&mut self) {
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
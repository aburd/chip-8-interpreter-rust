/**
 * REGISTERS
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
 * KEYBOARD - The keyboard layout is as below
 * 1 2 3 C
 * 4 5 6 D
 * 7 8 9 E
 * A 0 B F
 */ 

/*

*/

fn main() {
    let mut registers: [u8; 16];
    
}

#[test]
fn test_registers_initialize_to_zero() {
    let regs= GeneralRegisters::default();
    for reg in regs.iter() {
        assert_eq!(*reg, 0);
    }
}
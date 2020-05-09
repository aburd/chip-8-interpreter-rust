use super::Opcode;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    // Standard Instructions
    Call(u16), //  0NNN 	Call 		Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
    Clear,   //  00E0 	Display 	disp_clear() 	Clears the screen.
    SubReturn, //  00EE 	Flow 	    return; 	Returns from a subroutine.
    Jump(u16), //  1NNN 	Flow 	    goto NNN; 	Jumps to address NNN.
    CallSubroutine(u16), //  2NNN 	Flow 	    *(0xNNN)() 	Calls subroutine at NNN.
    SkipEq(u8, u8), //  3XNN 	Cond 	    if(Vx==NN) 	Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block)
    SkipNeq(u8, u8), //  4XNN 	Cond 	    if(Vx!=NN) 	Skips the next instruction if VX doesn't equal NN. (Usually the next instruction is a jump to skip a code block)
    SkipRegEq(u8, u8), //  5XY0 	Cond 	    if(Vx==Vy) 	Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block)
    Set(u8, u8),       //  6XNN 	Const 	    Vx = NN 	Sets VX to NN.
    AddNoCarry(u8, u8), //  7XNN 	Const 	    Vx += NN 	Adds NN to VX. (Carry flag is not changed)
    Assign(u8, u8),    //  8XY0 	Assign 	    Vx=Vy 	Sets VX to the value of VY.
                       //  8XY1 	BitOp 	    Vx=Vx|Vy 	Sets VX to VX or VY. (Bitwise OR operation)
                       //  8XY2 	BitOp 	    Vx=Vx&Vy 	Sets VX to VX and VY. (Bitwise AND operation)
                       //  8XY3[a]  BitOp 	    Vx=Vx^Vy 	Sets VX to VX xor VY.
                       //  8XY4 	Math 	    Vx += Vy 	Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
                       //  8XY5 	Math 	    Vx -= Vy 	VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                       //  8XY6[a]  BitOp 	    Vx>>=1 	Stores the least significant bit of VX in VF and then shifts VX to the right by 1.[b]
                       //  8XY7[a]	Math 	    Vx=Vy-Vx 	Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                       //  8XYE[a]	BitOp 	    Vx<<=1 	Stores the most significant bit of VX in VF and then shifts VX to the left by 1.[b]
                       //  9XY0 	Cond 	    if(Vx!=Vy) 	Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block)
                       //  ANNN 	MEM 	    I = NNN 	Sets I to the address NNN.
                       //  BNNN 	Flow 	    PC=V0+NNN 	Jumps to the address NNN plus V0.
                       //  CXNN 	Rand 	    Vx=rand()&NN 	Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
                       //  DXYN 	Disp 	    draw(Vx,Vy,N) 	Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen
                       //  EX9E 	KeyOp 	    if(key()==Vx) 	Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block)
                       //  EXA1 	KeyOp 	    if(key()!=Vx) 	Skips the next instruction if the key stored in VX isn't pressed. (Usually the next instruction is a jump to skip a code block)
                       //  FX07 	Timer 	    Vx = get_delay() 	Sets VX to the value of the delay timer.
                       //  FX0A 	KeyOp 	    Vx = get_key() 	A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)
                       //  FX15 	Timer 	    delay_timer(Vx) 	Sets the delay timer to VX.
                       //  FX18 	Sound 	    sound_timer(Vx) 	Sets the sound timer to VX.
                       //  FX1E 	MEM 	    I +=Vx 	Adds VX to I. VF is set to 1 when there is a range overflow (I+VX>0xFFF), and to 0 when there isn't.[c]
                       //  FX29 	MEM 	    I=sprite_addr[Vx] 	Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
                       //  FX33 	BCD 	    set_BCD(Vx);
                       //  *(I+0)=BCD(3);
                       //  *(I+1)=BCD(2);
                       //  *(I+2)=BCD(1);
                       //      Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
                       //  FX55 	MEM 	    reg_dump(Vx,&I) 	Stores V0 to VX (including VX) in memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
                       //  FX65 	MEM 	    reg_load(Vx,&I) 	Fills V0 to VX (including VX) with values from memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
                       // * =========================
                       // * Super Chip-48 Instructions
                       // * ==========================
                       // * - 00Cn - SCD nibble
                       // * - 00FB - SCR
                       // * - 00FC - SCL
                       // * - 00FD - EXIT
                       // * - 00FE - LOW
                       // * - 00FF - HIGH
                       // * - Dxy0 - DRW Vx, Vy, 0
                       // * - Fx30 - LD HF, Vx
                       // * - Fx75 - LD R, Vx
                       // * - Fx85 - LD Vx, R
}

fn get_nnn(opcode: Opcode) -> u16 {
    opcode & 0x0FFF
}
fn get_nn(opcode: Opcode) -> u8 {
    (opcode & 0x00FF) as u8
}
fn get_x(opcode: Opcode) -> u8 {
    ((opcode & 0x0F00) >> 8) as u8
}
fn get_y(opcode: Opcode) -> u8 {
    ((opcode & 0x00F0) >> 4) as u8
}

pub fn decode_opcode(opcode: Opcode) -> Result<Instruction, String> {
    match opcode {
        0x00E0 => Ok(Instruction::Clear),
        0x00EE => Ok(Instruction::SubReturn),
        o if o > 0x0000 && o <= 0x0FFF => Ok(Instruction::Call(get_nnn(o))),
        o if o >= 0x1000 && o < 0x2000 => Ok(Instruction::Jump(get_nnn(o))),
        o if o >= 0x2000 && o < 0x3000 => Ok(Instruction::CallSubroutine(get_nnn(o))),
        o if o >= 0x3000 && o < 0x4000 => {
            Ok(Instruction::SkipEq(
                get_x(o),
                get_nn(o),
            ))
        },
        o if o >= 0x4000 && o < 0x5000 => {
            Ok(Instruction::SkipNeq(
                get_x(o),
                get_nn(o),
            ))
        },
        o if o >= 0x5000 && o < 0x6000 => {
            Ok(Instruction::SkipRegEq(
                get_x(o),
                get_y(o),
            ))
        },
        o if o >= 0x6000 && o < 0x7000 => {
            Ok(Instruction::Set(
                get_x(o),
                get_nn(o),
            ))
        },
        _ => Err("Opcode not implemented!".to_string()),
    }
}

#[test]
fn test_parse_call() {
    let opcode: Opcode = 0x0111;
    let instruction = decode_opcode(opcode);

    assert_eq!(instruction, Ok(Instruction::Call(0x111)));
}
#[test]
fn test_parse_clear() {
    let opcode: Opcode = 0x00E0;
    let instruction = decode_opcode(opcode);

    assert_eq!(instruction, Ok(Instruction::Clear));
}
#[test]
fn test_parse_subreturn() {
    let opcode: Opcode = 0x00EE;
    let instruction = decode_opcode(opcode);

    assert_eq!(instruction, Ok(Instruction::SubReturn));
}
#[test]
fn test_parse_jump() {
    let opcode: Opcode = 0x1111;
    let instruction = decode_opcode(opcode);

    assert_eq!(instruction, Ok(Instruction::Jump(0x111)));
}
#[test]
fn test_parse_subroutine() {
    let opcode: Opcode = 0x2111;
    let instruction = decode_opcode(opcode);

    assert_eq!(instruction, Ok(Instruction::CallSubroutine(0x111)));
}
#[test]
fn test_parse_skip() {
    let opcode_skip_eq: Opcode = 0x3122;
    let instruction_skip_eq = decode_opcode(opcode_skip_eq);
    assert_eq!(instruction_skip_eq, Ok(Instruction::SkipEq(0x1, 0x22)));

    let opcode_skip_neq: Opcode = 0x4122;
    let instruction_skip_neq = decode_opcode(opcode_skip_neq);
    assert_eq!(instruction_skip_neq, Ok(Instruction::SkipNeq(0x1, 0x22)));
    
    let opcode_skip_eq: Opcode = 0x5120;
    let instruction_skip_eq = decode_opcode(opcode_skip_eq);
    assert_eq!(instruction_skip_eq, Ok(Instruction::SkipRegEq(0x1, 0x2)));
}
#[test]
fn test_parse_set() {
    let opcode: Opcode = 0x6A12;
    let instruction = decode_opcode(opcode);

    assert_eq!(instruction, Ok(Instruction::Set(0xA, 0x12)));
}
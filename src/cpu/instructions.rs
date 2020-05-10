use super::Opcode;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    // Standard Instructions
    Call(u16), //  0NNN 	Call 		Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
    Clear,     //  00E0 	Display 	disp_clear() 	Clears the screen.
    SubReturn, //  00EE 	Flow 	    return; 	Returns from a subroutine.
    Jump(u16), //  1NNN 	Flow 	    goto NNN; 	Jumps to address NNN.
    CallSubroutine(u16), //  2NNN 	Flow 	    *(0xNNN)() 	Calls subroutine at NNN.
    SkipEq(u8, u8), //  3XNN 	Cond 	    if(Vx==NN) 	Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block)
    SkipNeq(u8, u8), //  4XNN 	Cond 	    if(Vx!=NN) 	Skips the next instruction if VX doesn't equal NN. (Usually the next instruction is a jump to skip a code block)
    SkipRegEq(u8, u8), //  5XY0 	Cond 	    if(Vx==Vy) 	Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block)
    Set(u8, u8),       //  6XNN 	Const 	    Vx = NN 	Sets VX to NN.
    AddNoCarry(u8, u8), //  7XNN 	Const 	    Vx += NN 	Adds NN to VX. (Carry flag is not changed)
    Assign(u8, u8),    //  8XY0 	Assign 	    Vx=Vy 	Sets VX to the value of VY.
    AssignOr(u8, u8),  //  8XY1 	BitOp 	    Vx=Vx|Vy 	Sets VX to VX or VY. (Bitwise OR operation)
    AssignAnd(u8, u8), //  8XY2 	BitOp 	    Vx=Vx&Vy 	Sets VX to VX and VY. (Bitwise AND operation)
    AssignXor(u8, u8), //  8XY3[a]  BitOp 	    Vx=Vx^Vy 	Sets VX to VX xor VY.
    AddCarry(u8, u8), //  8XY4 	Math 	    Vx += Vy 	Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
    SubLeft(u8, u8), //  8XY5 	Math 	    Vx -= Vy 	VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
    LeastSig(u8), //  8XY6[a]  BitOp 	    Vx>>=1 	Stores the least significant bit of VX in VF and then shifts VX to the right by 1.[b]
    SubRight(u8, u8), //  8XY7[a]	Math 	    Vx=Vy-Vx 	Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
    MostSig(u8), //  8XYE[a]	BitOp 	    Vx<<=1 	Stores the most significant bit of VX in VF and then shifts VX to the left by 1.[b]
    CondNeq(u8, u8), //  9XY0 	Cond 	    if(Vx!=Vy) 	Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block)
    SetI(u16),       //  ANNN 	MEM 	    I = NNN 	Sets I to the address NNN.
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

pub fn decode_opcode(opcode: Opcode) -> Result<Instruction, String> {
    let nibbles = (
        ((opcode & 0xF000) >> 12) as u8,
        ((opcode & 0x0F00) >> 8) as u8,
        ((opcode & 0x00F0) >> 4) as u8,
        (opcode & 0x000F) as u8,
    );
    let nnn = (opcode & 0x0FFF) as u16;
    let nn = (opcode & 0x00FF) as u8;
    let x = ((opcode & 0x0F00) >> 8) as u8;
    let y = ((opcode & 0x00F0) >> 4) as u8;

    match (opcode & 0xF000) >> 12 {
        0x0 => match nibbles {
            (_, _, 0xE, 0x0) => Ok(Instruction::Clear),
            (_, _, 0xE, 0xE) => Ok(Instruction::SubReturn),
            _ => Ok(Instruction::Call(nnn)),
        },
        0x1 => Ok(Instruction::Jump(nnn)),
        0x2 => Ok(Instruction::CallSubroutine(nnn)),
        0x3 => Ok(Instruction::SkipEq(x, nn)),
        0x4 => Ok(Instruction::SkipNeq(x, nn)),
        0x5 => Ok(Instruction::SkipRegEq(x, y)),
        0x6 => Ok(Instruction::Set(x, nn)),
        0x7 => Ok(Instruction::AddNoCarry(x, nn)),
        0x8 => match nibbles {
            (_, _, _, 0x0) => Ok(Instruction::Assign(x, y)),
            (_, _, _, 0x1) => Ok(Instruction::AssignOr(x, y)),
            (_, _, _, 0x2) => Ok(Instruction::AssignAnd(x, y)),
            (_, _, _, 0x3) => Ok(Instruction::AssignXor(x, y)),
            (_, _, _, 0x4) => Ok(Instruction::AddCarry(x, y)),
            (_, _, _, 0x5) => Ok(Instruction::SubLeft(x, y)),
            (_, _, _, 0x6) => Ok(Instruction::LeastSig(x)),
            (_, _, _, 0x7) => Ok(Instruction::SubRight(x, y)),
            (_, _, _, 0xE) => Ok(Instruction::MostSig(x)),
            _ => Err("Opcode not implemented!".to_string()),
        },
        0x9 => Ok(Instruction::CondNeq(x, y)),
        0xA => Ok(Instruction::SetI(nnn)),
        _ => Err("Opcode not implemented!".to_string()),
    }
}

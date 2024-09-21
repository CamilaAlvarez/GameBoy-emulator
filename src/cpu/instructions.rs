enum Instruction {
    // Add ArithmeticTarget to the value stored in A, then
    // store the result in A
    ADD(ArithmeticTarget),
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L
}

enum IncDecTarget {
    BC, DE
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            _ => None

        }
    }
}

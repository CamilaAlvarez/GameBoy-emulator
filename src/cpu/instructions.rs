enum Instruction {
    // Add ArithmeticTarget to the value stored in A, then
    // store the result in A
    ADD(ArithmeticTarget),
    INC(IncDecTarget),
    RLC(PrefixTarget),
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L
}

enum IncDecTarget {
    BC, DE
}

enum PrefixTarget {
    B
}

enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}

enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

enum StackTarget {
    BC
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            _ => None
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::INC(IncDecTarget::BC)),
            _ => None
        }
    }
}

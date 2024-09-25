// The gameboy requires us to handle registers manually
// We define a series of structs that make handling the registers easier
pub mod registers;
pub mod instructions;
pub mod memory;

pub struct CPU {
    registers: Registers,
    // current instruction in execution
    pc: u16,
    // stack pointer, pointer grows downwards (away from the end of memory)
    sp: u16,
    bus: MemoryBus,
}
impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        // move to the next instruction and return the new pc
                        // wrapping add wraps the result => 255 + 1 = 0
                        self.pc.wrapping_add(1)
                    }
                    _ => { self.pc }
                }
            }
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                    _ => { panic!("TODO: support more targets") }
                };
                self.push(value);
                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::BC => self.registers.set_bc(result),
                    _ => { panic!("TODO: support more targets") }
                };
                self.pc.wrapping_add(1);
            }
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            }
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                       let source_value = match source {
                           LoadByteSource::A => self.registers.a,
                           LoadByteSource::D8 => self.read_next_byte(),
                           // HL is the source register
                           LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                           _ => { panic!("TODO: Implement other sources") }
                       };
                       match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
                            _ => { panic!("TODO: Implement other targets") }
                       };
                       match source {
                           LoadByteSource::D8 => self.pc.wrapping_add(2),
                           _                  => self.pc.wrapping_add(1),
                       }
                    }
                    _ => { panic!("TODO: implement other load types") }
                }
            }
            _ => { self.pc }
        }
    }

    fn jump(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            // even if we don't jump we still need to move the pc (the instruction
            // is 3 bytes wide)
            self.pc.wrapping_add(3)
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        // overflowing_add is an u8 method (+ panics when the result overflows)
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        // We use the overflow to set the carry flag
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            // TODO: maybe we should do a wrapping_add? NO! Because it needs to be the next
            // instruction
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc 0 if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execite(instruction);
        } else {
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unknown instruction found for: {}", description);
        };
        
        self.pc = next_pc;
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        // Most significant byte 
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value && 0x00FF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }
}

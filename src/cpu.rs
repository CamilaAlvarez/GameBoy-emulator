// The gameboy requires us to handle registers manually
// We define a series of structs that make handling the registers easier
pub mod registers;
pub mod instructions;
pub mod memory;

pub struct CPU {
    registers: Registers,
    // current instruction in execution
    pc: u16,
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
                    }
                    _ => { }
                }
            }
            _ => { }
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

    fn setp(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let next_pc 0 if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.execite(instruction);
        } else {
            panic!("Unknown instruction found for: 0x{:x}", instruction_byte);
        };
        
        self.pc = next_pc;
    }
}

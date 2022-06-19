use super::{CPU, addressing_modes::AddressMode, Flag};

impl CPU {
    pub fn call_operation(&mut self, opcode: Opcode) -> u8 {
        match opcode {
            _ => todo!("Implement operation {:?}", opcode)
        }
        
        0
    }

    fn fetch(&mut self) -> u8 {
        if self.instructions[self.opcode as usize].address_mode != AddressMode::IMP {
            self.fetched = self.read(self.address_absolute);
            println!("fetched: {}", self.fetched);
        }

        self.fetched
    }

    /// Instruction: Bitwise Logic AND
    /// Function: A = A & M
    /// Flags: Negative, Zero
    fn and(&mut self) -> u8 {
        self.fetch();
        self.accumulator &= self.fetched;

        self.set_flag(Flag::Zero, self.accumulator == 0x00);
        self.set_flag(Flag::Negative, (self.accumulator & 0x80) != 0);

        1
    }

    
    /// Instruction: Branch if Carry Set
    /// Function: if Carry flag is set, set program counter to address
    fn bcs(&mut self) -> u8 {
        if self.get_flag(Flag::Carry) == true {
            self.remaining_cycles += 1;

            self.address_absolute = self.program_counter + self.address_relative;
            if (self.address_absolute & 0xFF00) != (self.program_counter & 0xFF00) {
                self.remaining_cycles += 1;
            }

            self.program_counter = self.address_absolute;
        }
        
        0
    }

    /// Instruction: Branch if Carry Clear
    /// Function: if Carry flag is not set, set program counter to address
    fn bcc(&mut self) -> u8 {
        if self.get_flag(Flag::Carry) == false {
            self.remaining_cycles += 1;
            self.address_absolute = self.program_counter + self.address_relative;

            if (self.address_absolute & 0xFF00) != (self.program_counter & 0xFF00) {
                self.remaining_cycles += 1;
            }

            self.program_counter = self.address_absolute;
        }
        
        0
    }

    /// Instruction: Branch if Equal
    /// Function: if Zero flag is set, set program counter to address
    fn beq(&mut self) -> u8 {
        if self.get_flag(Flag::Zero) == true {
            self.remaining_cycles += 1;
            self.address_absolute = self.program_counter + self.address_relative;

            if (self.address_absolute & 0xFF00) != (self.program_counter & 0xFF00) {
                self.remaining_cycles += 1;
            }

            self.program_counter = self.address_absolute;
        }
        
        0
    }

    /// Instruction: Branch if Negative
    /// Function: If Negative flag is set, set program counter to address
    fn bmi(&mut self) -> u8 {
        if self.get_flag(Flag::Negative) == true {
            self.remaining_cycles += 1;
            self.address_absolute = self.program_counter + self.address_relative;

            if (self.address_absolute & 0xFF00) != (self.program_counter & 0xFF00) {
                self.remaining_cycles += 1;
            }

            self.program_counter = self.address_absolute;
        }
        
        0
    }

    /// Instruction: Branch if Not Equal
    /// Function: If Zero flag is not set, set program counter to address
    fn bne(&mut self) -> u8 {
        if self.get_flag(Flag::Zero) == false {
            self.remaining_cycles += 1;
            self.address_absolute = self.program_counter + self.address_relative;

            if (self.address_absolute & 0xFF00) != (self.program_counter & 0xFF00) {
                self.remaining_cycles += 1;
            }

            self.program_counter = self.address_absolute;
        }
        
        0
    }

    /// Instruction: Branch if Positive
    /// Function: If Negative flag is not set, set program counter to address
    fn bpl(&mut self) -> u8 {
        if self.get_flag(Flag::Negative) == false {
            self.remaining_cycles += 1;
            self.address_absolute = self.program_counter + self.address_relative;

            if (self.address_absolute & 0xFF00) != (self.program_counter & 0xFF00) {
                self.remaining_cycles += 1;
            }

            self.program_counter = self.address_absolute;
        }
        
        0
    }

    /// Instruction: Branch if Overflow Clear
    /// Function: If Overflow flag is not set, set program counter to address
    fn bvc(&mut self) -> u8 {
        if self.get_flag(Flag::Overflow) == false {
            self.remaining_cycles += 1;
            self.address_absolute = self.program_counter + self.address_relative;

            if (self.address_absolute & 0xFF00) != (self.program_counter & 0xFF00) {
                self.remaining_cycles += 1;
            }

            self.program_counter = self.address_absolute;
        }
        
        0
    }

    /// Instruction: Branch if Overflow Set
    /// Function: If Overflow flag is set, set program counter to address
    fn bvs(&mut self) -> u8 {
        if self.get_flag(Flag::Overflow) == true {
            self.remaining_cycles += 1;
            self.address_absolute = self.program_counter + self.address_relative;

            if (self.address_absolute & 0xFF00) != (self.program_counter & 0xFF00) {
                self.remaining_cycles += 1;
            }

            self.program_counter = self.address_absolute;
        }
        
        0
    }

    /// Instruction: Clear Carry Flag
    /// Function: set Carry flag to 0
    fn clc(&mut self) -> u8 {
        self.set_flag(Flag::Carry, false);
        0
    }

    /// Instruction: Clear Decimal Flag
    /// Function: set Decimal flag to 0
    fn cld(&mut self) -> u8 {
        self.set_flag(Flag::Decimal, false);
        0
    }

    /// Instruction: Clear Interrupt Flag
    /// Function: set Interrupt flag to 0
    fn cli(&mut self) -> u8 {
        self.set_flag(Flag::DisableInterrupt, false);
        0
    }

    /// Instruction: Clear Overflow Flag
    /// Function: set Overflow flag to 0
    fn clv(&mut self) -> u8 {
        self.set_flag(Flag::Overflow, false);
        0
    }

    /// Instruction: Add with Carry In
    /// Function: Add to the accumulator the data + the Carry bit
    /// Flags: Carry, Overflow, Negative, Zero
    fn adc(&mut self) -> u8 {
        self.fetch();

        let temp = self.accumulator as u16 + self.fetched as u16 + self.get_flag(Flag::Carry) as u16;

        self.set_flag(Flag::Carry, temp > 255);
        self.set_flag(Flag::Zero, (temp & 0x00FF) == 0);
        self.set_flag(Flag::Overflow, ((!(self.accumulator as u16 ^ self.fetched as u16) & (self.accumulator as u16 ^ temp as u16)) & 0x0080) != 0);
        self.set_flag(Flag::Negative, (temp & 0x80) != 0);
        self.accumulator = temp as u8;

        1
    }

    /// Instruction: Substraction with Borrow In
    /// Function: Subtract the data + the opposite of the Carry bit from the accumulator
    fn sbc(&mut self) -> u8 {
        self.fetch();

        let value = self.fetched as u16 ^ 0x00FF;
        
        let temp = self.accumulator as u16 + value + self.get_flag(Flag::Carry) as u16;
        
        self.set_flag(Flag::Carry, (temp & 0xFF00) != 0);
        self.set_flag(Flag::Zero, (temp & 0x00FF) == 0);
        self.set_flag(Flag::Overflow, ((temp ^ self.accumulator as u16) & (temp ^ value) & 0x0080) != 0);
        self.set_flag(Flag::Negative, (temp & 0x0080) != 0);
        self.accumulator = temp as u8;

        1
    }

    /// Instruction: Push Accumulator to Stack
    /// Function: Copies the value from the accumulator to the stack
    fn pha(&mut self) -> u8 {
        self.write(0x0100 + self.stack_pointer as u16, self.accumulator);
        self.stack_pointer -= 1;
        
        0
    }

    /// Instruction: Push Status Register to Stack
    /// Function: Copies the value from the status register to the stack
    /// Note: Break flag is set to 1 before push
    fn php(&mut self) -> u8 {
        let break_or_unused = self.get_flag(Flag::Break) as u8 | self.get_flag(Flag::Unused) as u8;
        self.write(0x0100 + self.stack_pointer as u16, self.status | break_or_unused);
        self.set_flag(Flag::Break, false);
        self.set_flag(Flag::Unused, false);
        self.stack_pointer -= 1;
        
        0
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {

}
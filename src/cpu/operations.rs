use super::{CPU, addressing_modes::AddressMode, Flag};

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    
}


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

    /// Instruction: Arithmetic Shift Left
    /// Function: Shifts the accumulator left by one bit, set the Carry flag if overflow.
    /// Flags: Carry, Zero, Negative
    fn asl(&mut self) -> u8 {
        self.fetch();
        let temp = (self.fetched as u16) << 1;
        self.set_flag(Flag::Carry, (temp & 0xFF00) != 0);
        self.set_flag(Flag::Zero, (temp & 0x00FF) == 0);
        self.set_flag(Flag::Negative, (temp & 0x80) != 0);
        if self.instructions[self.opcode as usize].address_mode == AddressMode::IMP {
            self.accumulator = temp as u8;
        } else {
            self.write(self.address_absolute, temp as u8);
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

    /// Instruction: Break
    /// Function: Saves the current state of the program and set program counter to new address
    fn brk(&mut self) -> u8 {
        self.remaining_cycles += 1;

        self.set_flag(Flag::DisableInterrupt, true);
        self.write(0x0100 + self.stack_pointer as u16, (self.program_counter >> 8) as u8);
        self.stack_pointer -= 1;
        self.write(0x0100 + self.stack_pointer as u16, self.program_counter as u8);
        self.stack_pointer -= 1;

        self.set_flag(Flag::Break, true);
        self.write(0x0100 + self.stack_pointer as u16, self.status);
        self.stack_pointer -= 1;
        self.set_flag(Flag::Break, false);

        self.program_counter = self.read(0xFFFE) as u16 | (self.read(0xFFFF) as u16) << 8;
        
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

    /// Instruction: Compare Accumulator
    /// Function: Compare the contents of the accumulator with the contents of memory.
    /// Flags: Negative, Carry, Zero
    fn cmp(&mut self) -> u8 {
        self.fetch();

        let temp = self.accumulator as u16 - self.fetched as u16;
        self.set_flag(Flag::Carry, self.accumulator >= self.fetched);
        self.set_flag(Flag::Zero, (temp as u8) == 0);
        self.set_flag(Flag::Negative, (temp & 0x080) != 0);

        1
    }

    /// Instruction: Compare X Register
    /// Function: Compare the contents of the X register with the contents of memory.
    /// Flags: Negative, Carry, Zero
    fn cpx(&mut self) -> u8 {
        self.fetch();

        let temp = self.register_x as u16 - self.fetched as u16;
        self.set_flag(Flag::Carry, self.register_x >= self.fetched);
        self.set_flag(Flag::Zero, (temp as u8) == 0);
        self.set_flag(Flag::Negative, (temp & 0x080) != 0);

        0
    }

    /// Instruction: Compare Y Register
    /// Function: Compare the contents of the Y register with the contents of memory.
    /// Flags: Negative, Carry, Zero
    fn cpy(&mut self) -> u8 {
        self.fetch();

        let temp = self.register_y as u16 - self.fetched as u16;
        self.set_flag(Flag::Carry, self.register_y >= self.fetched);
        self.set_flag(Flag::Zero, (temp as u8) == 0);
        self.set_flag(Flag::Negative, (temp & 0x080) != 0);

        0
    }

    /// Instruction: Decrement Memory
    /// Function: Decrement the contents of memory by one.
    /// Flags: Negative, Zero
    fn dec(&mut self) -> u8 {
        self.fetch();

        let temp = self.fetched - 1;
        self.write(self.address_absolute, temp as u8);
        self.set_flag(Flag::Zero, (temp as u8) == 0);
        self.set_flag(Flag::Negative, (temp & 0x0080) != 0);

        0
    }

    /// Instruction: Decrement X Register
    /// Function: Decrement the contents of the X register by one.
    /// Flags: Negative, Zero
    fn dex(&mut self) -> u8 {
        self.register_x -= 1;
        self.set_flag(Flag::Zero, self.register_x == 0);
        self.set_flag(Flag::Negative, (self.register_x & 0x80) != 0);

        0
    }

    /// Instruction: Decrement Y Register
    /// Function: Decrement the contents of the Y register by one.
    /// Flags: Negative, Zero
    fn dey(&mut self) -> u8 {
        self.register_y -= 1;
        self.set_flag(Flag::Zero, self.register_y == 0);
        self.set_flag(Flag::Negative, (self.register_y & 0x80) != 0);

        0
    }

    /// Instruction: Bitwise Logic XOR
    /// Function: Perform a bitwise logical XOR on the accumulator and the contents of memory.
    /// Flags: Negative, Zero
    fn eor(&mut self) -> u8 {
        self.fetch();

        self.accumulator ^= self.fetched;
        self.set_flag(Flag::Zero, self.accumulator == 0);
        self.set_flag(Flag::Negative, (self.accumulator & 0x80) != 0);

        0
    }

    /// Instruction: Increment Memory
    /// Function: Increment the contents of memory by one.
    /// Flags: Negative, Zero
    fn inc(&mut self) -> u8 {
        self.fetch();

        let temp = self.fetched + 1;
        self.write(self.address_absolute, temp as u8);
        self.set_flag(Flag::Zero, (temp as u8) == 0);
        self.set_flag(Flag::Negative, (temp & 0x0080) != 0);

        0
    }

    /// Instruction: Increment X Register
    /// Function: Increment the contents of the X register by one.
    /// Flags: Negative, Zero
    fn inx(&mut self) -> u8 {
        self.register_x += 1;
        self.set_flag(Flag::Zero, self.register_x == 0);
        self.set_flag(Flag::Negative, (self.register_x & 0x80) != 0);

        0
    }

    /// Instruction: Increment Y Register
    /// Function: Increment the contents of the Y register by one.
    /// Flags: Negative, Zero
    fn iny(&mut self) -> u8 {
        self.register_y += 1;
        self.set_flag(Flag::Zero, self.register_y == 0);
        self.set_flag(Flag::Negative, (self.register_y & 0x80) != 0);

        0
    }

    /// Instruction: Jump
    /// Function: Set program counter to address
    /// Flags: None
    fn jmp(&mut self) -> u8 {
        self.program_counter = self.address_absolute;
        0
    }

    /// Instruction: Jump to Subroutine
    /// Function: Push program counter to stack and set program counter to address
    /// Flags: None
    fn jsr(&mut self) -> u8 {
        self.program_counter -= 1;
        
        self.write(0x0100 + self.stack_pointer as u16, (self.program_counter >> 8) as u8);
        self.stack_pointer -= 1;
        self.write(0x0100 + self.stack_pointer as u16, self.program_counter as u8);
        self.stack_pointer -= 1;

        self.program_counter = self.address_absolute;
        0
    }

    /// Instruction: Load Accumulator
    /// Function: Load the accumulator with the contents of memory.
    /// Flags: Negative, Zero
    fn lda(&mut self) -> u8 {
        self.fetch();

        self.accumulator = self.fetched;
        self.set_flag(Flag::Zero, self.accumulator == 0);
        self.set_flag(Flag::Negative, (self.accumulator & 0x80) != 0);

        1
    }

    /// Instruction: Load X Register
    /// Function: Load the X register with the contents of memory.
    /// Flags: Negative, Zero
    fn ldx(&mut self) -> u8 {
        self.fetch();

        self.register_x = self.fetched;
        self.set_flag(Flag::Zero, self.register_x == 0);
        self.set_flag(Flag::Negative, (self.register_x & 0x80) != 0);

        1
    }

    /// Instruction: Load Y Register
    /// Function: Load the Y register with the contents of memory.
    /// Flags: Negative, Zero
    fn ldy(&mut self) -> u8 {
        self.fetch();

        self.register_y = self.fetched;
        self.set_flag(Flag::Zero, self.register_y == 0);
        self.set_flag(Flag::Negative, (self.register_y & 0x80) != 0);

        1
    }

    /// Instruction: Logical Shift Right
    /// Function: Perform a logical shift right on the accumulator.
    /// Flags: Carry, Zero, Negative
    fn lsr(&mut self) -> u8 {
        self.fetch();

        self.set_flag(Flag::Carry, self.fetched & 0x01 != 0);
        let temp = self.fetched >> 1;
        self.set_flag(Flag::Zero, (temp as u8) == 0);
        self.set_flag(Flag::Negative, (temp & 0x80) != 0);

        if self.instructions[self.opcode as usize].address_mode == AddressMode::IMP {
            self.accumulator = temp as u8;
        } else {
            self.write(self.address_absolute, temp as u8);
        }

        0
    }

    /// Instruction: No Operation
    /// Function: Do nothing.
    /// Flags: None
    fn nop(&mut self) -> u8 {
        match self.opcode {
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => return 1,
            _ => todo!("Cover NOP instruction {}", self.opcode),
        }
        
        0
    }

    /// Instruction: Bitwise Logic OR
    /// Function: Perform a bitwise logical OR on the accumulator and the contents of memory.
    /// Flags: Negative, Zero
    fn ora(&mut self) -> u8 {
        self.fetch();

        self.accumulator |= self.fetched;
        self.set_flag(Flag::Zero, self.accumulator == 0);
        self.set_flag(Flag::Negative, (self.accumulator & 0x80) != 0);

        1
    }

    /// Instruction: Push Accumulator
    /// Function: Push the accumulator to the stack.
    /// Flags: None
    fn pha(&mut self) -> u8 {
        self.write(0x0100 + self.stack_pointer as u16, self.accumulator);
        self.stack_pointer -= 1;
        0
    }

    /// Instruction: Push Processor Status
    /// Function: Push processor status to the stack.
    /// Flags: None
    /// Note: The break flag is set before the
    fn php(&mut self) -> u8 {
        let break_or_unused = self.get_flag(Flag::Break) as u8 | self.get_flag(Flag::Unused) as u8;
        self.write(0x0100 + self.stack_pointer as u16, self.status | break_or_unused);
        self.set_flag(Flag::Break, false);
        self.set_flag(Flag::Unused, false);
        self.stack_pointer -= 1;
        
        0
    }

    /// Instruction: Pop Accumulator
    /// Function: Pop a byte from the stack into the accumulator.
    /// Flags: Negative, Zero
    fn pla(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.accumulator = self.read(0x0100 + self.stack_pointer as u16);
        self.set_flag(Flag::Zero, self.accumulator == 0);
        self.set_flag(Flag::Negative, (self.accumulator & 0x80) != 0);

        0
    }

    /// Instruction: Pop Processor Status
    /// Function: Pop a byte from the stack into the processor status.
    /// Flags: None
    fn plp(&mut self) -> u8 {
        self.stack_pointer += 1;
        let status = self.read(0x0100 + self.stack_pointer as u16);
        self.set_flag(Flag::Unused, true);

        0
    }

    /// Instruction: Rotate Left
    /// Function: Perform a bitwise rotate left on the accumulator.
    /// Flags: Carry, Zero, Negative
    fn rol(&mut self) -> u8 {
        self.fetch();

        let temp = (self.fetched << 1) as u16 | self.get_flag(Flag::Carry) as u16;
        self.set_flag(Flag::Carry, (temp & 0xFF00) != 0);
        self.set_flag(Flag::Zero, (temp as u8) == 0);
        self.set_flag(Flag::Negative, (temp & 0x80) != 0);

        if self.instructions[self.opcode as usize].address_mode == AddressMode::IMP {
            self.accumulator = temp as u8;
        } else {
            self.write(self.address_absolute, temp as u8);
        }

        0
    }

    /// Instruction: Rotate Right
    /// Function: Perform a bitwise rotate right on the accumulator.
    /// Flags: Carry, Zero, Negative
    fn ror(&mut self) -> u8 {
        self.fetch();

        let temp = ((self.get_flag(Flag::Carry) as u16) << 7) | (self.fetched >> 1) as u16;
        self.set_flag(Flag::Carry, self.fetched & 0x01 != 0);
        self.set_flag(Flag::Zero, (temp as u8) == 0);
        self.set_flag(Flag::Negative, (temp & 0x80) != 0);
        if self.instructions[self.opcode as usize].address_mode == AddressMode::IMP {
            self.accumulator = temp as u8;
        } else {
            self.write(self.address_absolute, temp as u8);
        }

        0
    }

    /// Instruction: Return from Interrupt
    /// Function: Return from an interrupt.
    /// Flags: None
    fn rti(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.status = self.read(0x0100 + self.stack_pointer as u16);
        self.status &= !(Flag::Break as u8);
        self.status &= !(Flag::Unused as u8);

        self.stack_pointer += 1;
        self.program_counter = self.read(0x0100 + self.stack_pointer as u16) as u16;
        self.stack_pointer += 1;
        self.program_counter |= (self.read(0x0100 + self.stack_pointer as u16) as u16) << 8;

        0
    }

    /// Instruction: Return from Subroutine
    /// Function: Return from a subroutine.
    /// Flags: None
    fn rts(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.program_counter = self.read(0x0100 + self.stack_pointer as u16) as u16;
        self.stack_pointer += 1;
        self.program_counter |= (self.read(0x0100 + self.stack_pointer as u16) as u16) << 8;
        
        self.program_counter += 1;
        
        0
    }

    /// Instruction: Set Carry Flag
    /// Function: Set the carry flag.
    /// Flags: Carry
    fn sec(&mut self) -> u8 {
        self.set_flag(Flag::Carry, true);
        
        0
    }

    /// Instruction: Set Decimal Flag
    /// Function: Set the decimal flag.
    /// Flags: Decimal
    fn sed(&mut self) -> u8 {
        self.set_flag(Flag::Decimal, true);
        
        0
    }

    /// Instruction: Set Interrupt Disable Flag
    /// Function: Set the interrupt disable flag.
    /// Flags: Interrupt Disable
    fn sei(&mut self) -> u8 {
        self.set_flag(Flag::DisableInterrupt, true);
        
        0
    }

    /// Instruction: Store Accumulator
    /// Function: Store the accumulator in memory.
    /// Flags: None
    fn sta(&mut self) -> u8 {
        self.write(self.address_absolute, self.accumulator);
        
        0
    }

    /// Instruction: Store X Register
    /// Function: Store the X register in memory.
    /// Flags: None
    fn stx(&mut self) -> u8 {
        self.write(self.address_absolute, self.register_x);
        
        0
    }

    /// Instruction: Store Y Register
    /// Function: Store the Y register in memory.
    /// Flags: None
    fn sty(&mut self) -> u8 {
        self.write(self.address_absolute, self.register_y);
        
        0
    }

    /// Instruction: Transfer Accumulator to X Register
    /// Function: Transfer the accumulator to the X register.
    /// Flags: Negative, Zero
    fn tax(&mut self) -> u8 {
        self.register_x = self.accumulator;
        self.set_flag(Flag::Zero, self.register_x == 0);
        self.set_flag(Flag::Negative, (self.register_x & 0x80) != 0);

        0
    }

    /// Instruction: Transfer Accumulator to Y Register
    /// Function: Transfer the accumulator to the Y register.
    /// Flags: Negative, Zero
    fn tay(&mut self) -> u8 {
        self.register_y = self.accumulator;
        self.set_flag(Flag::Zero, self.register_y == 0);
        self.set_flag(Flag::Negative, (self.register_y & 0x80) != 0);

        0
    }

    /// Instruction: Transfer Stack Pointer to X Register
    /// Function: Transfer the stack pointer to the X register.
    /// Flags: Negative, Zero
    fn tsx(&mut self) -> u8 {
        self.register_x = self.stack_pointer;
        self.set_flag(Flag::Zero, self.register_x == 0);
        self.set_flag(Flag::Negative, (self.register_x & 0x80) != 0);

        0
    }

    /// Instruction: Transfer X Register to Accumulator
    /// Function: Transfer the X register to the accumulator.
    /// Flags: Negative, Zero
    fn txa(&mut self) -> u8 {
        self.accumulator = self.register_x;
        self.set_flag(Flag::Zero, self.accumulator == 0);
        self.set_flag(Flag::Negative, (self.accumulator & 0x80) != 0);

        0
    }

    /// Instruction: Transfer X Register to Stack Pointer
    /// Function: Transfer the X register to the stack pointer.
    /// Flags: None
    fn txs(&mut self) -> u8 {
        self.stack_pointer = self.register_x;
        
        0
    }

    /// Instruction: Transfer Y Register to Accumulator
    /// Function: Transfer the Y register to the accumulator.
    /// Flags: Negative, Zero
    fn tya(&mut self) -> u8 {
        self.accumulator = self.register_y;
        self.set_flag(Flag::Zero, self.accumulator == 0);
        self.set_flag(Flag::Negative, (self.accumulator & 0x80) != 0);

        0
    }

    /// Illegal Instruction
    fn xxx(&mut self) -> u8 {
        0
    }

    fn complete(&self) -> bool {
        self.remaining_cycles == 0
    }
}
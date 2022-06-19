use super::CPU;

impl CPU {
    pub fn call_addressing_mode(&mut self, address_mode: AddressMode) -> u8 {
        match address_mode {
            AddressMode::IMP => return self.imp(),
            AddressMode::IMM => return self.imm(),
            AddressMode::ZP0 => return self.zp0(),
            AddressMode::ZPX => return self.zpx(),
            AddressMode::ZPY => return self.zpy(),
            AddressMode::REL => return self.rel(),
            AddressMode::ABS => return self.abs(),
            AddressMode::ABX => return self.abx(),
            AddressMode::ABY => return self.aby(),
            AddressMode::IND => return self.ind(),
            AddressMode::IZX => return self.izx(),
            AddressMode::IZY => return self.izy(),
        }
    }

    /// Address Mode: Implied
    fn imp(&mut self) -> u8 {
        self.fetched = self.accumulator;

        0
    }

    /// Address Mode: Immediate
    fn imm(&mut self) -> u8 {
        self.address_absolute = self.program_counter;
        self.program_counter += 1;

        0
    }

    /// Address Mode: Zerp Page
    fn zp0(&mut self) -> u8 {
        self.address_absolute = self.read(self.program_counter) as u16;
        self.program_counter += 1;
        self.address_absolute &= 0x00FF;

        0        
    }

    /// Address Mode: Zero Page with X Offset
    fn zpx(&mut self) -> u8 {
        self.address_absolute = (self.read(self.program_counter) + self.register_x) as u16;
        self.program_counter += 1;
        self.address_absolute &= 0x00FF;

        0
    }

    /// Address Mode: Zero Page with Y Offset
    fn zpy(&mut self) -> u8 {
        self.address_absolute = (self.read(self.program_counter) + self.register_y) as u16;
        self.program_counter += 1;
        self.address_absolute &= 0x00FF;

        0
    }

    /// Address Mode: Relative
    fn rel(&mut self) -> u8 {
        self.address_relative = self.read(self.program_counter) as u16;
        self.program_counter += 1;

        if self.address_relative & 0x80 != 0 {
            self.address_relative |= 0xFF00;
        }

        0
    }
 
    /// Address Mode: Absolute
    fn abs(&mut self) -> u8 {
        let lo = self.read(self.program_counter) as u16;
        self.program_counter += 1;
        let hi = self.read(self.program_counter) as u16;
        self.program_counter += 1;

        self.address_absolute = (hi << 8) | lo;

        0
    }

    /// Address Mode: Absolute with X Offset
    fn abx(&mut self) -> u8 {
        let lo = self.read(self.program_counter) as u16;
        self.program_counter += 1;
        let hi = self.read(self.program_counter) as u16;
        self.program_counter += 1;

        self.address_absolute = (hi << 8) | lo;
        self.address_absolute += self.register_x as u16;

        if (self.address_absolute & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    /// Address Mode: Absolute with Y Offset
    fn aby(&mut self) -> u8 {
        let lo = self.read(self.program_counter) as u16;
        self.program_counter += 1;
        let hi = self.read(self.program_counter) as u16;
        self.program_counter += 1;

        self.address_absolute = (hi << 8) | lo;
        self.address_absolute += self.register_y as u16;

        if (self.address_absolute & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    /// Address Mode: Indirect
    fn ind(&mut self) -> u8 {
        let ptr_lo = self.read(self.program_counter) as u16;
        self.program_counter += 1;
        let ptr_hi = self.read(self.program_counter) as u16;
        self.program_counter += 1;

        let ptr = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF {
            self.address_absolute = ((self.read(ptr & 0xFF00) as u16) << 8) | self.read(ptr + 0) as u16;
        } else {
            self.address_absolute = (self.read(ptr + 1) as u16) << 8 | self.read(ptr + 0) as u16;
        }

        0
    }

    /// Address Mode: Indirect X
    fn izx(&mut self) -> u8 {
        let t = self.read(self.program_counter) as u16;
        self.program_counter += 1;

        let lo = self.read((t + self.register_x as u16) & 0x00FF) as u16;
        let hi = self.read((t + self.register_x as u16 + 1) & 0x00FF) as u16;

        self.address_absolute = (hi << 8) | lo;

        0
    }

    /// Address Mode: Indirect Y
    fn izy(&mut self) -> u8 {
        let t = self.read(self.program_counter) as u16;
        self.program_counter += 1;

        let lo = self.read(t & 0x00FF) as u16;
        let hi = self.read((t+ 1) & 0x00FF) as u16;

        self.address_absolute = (hi << 8) | lo;
        self.address_absolute += self.register_y as u16;

        if (self.address_absolute & 0x00FF) != (hi <<8) {
            1
        } else {
            0
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AddressMode {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    REL,
    ABS,
    ABX,
    ABY,
    IND,
    IZX,
    IZY,
}
use crate::cpu::CPU;

pub struct Bus {
    cpu: CPU,
    ram: [u8; 64 * 1024],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cpu: CPU::default(),
            ram: [0; 64 * 1024],
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }

    pub fn read(&self, address: u16, read_only: bool) -> u8 {
        self.ram[address as usize]
    }
}
mod instructions;
mod addressing_modes;
mod operations;

use crate::bus::Bus;
use self::instructions::Instruction;

enum Flag {
    Carry               = (1 << 0),
    Zero                = (1 << 1),
    DisableInterrupt    = (1 << 2),
    Decimal             = (1 << 3),
    Break               = (1 << 4),
    Unused              = (1 << 5),
    Overflow            = (1 << 6),
    Negative            = (1 << 7),
}

pub struct CPU {
    pub accumulator: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub status: u8,

    instructions: [Instruction; 256],

    bus: *mut Bus,
    fetched: u8,
    address_absolute: u16, 
    address_relative: u16,
    opcode: u8,
    remaining_cycles: u8,
}

impl CPU
{
    pub fn connect_bus(&mut self, n: *mut Bus) {
        self.bus = n;
    }

    pub fn read(&self, address: u16) -> u8 {
        unsafe {
            (*self.bus).read(address, false)
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        unsafe {
            (*self.bus).write(address, data);
        }
    }

    pub fn clock(&mut self) {
        if self.remaining_cycles == 0 {
            self.opcode = self.read(self.program_counter);
            self.program_counter += 1;

            self.remaining_cycles = self.instructions[self.opcode as usize].number_cycles;

            let additional_cycle1 = self.call_addressing_mode(
                self.instructions[self.opcode as usize].address_mode);
            let additional_cycle2 = self.call_operation(
                self.instructions[self.opcode as usize].operation);
            
            self.remaining_cycles += (additional_cycle1 & additional_cycle2);

        }

        self.remaining_cycles -= 1;
    }

    pub fn reset() {
        todo!()
    }

    pub fn interrupt_request() {
        todo!()
    }

    pub fn non_maskable_interrupt() {
        todo!()
    }

    fn get_flag(&self, flag: Flag) -> bool {
        todo!()
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        todo!()
    }

}

impl Default for CPU
{
    fn default() -> Self {
        Self { 
            accumulator: Default::default(),
            register_x: Default::default(), 
            register_y: Default::default(), 
            stack_pointer: Default::default(), 
            program_counter: Default::default(), 
            status: Default::default(),

            instructions: Instruction::get_all(),

            bus: std::ptr::null_mut(),
            fetched: Default::default(),
            address_absolute: Default::default(),
            address_relative: Default::default(),
            opcode: Default::default(),
            remaining_cycles: Default::default(),
            
        }
    }
}
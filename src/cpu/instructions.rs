use super::operations::Opcode;
use super::addressing_modes::AddressMode;

pub(super) struct Instruction {
    pub name: InstructionName,
    pub operation: Opcode,
    pub address_mode: AddressMode,
    pub number_cycles: u8,
}

impl Instruction {
    pub fn get_all() -> [Instruction; 256] {
        [Instruction {
            name: todo!(),
            operation: todo!(),
            address_mode: todo!(),
            number_cycles: todo!(),
        }; 256]
    }
}

pub enum InstructionName {

}
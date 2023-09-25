use super::{
    memory::{CpuMemoryError, Memory},
    Register,
};

use thiserror::Error;

pub mod opcodes;
use opcodes::*;

pub mod addressing_mode;
use addressing_mode::*;

#[derive(Debug)]
pub enum Instruction {
    Break,
    Adc {
        addressing_mode: AddressingMode,
    },
    And {
        addressing_mode: AddressingMode,
    },
    Asl {
        addressing_mode: AddressingMode,
    },
    Bcc {
        addressing_mode: AddressingMode,
    },
    Ld {
        destination: Register,
        addressing_mode: AddressingMode,
    },
    Trr {
        origin: Register,
        destination: Register,
    },
    In {
        destination: Register,
    },
}

#[derive(Debug, Error)]
pub enum InstructionError {
    #[error("invalid instruction code {:#03x}", code)]
    InvalidInstructionCode { code: u8 },
    #[error(transparent)]
    CpuMemoryError(#[from] CpuMemoryError),
}

impl Instruction {
    pub fn get_instruction(
        memory: &Memory,
        program_counter: &u16,
    ) -> Result<(Instruction, u16), InstructionError> {
        let mut program_counter = *program_counter;
        let instruction = memory.read(program_counter)?;
        program_counter += 1;
        let instruction = match instruction {
            0x00 => Instruction::Break,
            ADC_IMMEDIATE => {
                let addressing_mode = AddressingMode::immediate(memory, &mut program_counter)?;
                Instruction::Adc { addressing_mode }
            }
            ADC_ZERO_PAGE => {
                let addressing_mode = AddressingMode::zero_page(memory, &mut program_counter)?;
                Instruction::Adc { addressing_mode }
            }
            ADC_ZERO_PAGE_X => {
                let addressing_mode = AddressingMode::zero_page_x(memory, &mut program_counter)?;
                Instruction::Adc { addressing_mode }
            }
            ADC_ABSOLUTE => {
                let addressing_mode = AddressingMode::absolute(memory, &mut program_counter)?;
                Instruction::Adc { addressing_mode }
            }
            ADC_ABSOLUTE_X => {
                let addressing_mode = AddressingMode::absolute_x(memory, &mut program_counter)?;
                Instruction::Adc { addressing_mode }
            }
            ADC_ABSOLUTE_Y => {
                let addressing_mode = AddressingMode::absolute_y(memory, &mut program_counter)?;
                Instruction::Adc { addressing_mode }
            }
            ADC_INDIRECT_X => {
                let addressing_mode = AddressingMode::indirect_x(memory, &mut program_counter)?;
                Instruction::Adc { addressing_mode }
            }
            ADC_INDIRECT_Y => {
                let addressing_mode = AddressingMode::indirect_y(memory, &mut program_counter)?;
                Instruction::Adc { addressing_mode }
            }
            AND_IMMEDIATE => {
                let addressing_mode = AddressingMode::immediate(memory, &mut program_counter)?;
                Instruction::And { addressing_mode }
            }
            AND_ZERO_PAGE => {
                let addressing_mode = AddressingMode::zero_page(memory, &mut program_counter)?;
                Instruction::And { addressing_mode }
            }
            AND_ZERO_PAGE_X => {
                let addressing_mode = AddressingMode::zero_page_x(memory, &mut program_counter)?;
                Instruction::And { addressing_mode }
            }
            AND_ABSOLUTE => {
                let addressing_mode = AddressingMode::absolute(memory, &mut program_counter)?;
                Instruction::And { addressing_mode }
            }
            AND_ABSOLUTE_X => {
                let addressing_mode = AddressingMode::absolute_x(memory, &mut program_counter)?;
                Instruction::And { addressing_mode }
            }
            AND_ABSOLUTE_Y => {
                let addressing_mode = AddressingMode::absolute_y(memory, &mut program_counter)?;
                Instruction::And { addressing_mode }
            }
            AND_INDIRECT_X => {
                let addressing_mode = AddressingMode::indirect_x(memory, &mut program_counter)?;
                Instruction::And { addressing_mode }
            }
            AND_INDIRECT_Y => {
                let addressing_mode = AddressingMode::indirect_y(memory, &mut program_counter)?;
                Instruction::And { addressing_mode }
            }
            ASL_ACCUMULATOR => Instruction::Asl {
                addressing_mode: AddressingMode::Accumulator,
            },
            ASL_ZERO_PAGE => {
                let addressing_mode = AddressingMode::zero_page(memory, &mut program_counter)?;
                Instruction::Asl { addressing_mode }
            }
            ASL_ZERO_PAGE_X => {
                let addressing_mode = AddressingMode::zero_page_x(memory, &mut program_counter)?;
                Instruction::Asl { addressing_mode }
            }
            ASL_ABSOLUTE => {
                let addressing_mode = AddressingMode::absolute(memory, &mut program_counter)?;
                Instruction::Asl { addressing_mode }
            }
            ASL_ABSOLUTE_X => {
                let addressing_mode = AddressingMode::absolute_x(memory, &mut program_counter)?;
                Instruction::Asl { addressing_mode }
            }
            BCC => {
                let addressing_mode = AddressingMode::relative(memory, &mut program_counter)?;
                Instruction::Bcc { addressing_mode }
            }
            LDA_IMMEDIATE => {
                let addressing_mode = AddressingMode::immediate(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_ZERO_PAGE => {
                let addressing_mode = AddressingMode::zero_page(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_ZERO_PAGE_X => {
                let addressing_mode = AddressingMode::zero_page_x(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_ABSOLUTE => {
                let addressing_mode = AddressingMode::absolute(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_ABSOLUTE_X => {
                let addressing_mode = AddressingMode::absolute_x(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_ABSOLUTE_Y => {
                let addressing_mode = AddressingMode::absolute_y(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_INDIRECT_X => {
                let addressing_mode = AddressingMode::indirect_x(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_INDIRECT_Y => {
                let addressing_mode = AddressingMode::indirect_y(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDX_IMMEDIATE => {
                let addressing_mode = AddressingMode::immediate(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::X,
                    addressing_mode,
                }
            }
            LDX_ZERO_PAGE => {
                let addressing_mode = AddressingMode::zero_page(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::X,
                    addressing_mode,
                }
            }
            LDX_ZERO_PAGE_Y => {
                let addressing_mode = AddressingMode::zero_page_y(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::X,
                    addressing_mode,
                }
            }
            LDX_ABSOLUTE => {
                let addressing_mode = AddressingMode::absolute(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::X,
                    addressing_mode,
                }
            }
            LDX_ABSOLUTE_Y => {
                let addressing_mode = AddressingMode::absolute_y(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::X,
                    addressing_mode,
                }
            }
            LDY_IMMEDIATE => {
                let addressing_mode = AddressingMode::immediate(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::Y,
                    addressing_mode,
                }
            }
            LDY_ZERO_PAGE => {
                let addressing_mode = AddressingMode::zero_page(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::Y,
                    addressing_mode,
                }
            }
            LDY_ZERO_PAGE_X => {
                let addressing_mode = AddressingMode::zero_page_x(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::Y,
                    addressing_mode,
                }
            }
            LDY_ABSOLUTE => {
                let addressing_mode = AddressingMode::absolute(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::Y,
                    addressing_mode,
                }
            }
            LDY_ABSOLUTE_X => {
                let addressing_mode = AddressingMode::absolute_x(memory, &mut program_counter)?;
                Instruction::Ld {
                    destination: Register::Y,
                    addressing_mode,
                }
            }
            TAX => Instruction::Trr {
                origin: Register::A,
                destination: Register::X,
            },
            INX => Instruction::In {
                destination: Register::X,
            },
            code => {
                return Err(InstructionError::InvalidInstructionCode { code });
            }
        };

        Ok((instruction, program_counter))
    }
}

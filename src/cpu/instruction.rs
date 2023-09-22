use super::{
    memory::{CpuMemoryError, Memory},
    Register,
};

pub mod opcodes;

use opcodes::*;
use thiserror::Error;

#[derive(Debug)]
pub enum AddressingMode {
    Accumulator,
    Immediate { immediate: u8 },
    ZeroPage { address: u8 },
    ZeroPageX { address: u8 },
    ZeroPageY { address: u8 },
    Absolute { address: u16 },
    AbsoluteX { address: u16 },
    AbsoluteY { address: u16 },
    IndirectX { address: u8 },
    IndirectY { address: u8 },
}

impl AddressingMode {
    pub fn immediate(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> Result<AddressingMode, CpuMemoryError> {
        let immediate = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(AddressingMode::Immediate { immediate })
    }

    pub fn zero_page(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> Result<AddressingMode, CpuMemoryError> {
        let address = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(AddressingMode::ZeroPage { address })
    }

    pub fn zero_page_x(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> Result<AddressingMode, CpuMemoryError> {
        let address: u8 = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(AddressingMode::ZeroPageX { address })
    }

    pub fn zero_page_y(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> Result<AddressingMode, CpuMemoryError> {
        let address: u8 = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(AddressingMode::ZeroPageY { address })
    }

    pub fn absolute(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> Result<AddressingMode, CpuMemoryError> {
        let address = memory.read_u16(*program_counter)?;
        *program_counter += 2;
        Ok(AddressingMode::Absolute { address })
    }

    pub fn absolute_x(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> Result<AddressingMode, CpuMemoryError> {
        let address = memory.read_u16(*program_counter)?;
        *program_counter += 2;
        Ok(AddressingMode::AbsoluteX { address })
    }

    pub fn absolute_y(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> Result<AddressingMode, CpuMemoryError> {
        let address = memory.read_u16(*program_counter)?;
        *program_counter += 2;
        Ok(AddressingMode::AbsoluteY { address })
    }

    pub fn indirect_x(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> Result<AddressingMode, CpuMemoryError> {
        let address = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(AddressingMode::IndirectX { address })
    }

    pub fn indirect_y(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> Result<AddressingMode, CpuMemoryError> {
        let address = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(AddressingMode::IndirectY { address })
    }
}

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

use super::{
    memory::{CpuMemoryError, Memory},
    status::Flag,
    Register,
};

use derives::AddressingEnum;
use thiserror::Error;

pub mod opcodes;

#[cfg(test)]
mod tests;

pub mod addressing_mode;
use addressing_mode as AM;

#[derive(Debug, AddressingEnum)]
pub enum Instruction {
    Break,
    #[modes(
        mode = "immediate",
        mode = "zero_page",
        mode = "zero_page_x",
        mode = "zero_page_y",
        mode = "absolute",
        mode = "absolute_x",
        mode = "absolute_y",
        mode = "indirect_x",
        mode = "indirect_y"
    )]
    Adc {
        addressing_mode: AdcAddressingMode,
    },
    #[modes(
        mode = "immediate",
        mode = "zero_page",
        mode = "zero_page_x",
        mode = "absolute",
        mode = "absolute_x",
        mode = "absolute_y",
        mode = "indirect_x",
        mode = "indirect_y"
    )]
    And {
        addressing_mode: AndAddressingMode,
    },
    #[modes(
        mode = "accumulator",
        mode = "zero_page",
        mode = "zero_page_x",
        mode = "absolute",
        mode = "absolute_x"
    )]
    Asl {
        addressing_mode: AslAddressingMode,
    },
    Branch {
        addressing_mode: AM::Relative,
        flag: Flag,
        branch_if: bool,
    },
    #[modes(mode = "zero_page", mode = "absolute")]
    Bit {
        addressing_mode: BitAddressingMode,
    },
    Clear {
        flag: Flag,
    },
    #[modes(
        mode = "immediate",
        mode = "zero_page",
        mode = "zero_page_x",
        mode = "zero_page_y",
        mode = "absolute",
        mode = "absolute_x",
        mode = "absolute_y",
        mode = "indirect_x",
        mode = "indirect_y"
    )]
    Ld {
        destination: Register,
        addressing_mode: LdAddressingMode,
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
        use opcodes::*;
        let mut program_counter = *program_counter;
        let instruction = memory.read(program_counter);
        program_counter += 1;
        let instruction = match instruction {
            ADC_IMMEDIATE => {
                let addressing_mode = AdcAddressingMode::Immediate {
                    mode: AM::Immediate::new(memory, &mut program_counter),
                };
                Instruction::Adc { addressing_mode }
            }
            ADC_ZERO_PAGE => {
                let addressing_mode = AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage::new(memory, &mut program_counter),
                    },
                };
                Instruction::Adc { addressing_mode }
            }
            ADC_ZERO_PAGE_X => {
                let addressing_mode = AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::ZeroPageX {
                        mode: AM::ZeroPageX::new(memory, &mut program_counter),
                    },
                };
                Instruction::Adc { addressing_mode }
            }
            ADC_ABSOLUTE => {
                let addressing_mode = AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::Absolute {
                        mode: AM::Absolute::new(memory, &mut program_counter),
                    },
                };
                Instruction::Adc { addressing_mode }
            }
            ADC_ABSOLUTE_X => {
                let addressing_mode = AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::AbsoluteX {
                        mode: AM::AbsoluteX::new(memory, &mut program_counter),
                    },
                };
                Instruction::Adc { addressing_mode }
            }
            ADC_ABSOLUTE_Y => {
                let addressing_mode = AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::AbsoluteY {
                        mode: AM::AbsoluteY::new(memory, &mut program_counter),
                    },
                };
                Instruction::Adc { addressing_mode }
            }
            ADC_INDIRECT_X => {
                let addressing_mode = AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::IndirectX {
                        mode: AM::IndirectX::new(memory, &mut program_counter),
                    },
                };
                Instruction::Adc { addressing_mode }
            }
            ADC_INDIRECT_Y => {
                let addressing_mode = AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::IndirectY {
                        mode: AM::IndirectY::new(memory, &mut program_counter),
                    },
                };
                Instruction::Adc { addressing_mode }
            }
            AND_IMMEDIATE => {
                let addressing_mode = AndAddressingMode::Immediate {
                    mode: AM::Immediate::new(memory, &mut program_counter),
                };
                Instruction::And { addressing_mode }
            }
            AND_ZERO_PAGE => {
                let addressing_mode = AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage::new(memory, &mut program_counter),
                    },
                };
                Instruction::And { addressing_mode }
            }
            AND_ZERO_PAGE_X => {
                let addressing_mode = AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::ZeroPageX {
                        mode: AM::ZeroPageX::new(memory, &mut program_counter),
                    },
                };
                Instruction::And { addressing_mode }
            }
            AND_ABSOLUTE => {
                let addressing_mode = AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::Absolute {
                        mode: AM::Absolute::new(memory, &mut program_counter),
                    },
                };
                Instruction::And { addressing_mode }
            }
            AND_ABSOLUTE_X => {
                let addressing_mode = AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::AbsoluteX {
                        mode: AM::AbsoluteX::new(memory, &mut program_counter),
                    },
                };
                Instruction::And { addressing_mode }
            }
            AND_ABSOLUTE_Y => {
                let addressing_mode = AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::AbsoluteY {
                        mode: AM::AbsoluteY::new(memory, &mut program_counter),
                    },
                };
                Instruction::And { addressing_mode }
            }
            AND_INDIRECT_X => {
                let addressing_mode = AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::IndirectX {
                        mode: AM::IndirectX::new(memory, &mut program_counter),
                    },
                };
                Instruction::And { addressing_mode }
            }
            AND_INDIRECT_Y => {
                let addressing_mode = AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::IndirectY {
                        mode: AM::IndirectY::new(memory, &mut program_counter),
                    },
                };
                Instruction::And { addressing_mode }
            }
            ASL_ACCUMULATOR => Instruction::Asl {
                addressing_mode: AslAddressingMode::Accumulator {
                    mode: AM::Accumulator {},
                },
            },
            ASL_ZERO_PAGE => {
                let addressing_mode = AslAddressingMode::AslAddressAddressingMode {
                    mode: AslAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage::new(memory, &mut program_counter),
                    },
                };
                Instruction::Asl { addressing_mode }
            }
            ASL_ZERO_PAGE_X => {
                let addressing_mode = AslAddressingMode::AslAddressAddressingMode {
                    mode: AslAddressAddressingMode::ZeroPageX {
                        mode: AM::ZeroPageX::new(memory, &mut program_counter),
                    },
                };
                Instruction::Asl { addressing_mode }
            }
            ASL_ABSOLUTE => {
                let addressing_mode = AslAddressingMode::AslAddressAddressingMode {
                    mode: AslAddressAddressingMode::Absolute {
                        mode: AM::Absolute::new(memory, &mut program_counter),
                    },
                };
                Instruction::Asl { addressing_mode }
            }
            ASL_ABSOLUTE_X => {
                let addressing_mode = AslAddressingMode::AslAddressAddressingMode {
                    mode: AslAddressAddressingMode::AbsoluteX {
                        mode: AM::AbsoluteX::new(memory, &mut program_counter),
                    },
                };
                Instruction::Asl { addressing_mode }
            }
            BCC => Instruction::Branch {
                addressing_mode: AM::Relative::new(memory, &mut program_counter),
                flag: Flag::Carry,
                branch_if: false,
            },
            BCS => Instruction::Branch {
                addressing_mode: AM::Relative::new(memory, &mut program_counter),
                flag: Flag::Carry,
                branch_if: true,
            },
            BEQ => Instruction::Branch {
                addressing_mode: AM::Relative::new(memory, &mut program_counter),
                flag: Flag::Zero,
                branch_if: true,
            },
            BIT_ZERO_PAGE => {
                let addressing_mode = BitAddressingMode::ZeroPage {
                    mode: AM::ZeroPage::new(memory, &mut program_counter),
                };
                Instruction::Bit { addressing_mode }
            }
            BIT_ABSOLUTE => {
                let addressing_mode = BitAddressingMode::Absolute {
                    mode: AM::Absolute::new(memory, &mut program_counter),
                };
                Instruction::Bit { addressing_mode }
            }
            BMI => Instruction::Branch {
                addressing_mode: AM::Relative::new(memory, &mut program_counter),
                flag: Flag::Negative,
                branch_if: true,
            },
            BNE => Instruction::Branch {
                addressing_mode: AM::Relative::new(memory, &mut program_counter),
                flag: Flag::Zero,
                branch_if: false,
            },
            BPL => Instruction::Branch {
                addressing_mode: AM::Relative::new(memory, &mut program_counter),
                flag: Flag::Negative,
                branch_if: false,
            },
            BRK => Instruction::Break,
            BVC => Instruction::Branch {
                addressing_mode: AM::Relative::new(memory, &mut program_counter),
                flag: Flag::Overflow,
                branch_if: false,
            },
            BVS => Instruction::Branch {
                addressing_mode: AM::Relative::new(memory, &mut program_counter),
                flag: Flag::Overflow,
                branch_if: true,
            },
            CLC => Instruction::Clear { flag: Flag::Carry },
            LDA_IMMEDIATE => {
                let addressing_mode = LdAddressingMode::Immediate {
                    mode: AM::Immediate::new(memory, &mut program_counter),
                };
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_ZERO_PAGE => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_ZERO_PAGE_X => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPageX {
                        mode: AM::ZeroPageX::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_ABSOLUTE => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::Absolute {
                        mode: AM::Absolute::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_ABSOLUTE_X => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::AbsoluteX {
                        mode: AM::AbsoluteX::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_ABSOLUTE_Y => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::AbsoluteY {
                        mode: AM::AbsoluteY::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_INDIRECT_X => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::IndirectX {
                        mode: AM::IndirectX::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDA_INDIRECT_Y => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::IndirectY {
                        mode: AM::IndirectY::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::A,
                    addressing_mode,
                }
            }
            LDX_IMMEDIATE => {
                let addressing_mode = LdAddressingMode::Immediate {
                    mode: AM::Immediate::new(memory, &mut program_counter),
                };
                Instruction::Ld {
                    destination: Register::X,
                    addressing_mode,
                }
            }
            LDX_ZERO_PAGE => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::X,
                    addressing_mode,
                }
            }
            LDX_ZERO_PAGE_Y => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPageY {
                        mode: AM::ZeroPageY::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::X,
                    addressing_mode,
                }
            }
            LDX_ABSOLUTE => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::Absolute {
                        mode: AM::Absolute::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::X,
                    addressing_mode,
                }
            }
            LDX_ABSOLUTE_Y => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::AbsoluteY {
                        mode: AM::AbsoluteY::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::X,
                    addressing_mode,
                }
            }
            LDY_IMMEDIATE => {
                let addressing_mode = LdAddressingMode::Immediate {
                    mode: AM::Immediate::new(memory, &mut program_counter),
                };
                Instruction::Ld {
                    destination: Register::Y,
                    addressing_mode,
                }
            }
            LDY_ZERO_PAGE => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::Y,
                    addressing_mode,
                }
            }
            LDY_ZERO_PAGE_X => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPageX {
                        mode: AM::ZeroPageX::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::Y,
                    addressing_mode,
                }
            }
            LDY_ABSOLUTE => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::Absolute {
                        mode: AM::Absolute::new(memory, &mut program_counter),
                    },
                };
                Instruction::Ld {
                    destination: Register::Y,
                    addressing_mode,
                }
            }
            LDY_ABSOLUTE_X => {
                let addressing_mode = LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::AbsoluteX {
                        mode: AM::AbsoluteX::new(memory, &mut program_counter),
                    },
                };
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

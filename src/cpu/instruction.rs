use super::{memory::Memory, Register};

pub const LDA_IMMEDIATE: u8 = 0xa9;
pub const LDA_ZERO_PAGE: u8 = 0xa5;
pub const LDA_ZERO_PAGE_X: u8 = 0xb5;
pub const LDA_ABSOLUTE: u8 = 0xad;
pub const LDX_IMMEDIATE: u8 = 0xa2;
pub const LDX_ZERO_PAGE: u8 = 0xa6;
pub const TAX: u8 = 0xaa;
pub const INX: u8 = 0xe8;

#[derive(Debug)]
pub enum LdType {
    Immediate { immediate: u8 },
    ZeroPage { address: u8 },
    ZeroPageX { address: u8 },
    Absolute { address: u16 },
}

#[derive(Debug)]
pub enum Instruction {
    Break,
    Ld {
        destination: Register,
        ld_type: LdType,
    },
    Trr {
        origin: Register,
        destination: Register,
    },
    In {
        destination: Register,
    },
}

fn null_pointer() -> color_eyre::Report {
    color_eyre::eyre::eyre!("Should have instruction at pointer.")
}

impl Instruction {
    pub fn get_instruction(
        memory: &Memory,
        program_counter: &u16,
    ) -> color_eyre::Result<(Instruction, u16)> {
        let mut program_counter = *program_counter;
        let instruction = memory.read(program_counter)?;
        program_counter += 1;
        let instruction = match instruction {
            0x00 => Instruction::Break,
            LDA_IMMEDIATE | LDX_IMMEDIATE => {
                let immediate = memory.read(program_counter)?;
                program_counter += 1;
                let destination = match instruction {
                    LDA_IMMEDIATE => Register::A,
                    LDX_IMMEDIATE => Register::X,
                    _ => panic!("Code should be unreachable."),
                };
                Instruction::Ld {
                    destination,
                    ld_type: LdType::Immediate { immediate },
                }
            }
            LDA_ZERO_PAGE | LDX_ZERO_PAGE => {
                let address = memory.read(program_counter)?;
                program_counter += 1;
                let destination = match instruction {
                    LDA_ZERO_PAGE => Register::A,
                    LDX_ZERO_PAGE => Register::X,
                    _ => panic!("Code should be unreachable."),
                };
                Instruction::Ld {
                    destination,
                    ld_type: LdType::ZeroPage { address },
                }
            }
            LDA_ZERO_PAGE_X => {
                let address = memory.read(program_counter)?;
                program_counter += 1;
                Instruction::Ld {
                    destination: Register::A,
                    ld_type: LdType::ZeroPageX { address },
                }
            }
            LDA_ABSOLUTE => {
                let address = memory.read_u16(program_counter)?;
                program_counter += 2;
                Instruction::Ld {
                    destination: Register::A,
                    ld_type: LdType::Absolute { address },
                }
            }
            TAX => Instruction::Trr {
                origin: Register::A,
                destination: Register::X,
            },

            INX => Instruction::In {
                destination: Register::X,
            },
            _ => {
                return Err(color_eyre::eyre::eyre!(
                    "Should have been able to find instruction!"
                ));
            }
        };

        Ok((instruction, program_counter))
    }
}

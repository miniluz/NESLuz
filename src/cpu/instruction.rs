use super::{memory::Memory, Cpu, RegisterImmutable, RegisterMutable};

pub const LDA_IMMEDIATE: u8 = 0xa9;
pub const LDX_IMMEDIATE: u8 = 0xa2;
pub const TAX: u8 = 0xaa;
pub const INX: u8 = 0xe8;

#[derive(Debug)]
pub enum LdType {
    Immediate { immediate: u8 },
}

pub enum Instruction {
    Break,
    Ld {
        destination: RegisterMutable,
        ld_type: LdType,
    },
    Trr {
        origin: RegisterImmutable,
        destination: RegisterMutable,
    },
    In {
        destination: RegisterMutable,
    },
}

fn null_pointer() -> color_eyre::Report {
    color_eyre::eyre::eyre!("Should have instruction at pointer.")
}

impl Instruction {
    pub fn get_instruction(
        memory: &Memory,
        offset: &u16,
    ) -> color_eyre::Result<(Instruction, u16)> {
        let mut instructions = memory.iter().skip(*offset as usize).enumerate();
        let (offset, instruction) = match instructions.next().ok_or(null_pointer())? {
            (_, 0x00) => (0, Instruction::Break),
            (_, &LDX_IMMEDIATE) => {
                // LDX Immidiate
                let (offset, &immediate) = instructions.next().ok_or(null_pointer())?;
                (
                    offset,
                    Instruction::Ld {
                        destination: Cpu::register_x_mut,
                        ld_type: LdType::Immediate { immediate },
                    },
                )
            }
            (_, &LDA_IMMEDIATE) => {
                let (offset, &immediate) = instructions.next().ok_or(null_pointer())?;
                (
                    offset,
                    Instruction::Ld {
                        destination: Cpu::register_a_mut,
                        ld_type: LdType::Immediate { immediate },
                    },
                )
            }
            (_, &TAX) => (
                // TAX
                0,
                Instruction::Trr {
                    origin: Cpu::register_a,
                    destination: Cpu::register_x_mut,
                },
            ),
            (_, &INX) => (
                // INX
                0,
                Instruction::In {
                    destination: Cpu::register_x_mut,
                },
            ),
            _ => {
                return Err(color_eyre::eyre::eyre!(
                    "Should have been able to find instruction!"
                ));
            }
        };

        Ok((instruction, u16::try_from(offset)? + 1))
    }
}

use super::{memory::Memory, Register};

pub const ADC_IMMEDIATE: u8 = 0x69;
pub const ADC_ZERO_PAGE: u8 = 0x65;
pub const ADC_ZERO_PAGE_X: u8 = 0x75;
pub const ADC_ABSOLUTE: u8 = 0x6d;
pub const ADC_ABSOLUTE_X: u8 = 0x7d;
pub const ADC_ABSOLUTE_Y: u8 = 0x79;
pub const ADC_INDIRECT_X: u8 = 0x61;
pub const ADC_INDIRECT_Y: u8 = 0x71;

pub const LDA_IMMEDIATE: u8 = 0xa9;
pub const LDA_ZERO_PAGE: u8 = 0xa5;
pub const LDA_ZERO_PAGE_X: u8 = 0xb5;
pub const LDA_ABSOLUTE: u8 = 0xad;
pub const LDA_ABSOLUTE_X: u8 = 0xbd;
pub const LDA_ABSOLUTE_Y: u8 = 0xb9;
pub const LDA_INDIRECT_X: u8 = 0xa1;
pub const LDA_INDIRECT_Y: u8 = 0xb1;

pub const LDX_IMMEDIATE: u8 = 0xa2;
pub const LDX_ZERO_PAGE: u8 = 0xa6;
pub const LDX_ZERO_PAGE_Y: u8 = 0xb6;

pub const TAX: u8 = 0xaa;
pub const INX: u8 = 0xe8;

#[derive(Debug)]
pub enum AddressingMode {
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
    ) -> color_eyre::Result<AddressingMode> {
        let immediate = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(AddressingMode::Immediate { immediate })
    }

    pub fn zero_page(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> color_eyre::Result<AddressingMode> {
        let address = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(AddressingMode::ZeroPage { address })
    }

    pub fn zero_page_x(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> color_eyre::Result<AddressingMode> {
        let address: u8 = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(AddressingMode::ZeroPageX { address })
    }

    pub fn zero_page_y(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> color_eyre::Result<AddressingMode> {
        let address: u8 = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(AddressingMode::ZeroPageY { address })
    }

    pub fn absolute(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> color_eyre::Result<AddressingMode> {
        let address = memory.read_u16(*program_counter)?;
        *program_counter += 2;
        Ok(AddressingMode::Absolute { address })
    }

    pub fn absolute_x(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> color_eyre::Result<AddressingMode> {
        let address = memory.read_u16(*program_counter)?;
        *program_counter += 2;
        Ok(AddressingMode::AbsoluteX { address })
    }

    pub fn absolute_y(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> color_eyre::Result<AddressingMode> {
        let address = memory.read_u16(*program_counter)?;
        *program_counter += 2;
        Ok(AddressingMode::AbsoluteY { address })
    }

    pub fn indirect_x(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> color_eyre::Result<AddressingMode> {
        let address = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(AddressingMode::IndirectX { address })
    }

    pub fn indirect_y(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> color_eyre::Result<AddressingMode> {
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

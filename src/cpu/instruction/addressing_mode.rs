use crate::cpu::memory::{CpuMemoryError, Memory};

#[derive(Debug)]
pub enum AddressingMode {
    Accumulator,
    Immediate { immediate: u8 },
    ZeroPage { address: u8 },
    ZeroPageX { address: u8 },
    ZeroPageY { address: u8 },
    Relative { offset: i8 },
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

    pub fn relative(
        memory: &Memory,
        program_counter: &mut u16,
    ) -> Result<AddressingMode, CpuMemoryError> {
        let offset = memory.read(*program_counter)? as i8;
        *program_counter += 1;
        Ok(AddressingMode::Relative { offset })
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

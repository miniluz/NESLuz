use crate::cpu::{
    memory::{CpuMemoryError, Memory},
    Cpu, CpuError,
};

use derives::TryIntoValue;

#[cfg(test)]
mod derive_try_into_value_test {
    use crate::cpu::{instruction::addressing_mode::TryIntoAddress, Cpu, CpuError};

    #[test]
    fn derive_try_into_value() {
        #[derive(derives::TryIntoValue)]
        struct AddressingMode {
            pub address: u8,
        }

        impl TryIntoAddress for AddressingMode {
            fn try_into_address(&self, _cpu: &Cpu) -> Result<u16, CpuError> {
                Ok(self.address as u16)
            }
        }
    }
}

pub struct Address {
    address: u16,
}

#[derive(Debug)]
pub struct Implicit {}

#[derive(Debug)]
pub struct Accumulator {}

#[derive(Debug)]
pub struct Immediate {
    pub immediate: u8,
}

impl Immediate {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<Immediate, CpuMemoryError> {
        let immediate = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(Immediate { immediate })
    }
}

impl TryIntoValue for Immediate {
    fn try_into_value(&self, _cpu: &Cpu) -> Result<u8, CpuError> {
        Ok(self.immediate)
    }
}

#[derive(Debug, TryIntoValue)]
pub struct ZeroPage {
    pub address: u8,
}

impl ZeroPage {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<ZeroPage, CpuMemoryError> {
        let address = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(ZeroPage { address })
    }
}

#[derive(Debug, TryIntoValue)]
pub struct ZeroPageX {
    pub address: u8,
}

impl ZeroPageX {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<ZeroPageX, CpuMemoryError> {
        let address: u8 = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(ZeroPageX { address })
    }
}

#[derive(Debug, TryIntoValue)]
pub struct ZeroPageY {
    pub address: u8,
}

impl ZeroPageY {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<ZeroPageY, CpuMemoryError> {
        let address: u8 = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(ZeroPageY { address })
    }
}

#[derive(Debug, TryIntoValue)]
pub struct Relative {
    pub offset: i8,
}

impl Relative {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<Relative, CpuMemoryError> {
        let offset = memory.read(*program_counter)? as i8;
        *program_counter += 1;
        Ok(Relative { offset })
    }
}

#[derive(Debug, TryIntoValue)]
pub struct Absolute {
    pub address: u16,
}

impl Absolute {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<Absolute, CpuMemoryError> {
        let address = memory.read_u16(*program_counter)?;
        *program_counter += 2;
        Ok(Absolute { address })
    }
}

#[derive(Debug, TryIntoValue)]
pub struct AbsoluteX {
    pub address: u16,
}

impl AbsoluteX {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<AbsoluteX, CpuMemoryError> {
        let address = memory.read_u16(*program_counter)?;
        *program_counter += 2;
        Ok(AbsoluteX { address })
    }
}

#[derive(Debug, TryIntoValue)]
pub struct AbsoluteY {
    pub address: u16,
}

impl AbsoluteY {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<AbsoluteY, CpuMemoryError> {
        let address = memory.read_u16(*program_counter)?;
        *program_counter += 2;
        Ok(AbsoluteY { address })
    }
}

#[derive(Debug, TryIntoValue)]
pub struct IndirectX {
    pub address: u8,
}

impl IndirectX {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<IndirectX, CpuMemoryError> {
        let address = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(IndirectX { address })
    }
}

#[derive(Debug, TryIntoValue)]
pub struct IndirectY {
    pub address: u8,
}

impl IndirectY {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<IndirectY, CpuMemoryError> {
        let address = memory.read(*program_counter)?;
        *program_counter += 1;
        Ok(IndirectY { address })
    }
}

pub trait TryIntoAddress {
    fn try_into_address(&self, cpu: &Cpu) -> Result<u16, CpuError>;
}

pub trait TryIntoValue {
    fn try_into_value(&self, cpu: &Cpu) -> Result<u8, CpuError>;
}

impl TryIntoAddress for ZeroPage {
    fn try_into_address(&self, _cpu: &Cpu) -> Result<u16, CpuError> {
        Ok(self.address as u16)
    }
}

impl TryIntoAddress for ZeroPageX {
    fn try_into_address(&self, cpu: &Cpu) -> Result<u16, CpuError> {
        let address = self.address.wrapping_add(cpu.register_x);
        Ok(address as u16)
    }
}

impl TryIntoAddress for ZeroPageY {
    fn try_into_address(&self, cpu: &Cpu) -> Result<u16, CpuError> {
        let address = self.address.wrapping_add(cpu.register_y);
        Ok(address as u16)
    }
}

impl TryIntoAddress for Relative {
    fn try_into_address(&self, cpu: &Cpu) -> Result<u16, CpuError> {
        Ok(cpu.program_counter.wrapping_add(self.offset as u16))
    }
}

impl TryIntoAddress for Absolute {
    fn try_into_address(&self, _cpu: &Cpu) -> Result<u16, CpuError> {
        Ok(self.address)
    }
}

impl TryIntoAddress for AbsoluteX {
    fn try_into_address(&self, cpu: &Cpu) -> Result<u16, CpuError> {
        let address = self.address.wrapping_add(cpu.register_x as u16);
        Ok(address)
    }
}

impl TryIntoAddress for AbsoluteY {
    fn try_into_address(&self, cpu: &Cpu) -> Result<u16, CpuError> {
        let address = self.address.wrapping_add(cpu.register_y as u16);
        Ok(address)
    }
}

impl TryIntoAddress for IndirectX {
    fn try_into_address(&self, cpu: &Cpu) -> Result<u16, CpuError> {
        let base = self.address.wrapping_add(cpu.register_x);
        let lo = cpu.memory.read(base as u16)?;
        let hi = cpu.memory.read(base.wrapping_add(1) as u16)?;
        let address = (hi as u16) << 8 | lo as u16;
        Ok(address)
    }
}

impl TryIntoAddress for IndirectY {
    fn try_into_address(&self, cpu: &Cpu) -> Result<u16, CpuError> {
        let lo = cpu.memory.read(self.address as u16)?;
        let hi = cpu.memory.read(self.address.wrapping_add(1) as u16)?;
        let address = (hi as u16) << 8 | lo as u16;
        let address = address.wrapping_add(cpu.register_y as u16);
        Ok(address)
    }
}

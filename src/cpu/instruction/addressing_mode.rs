use crate::cpu::{
    memory::{CpuMemoryError, Memory},
    Cpu,
};

pub struct Address {
    address: u16,
}

pub trait IntoAddress {
    fn into_address(&self, cpu: &Cpu) -> u16;
}

pub trait IntoValue {
    fn into_value(&self, cpu: &Cpu) -> u8;
}

impl<T: IntoAddress> IntoValue for T {
    fn into_value(&self, cpu: &Cpu) -> u8 {
        let address = self.into_address(cpu);
        let result = cpu.memory.read(address);
        result
    }
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
        let immediate = memory.read(*program_counter);
        *program_counter += 1;
        Ok(Immediate { immediate })
    }
}

impl IntoValue for Immediate {
    fn into_value(&self, _cpu: &Cpu) -> u8 {
        self.immediate
    }
}

#[derive(Debug)]
pub struct ZeroPage {
    pub address: u8,
}

impl ZeroPage {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<ZeroPage, CpuMemoryError> {
        let address = memory.read(*program_counter);
        *program_counter += 1;
        Ok(ZeroPage { address })
    }
}

#[derive(Debug)]
pub struct ZeroPageX {
    pub address: u8,
}

impl ZeroPageX {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<ZeroPageX, CpuMemoryError> {
        let address: u8 = memory.read(*program_counter);
        *program_counter += 1;
        Ok(ZeroPageX { address })
    }
}

#[derive(Debug)]
pub struct ZeroPageY {
    pub address: u8,
}

impl ZeroPageY {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<ZeroPageY, CpuMemoryError> {
        let address: u8 = memory.read(*program_counter);
        *program_counter += 1;
        Ok(ZeroPageY { address })
    }
}

#[derive(Debug)]
pub struct Relative {
    pub offset: i8,
}

impl Relative {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<Relative, CpuMemoryError> {
        let offset = memory.read(*program_counter) as i8;
        *program_counter += 1;
        Ok(Relative { offset })
    }
}

#[derive(Debug)]
pub struct Absolute {
    pub address: u16,
}

impl Absolute {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<Absolute, CpuMemoryError> {
        let address = memory.read_u16(*program_counter);
        *program_counter += 2;
        Ok(Absolute { address })
    }
}

#[derive(Debug)]
pub struct AbsoluteX {
    pub address: u16,
}

impl AbsoluteX {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<AbsoluteX, CpuMemoryError> {
        let address = memory.read_u16(*program_counter);
        *program_counter += 2;
        Ok(AbsoluteX { address })
    }
}

#[derive(Debug)]
pub struct AbsoluteY {
    pub address: u16,
}

impl AbsoluteY {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<AbsoluteY, CpuMemoryError> {
        let address = memory.read_u16(*program_counter);
        *program_counter += 2;
        Ok(AbsoluteY { address })
    }
}

#[derive(Debug)]
pub struct IndirectX {
    pub address: u8,
}

impl IndirectX {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<IndirectX, CpuMemoryError> {
        let address = memory.read(*program_counter);
        *program_counter += 1;
        Ok(IndirectX { address })
    }
}

#[derive(Debug)]
pub struct IndirectY {
    pub address: u8,
}

impl IndirectY {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Result<IndirectY, CpuMemoryError> {
        let address = memory.read(*program_counter);
        *program_counter += 1;
        Ok(IndirectY { address })
    }
}

impl IntoAddress for ZeroPage {
    fn into_address(&self, _cpu: &Cpu) -> u16 {
        self.address as u16
    }
}

impl IntoAddress for ZeroPageX {
    fn into_address(&self, cpu: &Cpu) -> u16 {
        let address = self.address.wrapping_add(cpu.register_x);
        address as u16
    }
}

impl IntoAddress for ZeroPageY {
    fn into_address(&self, cpu: &Cpu) -> u16 {
        let address = self.address.wrapping_add(cpu.register_y);
        address as u16
    }
}

impl IntoAddress for Relative {
    fn into_address(&self, cpu: &Cpu) -> u16 {
        cpu.program_counter.wrapping_add(self.offset as u16)
    }
}

impl IntoAddress for Absolute {
    fn into_address(&self, _cpu: &Cpu) -> u16 {
        self.address
    }
}

impl IntoAddress for AbsoluteX {
    fn into_address(&self, cpu: &Cpu) -> u16 {
        let address = self.address.wrapping_add(cpu.register_x as u16);
        address
    }
}

impl IntoAddress for AbsoluteY {
    fn into_address(&self, cpu: &Cpu) -> u16 {
        let address = self.address.wrapping_add(cpu.register_y as u16);
        address
    }
}

impl IntoAddress for IndirectX {
    fn into_address(&self, cpu: &Cpu) -> u16 {
        let base = self.address.wrapping_add(cpu.register_x);
        let lo = cpu.memory.read(base as u16);
        let hi = cpu.memory.read(base.wrapping_add(1) as u16);
        let address = (hi as u16) << 8 | lo as u16;
        address
    }
}

impl IntoAddress for IndirectY {
    fn into_address(&self, cpu: &Cpu) -> u16 {
        let lo = cpu.memory.read(self.address as u16);
        let hi = cpu.memory.read(self.address.wrapping_add(1) as u16);
        let address = (hi as u16) << 8 | lo as u16;
        let address = address.wrapping_add(cpu.register_y as u16);
        address
    }
}

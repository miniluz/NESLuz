use crate::cpu::{memory::Memory, Cpu};

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

#[derive(Debug, Copy, Clone)]
pub struct Implicit {}

#[derive(Debug, Copy, Clone)]
pub struct Accumulator {}

#[derive(Debug, Copy, Clone)]
pub struct Immediate {
    pub immediate: u8,
}

impl Immediate {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Immediate {
        let immediate = memory.read(*program_counter);
        *program_counter += 1;
        Immediate { immediate }
    }
}

impl IntoValue for Immediate {
    fn into_value(&self, _cpu: &Cpu) -> u8 {
        self.immediate
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ZeroPage {
    pub address: u8,
}

impl ZeroPage {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> ZeroPage {
        let address = memory.read(*program_counter);
        *program_counter += 1;
        ZeroPage { address }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ZeroPageX {
    pub address: u8,
}

impl ZeroPageX {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> ZeroPageX {
        let address: u8 = memory.read(*program_counter);
        *program_counter += 1;
        ZeroPageX { address }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ZeroPageY {
    pub address: u8,
}

impl ZeroPageY {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> ZeroPageY {
        let address: u8 = memory.read(*program_counter);
        *program_counter += 1;
        ZeroPageY { address }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Relative {
    pub offset: i8,
}

impl Relative {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Relative {
        let offset = memory.read(*program_counter) as i8;
        *program_counter += 1;
        Relative { offset }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Absolute {
    pub address: u16,
}

impl Absolute {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Absolute {
        let address = memory.read_u16(*program_counter);
        *program_counter += 2;
        Absolute { address }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AbsoluteX {
    pub address: u16,
}

impl AbsoluteX {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> AbsoluteX {
        let address = memory.read_u16(*program_counter);
        *program_counter += 2;
        AbsoluteX { address }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AbsoluteY {
    pub address: u16,
}

impl AbsoluteY {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> AbsoluteY {
        let address = memory.read_u16(*program_counter);
        *program_counter += 2;
        AbsoluteY { address }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Indirect {
    pub address: u8,
}

impl Indirect {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> Indirect {
        let address = memory.read(*program_counter);
        *program_counter += 1;
        Indirect { address }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IndirectX {
    pub address: u8,
}

impl IndirectX {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> IndirectX {
        let address = memory.read(*program_counter);
        *program_counter += 1;
        IndirectX { address }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IndirectY {
    pub address: u8,
}

impl IndirectY {
    pub fn new(memory: &Memory, program_counter: &mut u16) -> IndirectY {
        let address = memory.read(*program_counter);
        *program_counter += 1;
        IndirectY { address }
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

impl IntoAddress for Indirect {
    fn into_address(&self, cpu: &Cpu) -> u16 {
        let base = self.address;
        let lo = cpu.memory.read(base as u16);
        let hi = cpu.memory.read(base.wrapping_add(1) as u16);
        let address = (hi as u16) << 8 | lo as u16;
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

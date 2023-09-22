mod instruction;
mod memory;
mod status;

#[cfg(test)]
mod test;

use instruction::*;

use crate::cpu::status::Flags;

use self::{memory::Memory, status::Status};

#[derive(Debug)]
pub struct Cpu {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub status: Status,
    pub program_counter: u16,
    memory: Memory,
}

#[derive(Debug)]
pub enum Register {
    X,
    Y,
    A,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register_a: 0x0,
            register_x: 0x0,
            register_y: 0x0,
            stack_pointer: 0xff,
            status: Status::new(),
            program_counter: 0x0,
            memory: Memory::new(),
        }
    }

    pub fn set_register(&mut self, destination: &Register, value: u8) {
        match destination {
            Register::X => self.register_x = value,
            Register::Y => self.register_y = value,
            Register::A => self.register_a = value,
        }
    }

    pub fn get_register(&self, origin: &Register) -> u8 {
        match origin {
            Register::X => self.register_x,
            Register::Y => self.register_y,
            Register::A => self.register_a,
        }
    }
}

impl Cpu {
    fn set_zero_and_negative(&mut self, register_value: u8) {
        self.status.set(Flags::Zero, register_value == 0);
        self.status.set(Flags::Negative, (register_value as i8) < 0);
    }

    pub fn reset(&mut self) -> color_eyre::Result<()> {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = 0xff;
        self.status = Status::new();

        self.program_counter = self.memory.read_u16(0xFFFC)?;
        Ok(())
    }

    pub fn load_and_run(&mut self, program: &[u8]) -> color_eyre::Result<()> {
        self.load(program)?;
        self.reset()?;
        self.run()?;

        Ok(())
    }

    #[cfg(test)]
    pub fn load_and_run_test(&mut self, program: &[u8]) -> color_eyre::Result<()> {
        self.load(program)?;
        self.reset()?;
        self.program_counter = 0x8000;
        self.run()?;

        Ok(())
    }

    pub fn load(&mut self, program: &[u8]) -> color_eyre::eyre::Result<()> {
        self.memory.load(0x8000, &program)?;

        Ok(())
    }
}

impl Cpu {
    pub fn read_address(&mut self, addressing_mode: AddressingMode) -> color_eyre::Result<u8> {
        use AddressingMode::*;
        match addressing_mode {
            Immediate { immediate } => Ok(immediate),
            ZeroPage { address } => Ok(self.memory.read(address as u16)?),
            ZeroPageX { address } => {
                let address = address.wrapping_add(self.register_x);
                Ok(self.memory.read(address as u16)?)
            }
            ZeroPageY { address } => {
                let address = address.wrapping_add(self.register_y);
                Ok(self.memory.read(address as u16)?)
            }
            Absolute { address } => Ok(self.memory.read(address)?),
            AbsoluteX { address } => {
                let address = address.wrapping_add(self.register_x as u16);
                Ok(self.memory.read(address)?)
            }
            AbsoluteY { address } => {
                let address = address.wrapping_add(self.register_y as u16);
                Ok(self.memory.read(address)?)
            }
            IndirectX { address } => {
                let base = address.wrapping_add(self.register_x);
                let lo = self.memory.read(base as u16)?;
                let hi = self.memory.read(base.wrapping_add(1) as u16)?;
                let address = (hi as u16) << 8 | lo as u16;
                Ok(self.memory.read(address)?)
            }
            IndirectY { address } => {
                let lo = self.memory.read(address as u16)?;
                let hi = self.memory.read(address.wrapping_add(1) as u16)?;
                let address = (hi as u16) << 8 | lo as u16;
                let address = address.wrapping_add(self.register_y as u16);
                Ok(self.memory.read(address)?)
            }
        }
    }

    pub fn run(&mut self) -> color_eyre::Result<()> {
        loop {
            let (instruction, program_counter) =
                Instruction::get_instruction(&self.memory, &self.program_counter)?;
            self.program_counter = program_counter;

            use Instruction::*;

            match instruction {
                Break => break,
                Adc { addressing_mode } => {
                    let value = self.read_address(addressing_mode)?;
                    let carry = self.status.get(Flags::Carry) as u8;

                    let carry_flag = {
                        let (value, first_carry) = self.register_a.overflowing_add(value);
                        let (_, second_carry) = value.overflowing_add(carry);
                        first_carry || second_carry
                    };

                    let (value, overflow_flag) = {
                        let (value, first_overflow) =
                            (self.register_a as i8).overflowing_add(value as i8);
                        let (value, second_overflow) = value.overflowing_add(carry as i8);
                        (value as u8, first_overflow || second_overflow)
                    };

                    self.register_a = value;

                    self.status.set(Flags::Overflow, overflow_flag);
                    self.status.set(Flags::Carry, carry_flag);
                    self.set_zero_and_negative(value);
                }
                And { addressing_mode } => {
                    let value = self.read_address(addressing_mode)?;
                    let value = self.register_a & value;
                    self.register_a = value;
                    self.set_zero_and_negative(value);
                }
                Ld {
                    destination,
                    addressing_mode,
                } => {
                    let value = self.read_address(addressing_mode)?;
                    self.set_register(&destination, value);
                    self.set_zero_and_negative(value);
                }
                Trr {
                    origin,
                    destination,
                } => {
                    let value = self.get_register(&origin);
                    self.set_register(&destination, value);
                    self.set_zero_and_negative(value);
                }
                In { destination } => {
                    let value = self.get_register(&destination);
                    let (value, overflow) = value.overflowing_add(1);
                    self.set_register(&destination, value);
                    self.status.set(Flags::Overflow, overflow);
                    self.set_zero_and_negative(value);
                }
            }
        }

        Ok(())
    }
}

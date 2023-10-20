pub mod instruction;
pub mod memory;
pub mod status;

#[cfg(test)]
mod macro_test;

use instruction::*;
use thiserror::Error;

use crate::cpu::{
    instruction::addressing_mode::{IntoAddress, IntoValue},
    status::Flag,
};

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
        self.status.set(Flag::Zero, register_value == 0);
        self.status.set(Flag::Negative, (register_value as i8) < 0);
    }

    pub fn reset(&mut self) -> Result<(), CpuError> {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = 0xff;
        self.status = Status::new();

        self.program_counter = self.memory.read_u16(0xFFFC);
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

    pub fn load(&mut self, program: &[u8]) -> Result<(), CpuError> {
        self.memory.load(0x8000, &program)?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum CpuError {
    #[error(transparent)]
    CpuMemoryError(#[from] memory::CpuMemoryError),
}

impl Cpu {
    pub fn run(&mut self) -> color_eyre::Result<()> {
        loop {
            let (instruction, program_counter) =
                Instruction::get_instruction(&self.memory, &self.program_counter)?;
            self.program_counter = program_counter;

            use Instruction::*;

            match instruction {
                Adc { addressing_mode } => {
                    let value = addressing_mode.into_value(self);
                    let carry = self.status.get(Flag::Carry) as u8;

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

                    self.status.set(Flag::Overflow, overflow_flag);
                    self.status.set(Flag::Carry, carry_flag);
                    self.set_zero_and_negative(value);
                }
                And { addressing_mode } => {
                    let value = addressing_mode.into_value(self);
                    let value = self.register_a & value;
                    self.register_a = value;
                    self.set_zero_and_negative(value);
                }
                Asl { addressing_mode } => {
                    let value = match addressing_mode {
                        AslAddressingMode::Accumulator { mode: _ } => self.register_a,
                        AslAddressingMode::AslAddressAddressingMode { mode } => {
                            mode.into_value(self)
                        }
                    };

                    self.status.set(Flag::Carry, (value as i8) < 0);
                    let value = value.wrapping_shl(1);
                    self.set_zero_and_negative(value);

                    match addressing_mode {
                        AslAddressingMode::Accumulator { mode: _ } => {
                            self.register_a = value;
                        }
                        AslAddressingMode::AslAddressAddressingMode { mode } => {
                            let address = mode.into_address(self);
                            self.memory.write(address, value);
                        }
                    };
                }
                Branch {
                    addressing_mode,
                    flag,
                    branch_if,
                } => {
                    if self.status.get(flag) == branch_if {
                        let new_address = addressing_mode.into_address(self);
                        self.program_counter = new_address;
                    }
                }
                Bit { addressing_mode } => {
                    let value = addressing_mode.into_value(self);
                    self.status.set(Flag::Negative, value & 0b1000_0000 != 0);
                    self.status.set(Flag::Overflow, value & 0b0100_0000 != 0);
                    self.status.set(Flag::Zero, self.register_a & value == 0);
                }
                Break => break,
                Clear { flag } => self.status.set(flag, false),
                Cmp {
                    register,
                    addressing_mode,
                } => {
                    let value = addressing_mode.into_value(self);
                    dbg!(value);
                    let register = self.get_register(&register);
                    let result = register.wrapping_sub(value);
                    self.set_zero_and_negative(result);
                    self.status.set(Flag::Carry, self.register_a >= value);
                }
                Ld {
                    destination,
                    addressing_mode,
                } => {
                    let value = addressing_mode.into_value(self);
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
                    self.status.set(Flag::Overflow, overflow);
                    self.set_zero_and_negative(value);
                }
            }
        }

        Ok(())
    }
}

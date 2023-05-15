mod instruction;
mod status;

#[cfg(test)]
mod test {
    use super::*;

    fn run_on_cpu(instructions: Vec<u8>) -> color_eyre::Result<Cpu> {
        let mut cpu = Cpu::new();
        cpu.interpret(instructions)?;
        Ok(cpu)
    }

    #[test]
    fn lda() {
        assert!(matches!(
            Instruction::get_instruction(&[LDA_IMMEDIATE, 0xc0, 0x00], &0).unwrap(),
            (
                Instruction::Ld {
                    destination: _,
                    ld_type: LdType::Immediate { immediate: 0xc0 }
                },
                2
            )
        ));

        let cpu = run_on_cpu(vec![LDA_IMMEDIATE, 0xf0, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0xf0);
        assert!(cpu.status.get(Flags::Negative));
        assert!(!cpu.status.get(Flags::Zero));

        let cpu = run_on_cpu(vec![LDA_IMMEDIATE, 0x00, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0x00);
        assert!(!cpu.status.get(Flags::Negative));
        assert!(cpu.status.get(Flags::Zero));
    }

    #[test]
    fn ldx() {
        assert!(matches!(
            Instruction::get_instruction(&[LDX_IMMEDIATE, 0xc0, 0x00], &0).unwrap(),
            (
                Instruction::Ld {
                    destination: _,
                    ld_type: LdType::Immediate { immediate: 0xc0 }
                },
                2
            )
        ));

        let cpu = run_on_cpu(vec![LDX_IMMEDIATE, 0xf0, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 0xf0);
        assert!(cpu.status.get(Flags::Negative));
        assert!(!cpu.status.get(Flags::Zero));

        let cpu = run_on_cpu(vec![LDX_IMMEDIATE, 0x00, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 0x00);
        assert!(!cpu.status.get(Flags::Negative));
        assert!(cpu.status.get(Flags::Zero));
    }

    #[test]
    fn tax() {
        assert!(matches!(
            Instruction::get_instruction(&[TAX, 0x00], &0).unwrap(),
            (
                Instruction::Trr {
                    origin: _,
                    destination: _,
                },
                1
            )
        ));

        let cpu = run_on_cpu(vec![LDA_IMMEDIATE, 0xf0, TAX, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 0xf0);
        assert!(cpu.status.get(Flags::Negative));
        assert!(!cpu.status.get(Flags::Zero));

        let cpu = run_on_cpu(vec![TAX, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 0x00);
        assert!(!cpu.status.get(Flags::Negative));
        assert!(cpu.status.get(Flags::Zero));
    }

    #[test]
    fn inx() {
        assert!(matches!(
            Instruction::get_instruction(&[INX, 0x00], &0).unwrap(),
            (Instruction::In { destination: _ }, 1)
        ));

        let cpu = run_on_cpu(vec![LDX_IMMEDIATE, 0xf0, INX, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 0xf1);
        assert!(cpu.status.get(Flags::Negative));
        assert!(!cpu.status.get(Flags::Zero));

        let cpu = run_on_cpu(vec![LDX_IMMEDIATE, 0xff, INX, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 0x00);
        assert!(!cpu.status.get(Flags::Negative));
        assert!(cpu.status.get(Flags::Zero));
    }
}

use instruction::*;

use crate::cpu::status::Flags;

use self::status::Status;

#[derive(Debug)]
pub struct Cpu {
    pub register_a: u8,
    pub register_x: u8,
    pub status: Status,
    pub program_counter: u16,
}

type RegisterMutable = fn(&mut Cpu) -> &mut u8;
type RegisterImmutable = fn(&Cpu) -> u8;

impl Cpu {
    // new, getters

    pub fn new() -> Self {
        Cpu {
            register_a: 0x0,
            register_x: 0x0,
            status: Status::new(),
            program_counter: 0x0,
        }
    }

    pub fn register_a_mut(&mut self) -> &mut u8 {
        &mut self.register_a
    }

    pub fn register_a(&self) -> u8 {
        self.register_a
    }

    pub fn register_x_mut(&mut self) -> &mut u8 {
        &mut self.register_x
    }

    pub fn register_x(&self) -> u8 {
        self.register_x
    }
}

impl Cpu {
    fn set_zero_and_negative(&mut self, register_value: u8) {
        self.status.set(Flags::Zero, register_value == 0);
        self.status.set(Flags::Negative, (register_value as i8) < 0);
    }

    pub fn interpret(&mut self, program: Vec<u8>) -> color_eyre::Result<()> {
        self.program_counter = 0;

        loop {
            let (instruction, offset) =
                Instruction::get_instruction(&program, &self.program_counter)?;
            self.program_counter += offset;

            use Instruction::*;

            match instruction {
                Break => break,
                Ld {
                    destination,
                    ld_type,
                } => {
                    match ld_type {
                        LdType::Immediate { immediate } => {
                            *destination(self) = immediate;
                        }
                    }
                    let register_value = *destination(self);
                    self.set_zero_and_negative(register_value);
                }
                Trr {
                    origin,
                    destination,
                } => {
                    *destination(self) = origin(self);
                    let register_value = *destination(self);
                    self.set_zero_and_negative(register_value)
                }
                In { destination } => {
                    let (result, _overflow) = u8::overflowing_add(*destination(self), 1);
                    *destination(self) = result;
                    let register_value: u8 = *destination(self);
                    self.set_zero_and_negative(register_value);
                }
            }
        }

        Ok(())
    }
}

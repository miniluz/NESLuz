use super::addressing_mode as AM;
use super::*;
use crate::cpu::status::Flag;
use crate::cpu::Cpu;

fn get_instruction(instructions: &[u8]) -> color_eyre::Result<(Instruction, u16)> {
    let mut memory = Memory::new();
    memory.load(0x8000, instructions)?;
    let instruction = Instruction::get_instruction(&memory, &0x8000)?;
    Ok(instruction)
}

#[test]
fn clc() {
    use super::opcodes::CLC;

    assert!(matches!(
        get_instruction(&[CLC]).unwrap(),
        (Instruction::Clear { flag: Flag::Carry }, 0x8001)
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[CLC, 0x00]).unwrap();
    assert!(!cpu.status.get(Flag::Carry));
    assert!(!cpu.status.get(Flag::Decimal));
    assert!(!cpu.status.get(Flag::InterruptDisable));
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load(&[CLC, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, true);
    cpu.status.set(Flag::Decimal, true);
    cpu.status.set(Flag::InterruptDisable, true);
    cpu.status.set(Flag::Negative, true);
    cpu.status.set(Flag::Overflow, true);
    cpu.status.set(Flag::Zero, true);
    cpu.run().unwrap();
    assert!(!cpu.status.get(Flag::Carry));
    assert!(cpu.status.get(Flag::Decimal));
    assert!(cpu.status.get(Flag::InterruptDisable));
    assert!(cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn cld() {
    use super::opcodes::CLD;

    assert!(matches!(
        get_instruction(&[CLD]).unwrap(),
        (
            Instruction::Clear {
                flag: Flag::Decimal
            },
            0x8001
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[CLD, 0x00]).unwrap();
    assert!(!cpu.status.get(Flag::Carry));
    assert!(!cpu.status.get(Flag::Decimal));
    assert!(!cpu.status.get(Flag::InterruptDisable));
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load(&[CLD, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, true);
    cpu.status.set(Flag::Decimal, true);
    cpu.status.set(Flag::InterruptDisable, true);
    cpu.status.set(Flag::Negative, true);
    cpu.status.set(Flag::Overflow, true);
    cpu.status.set(Flag::Zero, true);
    cpu.run().unwrap();
    assert!(cpu.status.get(Flag::Carry));
    assert!(!cpu.status.get(Flag::Decimal));
    assert!(cpu.status.get(Flag::InterruptDisable));
    assert!(cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn cli() {
    use super::opcodes::CLI;

    assert!(matches!(
        get_instruction(&[CLI]).unwrap(),
        (
            Instruction::Clear {
                flag: Flag::InterruptDisable
            },
            0x8001
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[CLI, 0x00]).unwrap();
    assert!(!cpu.status.get(Flag::Carry));
    assert!(!cpu.status.get(Flag::Decimal));
    assert!(!cpu.status.get(Flag::InterruptDisable));
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load(&[CLI, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, true);
    cpu.status.set(Flag::Decimal, true);
    cpu.status.set(Flag::InterruptDisable, true);
    cpu.status.set(Flag::Negative, true);
    cpu.status.set(Flag::Overflow, true);
    cpu.status.set(Flag::Zero, true);
    cpu.run().unwrap();
    assert!(cpu.status.get(Flag::Carry));
    assert!(cpu.status.get(Flag::Decimal));
    assert!(!cpu.status.get(Flag::InterruptDisable));
    assert!(cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn clv() {
    use super::opcodes::CLV;

    assert!(matches!(
        get_instruction(&[CLV]).unwrap(),
        (
            Instruction::Clear {
                flag: Flag::Overflow
            },
            0x8001
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[CLV, 0x00]).unwrap();
    assert!(!cpu.status.get(Flag::Carry));
    assert!(!cpu.status.get(Flag::Decimal));
    assert!(!cpu.status.get(Flag::InterruptDisable));
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load(&[CLV, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, true);
    cpu.status.set(Flag::Decimal, true);
    cpu.status.set(Flag::InterruptDisable, true);
    cpu.status.set(Flag::Negative, true);
    cpu.status.set(Flag::Overflow, true);
    cpu.status.set(Flag::Zero, true);
    cpu.run().unwrap();
    assert!(cpu.status.get(Flag::Carry));
    assert!(cpu.status.get(Flag::Decimal));
    assert!(cpu.status.get(Flag::InterruptDisable));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn asl_accumulator() {
    use super::opcodes::{ASL_ACCUMULATOR, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[ASL_ACCUMULATOR]).unwrap(),
        (
            Instruction::Asl {
                addressing_mode: AslAddressingMode::Accumulator {
                    mode: AM::Accumulator {}
                }
            },
            0x8001
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b10000000, ASL_ACCUMULATOR, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0b00000000);
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Carry));
    assert!(!cpu.status.get(Flag::Negative));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b0101_0101, ASL_ACCUMULATOR, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0b1010_1010);
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Carry));
}

#[test]
fn adc_immediate() {
    use super::opcodes::{ADC_IMMEDIATE, LDA_IMMEDIATE};
    assert!(matches!(
        get_instruction(&[ADC_IMMEDIATE, 0xc0]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AdcAddressingMode::Immediate {
                    mode: AM::Immediate { immediate: 0xc0 }
                }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[ADC_IMMEDIATE, 0xf0, 0x00]).unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0x71, ADC_IMMEDIATE, 0x72, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0x80, ADC_IMMEDIATE, 0x80, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Carry));
}

#[test]
fn and_immediate() {
    use super::opcodes::{AND_IMMEDIATE, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[AND_IMMEDIATE, 0xc0]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AndAddressingMode::Immediate {
                    mode: AM::Immediate { immediate: 0xc0 }
                }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[AND_IMMEDIATE, 0b11010011, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0b00000000);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b11010011, AND_IMMEDIATE, 0b10110001, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0b10010001);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b00110111, AND_IMMEDIATE, 0b00110111, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0b00110111);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
}

#[test]
fn lda_immediate() {
    use super::opcodes::LDA_IMMEDIATE;

    assert!(matches!(
        get_instruction(&[LDA_IMMEDIATE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: LdAddressingMode::Immediate {
                    mode: AM::Immediate { immediate: 0xc0 }
                }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0xf0, 0x00]).unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0x00, 0x00]).unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn ldx_immediate() {
    use super::opcodes::LDX_IMMEDIATE;

    assert!(matches!(
        get_instruction(&[LDX_IMMEDIATE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::X,
                addressing_mode: LdAddressingMode::Immediate {
                    mode: AM::Immediate { immediate: 0xc0 }
                }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_IMMEDIATE, 0xf0, 0x00]).unwrap();
    assert_eq!(cpu.register_x, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_IMMEDIATE, 0x00, 0x00]).unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn ldy_immediate() {
    use super::opcodes::LDY_IMMEDIATE;

    assert!(matches!(
        get_instruction(&[LDY_IMMEDIATE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::Y,
                addressing_mode: LdAddressingMode::Immediate {
                    mode: AM::Immediate { immediate: 0xc0 }
                }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDY_IMMEDIATE, 0xf0, 0x00]).unwrap();
    assert_eq!(cpu.register_y, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDY_IMMEDIATE, 0x00, 0x00]).unwrap();
    assert_eq!(cpu.register_y, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn adc_zero_page() {
    use super::opcodes::ADC_ZERO_PAGE;
    use super::opcodes::LDA_IMMEDIATE;

    assert!(matches!(
        get_instruction(&[ADC_ZERO_PAGE, 0xc0]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage { address: 0xc0 }
                    }
                }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ADC_ZERO_PAGE, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x00, &[0x01, 0x02, 0xf0, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_ZERO_PAGE, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x72, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_ZERO_PAGE, 0x03, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x80]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Carry));
}

#[test]
fn and_zero_page() {
    use super::opcodes::{AND_ZERO_PAGE, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[AND_ZERO_PAGE, 0xc0]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage { address: 0xc0 }
                    }
                }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[AND_ZERO_PAGE, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x00, &[0x01, 0x02, 0xf0, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00000000);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0b11010011, AND_ZERO_PAGE, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x00, &[0x01, 0x02, 0b10110001, 0x04])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b10010001);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b00110111, AND_ZERO_PAGE, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x00, &[0x01, 0x02, 0b00110111, 0x04])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00110111);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
}

#[test]
fn asl_zero_page() {
    use super::opcodes::ASL_ZERO_PAGE;

    assert!(matches!(
        get_instruction(&[ASL_ZERO_PAGE, 0xab]).unwrap(),
        (
            Instruction::Asl {
                addressing_mode: AslAddressingMode::AslAddressAddressingMode {
                    mode: AslAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage { address: 0xab }
                    }
                }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ASL_ZERO_PAGE, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x00, &[0x01, 0x02, 0b10000000, 0x04])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.memory.read(0x02), 0b00000000);
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Carry));
    assert!(!cpu.status.get(Flag::Negative));

    let mut cpu = Cpu::new();
    cpu.load(&[ASL_ZERO_PAGE, 0x03, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x00, &[0x01, 0x02, 0x03, 0b0101_0101])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.memory.read(0x03), 0b1010_1010);
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Carry));
}

#[test]
fn bit_zero_page() {
    use super::opcodes::BIT_ZERO_PAGE;

    assert!(matches!(
        get_instruction(&[BIT_ZERO_PAGE, 0xab]).unwrap(),
        (
            Instruction::Bit {
                addressing_mode: BitAddressingMode::ZeroPage {
                    mode: AM::ZeroPage { address: 0xab }
                }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[BIT_ZERO_PAGE, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x00, &[0x01, 0x02, 0b11000000, 0x04])
        .unwrap();
    cpu.register_a = 0b0011_1111;
    cpu.run().unwrap();
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Negative));

    let mut cpu = Cpu::new();
    cpu.load(&[BIT_ZERO_PAGE, 0x03, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x00, &[0x01, 0x02, 0x03, 0b0101_0101])
        .unwrap();
    cpu.register_a = 0b0001_0101;
    cpu.run().unwrap();
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Negative));

    let mut cpu = Cpu::new();
    cpu.load(&[BIT_ZERO_PAGE, 0x03, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x00, &[0x01, 0x02, 0x03, 0b1010_1010])
        .unwrap();
    cpu.register_a = 0b0101_0101;
    cpu.run().unwrap();
    assert!(cpu.status.get(Flag::Zero));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Negative));
}

#[test]
fn lda_zero_page() {
    use super::opcodes::LDA_ZERO_PAGE;

    assert!(matches!(
        get_instruction(&[LDA_ZERO_PAGE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage { address: 0xc0 }
                    }
                }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_ZERO_PAGE, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x00, &[0x01, 0x02, 0xf1, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf1);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_ZERO_PAGE, 0x02, 0x00]).unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn ldx_zero_page() {
    use super::opcodes::LDX_ZERO_PAGE;

    assert!(matches!(
        get_instruction(&[LDX_ZERO_PAGE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::X,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage { address: 0xc0 }
                    }
                }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDX_ZERO_PAGE, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x00, &[0x01, 0x02, 0xf1, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0xf1);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_ZERO_PAGE, 0x02, 0x00]).unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn ldy_zero_page() {
    use super::opcodes::LDY_ZERO_PAGE;

    assert!(matches!(
        get_instruction(&[LDY_ZERO_PAGE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::Y,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPage {
                        mode: AM::ZeroPage { address: 0xc0 }
                    }
                }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDY_ZERO_PAGE, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x00, &[0x01, 0x02, 0xf1, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_y, 0xf1);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDY_ZERO_PAGE, 0x02, 0x00]).unwrap();
    assert_eq!(cpu.register_y, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn adc_zero_page_x() {
    use super::opcodes::{ADC_ZERO_PAGE_X, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[ADC_ZERO_PAGE_X, 0xc0]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::ZeroPageX {
                        mode: AM::ZeroPageX { address: 0xc0 }
                    }
                }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ADC_ZERO_PAGE_X, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x02;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0xf0]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_ZERO_PAGE_X, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x72]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_ZERO_PAGE_X, 0x03, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x00;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x80]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Carry));
}

#[test]
fn and_zero_page_x() {
    use super::opcodes::{AND_ZERO_PAGE_X, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[AND_ZERO_PAGE_X, 0xc0]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::ZeroPageX {
                        mode: AM::ZeroPageX { address: 0xc0 }
                    }
                }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[AND_ZERO_PAGE_X, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0xf0, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00000000);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0b11010011, AND_ZERO_PAGE_X, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory
        .load(0x00, &[0x01, 0x02, 0x03, 0b10110001])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b10010001);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b00110111, AND_ZERO_PAGE_X, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x00;
    cpu.memory
        .load(0x00, &[0x01, 0x02, 0b00110111, 0x04])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00110111);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
}

#[test]
fn asl_zero_page_x() {
    use super::opcodes::ASL_ZERO_PAGE_X;

    assert!(matches!(
        get_instruction(&[ASL_ZERO_PAGE_X, 0xab]).unwrap(),
        (
            Instruction::Asl {
                addressing_mode: AslAddressingMode::AslAddressAddressingMode {
                    mode: AslAddressAddressingMode::ZeroPageX {
                        mode: AM::ZeroPageX { address: 0xab }
                    }
                }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ASL_ZERO_PAGE_X, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory
        .load(0x00, &[0x01, 0x02, 0b10000000, 0x04])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.memory.read(0x02), 0b00000000);
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Carry));
    assert!(!cpu.status.get(Flag::Negative));

    let mut cpu = Cpu::new();
    cpu.load(&[ASL_ZERO_PAGE_X, 0x03, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.register_x = 0x00;
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x00, &[0x01, 0x02, 0x03, 0b0101_0101])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.memory.read(0x03), 0b1010_1010);
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Carry));
}

#[test]
fn lda_zero_page_x() {
    use super::opcodes::LDA_ZERO_PAGE_X;

    assert!(matches!(
        get_instruction(&[LDA_ZERO_PAGE_X, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPageX {
                        mode: AM::ZeroPageX { address: 0xc0 }
                    }
                }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_ZERO_PAGE_X, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf4);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_ZERO_PAGE_X, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn ldy_zero_page_x() {
    use super::opcodes::LDY_ZERO_PAGE_X;

    assert!(matches!(
        get_instruction(&[LDY_ZERO_PAGE_X, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::Y,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPageX {
                        mode: AM::ZeroPageX { address: 0xc0 }
                    }
                }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDY_ZERO_PAGE_X, 0x04, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0xff;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_y, 0xf4);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDY_ZERO_PAGE_X, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_y, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn ldx_zero_page_y() {
    use super::opcodes::LDX_ZERO_PAGE_Y;

    assert!(matches!(
        get_instruction(&[LDX_ZERO_PAGE_Y, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::X,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::ZeroPageY {
                        mode: AM::ZeroPageY { address: 0xc0 }
                    }
                }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDX_ZERO_PAGE_Y, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0xf4);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_ZERO_PAGE_Y, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn bcc() {
    use super::opcodes::BCC;

    assert!(matches!(
        get_instruction(&[BCC, 0x0b]).unwrap(),
        (
            Instruction::Branch {
                addressing_mode: AM::Relative { offset: 0x0b },
                flag: Flag::Carry,
                branch_if: false,
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BCC, 0x00, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BCC, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x800b);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BCC, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BCC, 0xf8, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, false);
    cpu.run().unwrap();
    assert_eq!(
        cpu.program_counter,
        0x8003u16.wrapping_add(0xf8u8 as i8 as u16)
    );
}

#[test]
fn bcs() {
    use super::opcodes::BCS;

    assert!(matches!(
        get_instruction(&[BCS, 0x0b]).unwrap(),
        (
            Instruction::Branch {
                addressing_mode: AM::Relative { offset: 0x0b },
                flag: Flag::Carry,
                branch_if: true,
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BCS, 0x00, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BCS, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x800b);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BCS, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BCS, 0xf8, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Carry, true);
    cpu.run().unwrap();
    assert_eq!(
        cpu.program_counter,
        0x8003u16.wrapping_add(0xf8u8 as i8 as u16)
    );
}

#[test]
fn beq() {
    use super::opcodes::BEQ;

    assert!(matches!(
        get_instruction(&[BEQ, 0x0b]).unwrap(),
        (
            Instruction::Branch {
                addressing_mode: AM::Relative { offset: 0x0b },
                flag: Flag::Zero,
                branch_if: true,
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BEQ, 0x00, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Zero, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BEQ, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Zero, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x800b);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BEQ, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Zero, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BEQ, 0xf8, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Zero, true);
    cpu.run().unwrap();
    assert_eq!(
        cpu.program_counter,
        0x8003u16.wrapping_add(0xf8u8 as i8 as u16)
    );
}

#[test]
fn bmi() {
    use super::opcodes::BMI;

    assert!(matches!(
        get_instruction(&[BMI, 0x0b]).unwrap(),
        (
            Instruction::Branch {
                addressing_mode: AM::Relative { offset: 0x0b },
                flag: Flag::Negative,
                branch_if: true,
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BMI, 0x00, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Negative, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BMI, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Negative, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x800b);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BMI, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Negative, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BMI, 0xf8, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Negative, true);
    cpu.run().unwrap();
    assert_eq!(
        cpu.program_counter,
        0x8003u16.wrapping_add(0xf8u8 as i8 as u16)
    );
}

#[test]
fn bne() {
    use super::opcodes::BNE;

    assert!(matches!(
        get_instruction(&[BNE, 0x0b]).unwrap(),
        (
            Instruction::Branch {
                addressing_mode: AM::Relative { offset: 0x0b },
                flag: Flag::Zero,
                branch_if: false,
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BNE, 0x00, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Zero, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BNE, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Zero, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x800b);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BNE, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Zero, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BNE, 0xf8, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Zero, false);
    cpu.run().unwrap();
    assert_eq!(
        cpu.program_counter,
        0x8003u16.wrapping_add(0xf8u8 as i8 as u16)
    );
}

#[test]
fn bpl() {
    use super::opcodes::BPL;

    assert!(matches!(
        get_instruction(&[BPL, 0x0b]).unwrap(),
        (
            Instruction::Branch {
                addressing_mode: AM::Relative { offset: 0x0b },
                flag: Flag::Negative,
                branch_if: false,
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BPL, 0x00, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Negative, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BPL, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Negative, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x800b);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BPL, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Negative, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BPL, 0xf8, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Negative, false);
    cpu.run().unwrap();
    assert_eq!(
        cpu.program_counter,
        0x8003u16.wrapping_add(0xf8u8 as i8 as u16)
    );
}

#[test]
fn bvc() {
    use super::opcodes::BVC;

    assert!(matches!(
        get_instruction(&[BVC, 0x0b]).unwrap(),
        (
            Instruction::Branch {
                addressing_mode: AM::Relative { offset: 0x0b },
                flag: Flag::Overflow,
                branch_if: false,
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BVC, 0x00, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Overflow, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BVC, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Overflow, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x800b);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BVC, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Overflow, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BVC, 0xf8, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Overflow, false);
    cpu.run().unwrap();
    assert_eq!(
        cpu.program_counter,
        0x8003u16.wrapping_add(0xf8u8 as i8 as u16)
    );
}

#[test]
fn bvs() {
    use super::opcodes::BVS;

    assert!(matches!(
        get_instruction(&[BVS, 0x0b]).unwrap(),
        (
            Instruction::Branch {
                addressing_mode: AM::Relative { offset: 0x0b },
                flag: Flag::Overflow,
                branch_if: true,
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BVS, 0x00, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Overflow, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BVS, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Overflow, true);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x800b);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BVS, 0x08, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Overflow, false);
    cpu.run().unwrap();
    assert_eq!(cpu.program_counter, 0x8003);

    let mut cpu = Cpu::new();
    cpu.load_and_run(&[BVS, 0xf8, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.status.set(Flag::Overflow, true);
    cpu.run().unwrap();
    assert_eq!(
        cpu.program_counter,
        0x8003u16.wrapping_add(0xf8u8 as i8 as u16)
    );
}

#[test]
fn adc_absolute() {
    use super::opcodes::{ADC_ABSOLUTE, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[ADC_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::Absolute {
                        mode: AM::Absolute { address: 0xcdab },
                    },
                },
            },
            0x8003,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ADC_ABSOLUTE, 0x03, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf0]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_ABSOLUTE, 0x03, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0x72]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_ABSOLUTE, 0x02, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x80, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Carry));
}

#[test]
fn and_absolute() {
    use super::opcodes::{AND_ABSOLUTE, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[AND_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::Absolute {
                        mode: AM::Absolute { address: 0xcdab },
                    },
                },
            },
            0x8003
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[AND_ABSOLUTE, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x1000, &[0x01, 0x02, 0xf0, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00000000);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0b11010011, AND_ABSOLUTE, 0x03, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x0100, &[0x01, 0x02, 0x03, 0b10110001])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b10010001);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b00110111, AND_ABSOLUTE, 0x02, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x0100, &[0x01, 0x02, 0b00110111, 0x04])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00110111);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
}

#[test]
fn asl_absolute() {
    use super::opcodes::ASL_ABSOLUTE;

    assert!(matches!(
        get_instruction(&[ASL_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Asl {
                addressing_mode: AslAddressingMode::AslAddressAddressingMode {
                    mode: AslAddressAddressingMode::Absolute {
                        mode: AM::Absolute { address: 0xcdab },
                    },
                },
            },
            0x8003
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ASL_ABSOLUTE, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x0100, &[0x01, 0x02, 0b10000000, 0x04])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.memory.read(0x0102), 0b00000000);
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Carry));
    assert!(!cpu.status.get(Flag::Negative));

    let mut cpu = Cpu::new();
    cpu.load(&[ASL_ABSOLUTE, 0x03, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x0100, &[0x01, 0x02, 0x03, 0b0101_0101])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.memory.read(0x0103), 0b1010_1010);
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Carry));
}

#[test]
fn bit_absolute() {
    use super::opcodes::BIT_ABSOLUTE;

    assert!(matches!(
        get_instruction(&[BIT_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Bit {
                addressing_mode: BitAddressingMode::Absolute {
                    mode: AM::Absolute { address: 0xcdab }
                }
            },
            0x8003
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[BIT_ABSOLUTE, 0x02, 0x10, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x1000, &[0x01, 0x02, 0b11000000, 0x04])
        .unwrap();
    cpu.register_a = 0b0011_1111;
    cpu.run().unwrap();
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Negative));

    let mut cpu = Cpu::new();
    cpu.load(&[BIT_ABSOLUTE, 0x03, 0x10, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x1000, &[0x01, 0x02, 0x03, 0b0101_0101])
        .unwrap();
    cpu.register_a = 0b0001_0101;
    cpu.run().unwrap();
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Negative));

    let mut cpu = Cpu::new();
    cpu.load(&[BIT_ABSOLUTE, 0x03, 0x10, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x1000, &[0x01, 0x02, 0x03, 0b1010_1010])
        .unwrap();
    cpu.register_a = 0b0101_0101;
    cpu.run().unwrap();
    assert!(cpu.status.get(Flag::Zero));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Negative));
}

#[test]
fn lda_absolute() {
    use super::opcodes::LDA_ABSOLUTE;

    assert!(matches!(
        get_instruction(&[LDA_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::Absolute {
                        mode: AM::Absolute { address: 0xcdab },
                    },
                },
            },
            0x8003,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_ABSOLUTE, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0xf3, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf3);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_ABSOLUTE, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn ldx_absolute() {
    use super::opcodes::LDX_ABSOLUTE;

    assert!(matches!(
        get_instruction(&[LDX_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::X,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::Absolute {
                        mode: AM::Absolute { address: 0xcdab },
                    },
                },
            },
            0x8003,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDX_ABSOLUTE, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0xf3, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0xf3);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_ABSOLUTE, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn ldy_absolute() {
    use super::opcodes::LDY_ABSOLUTE;

    assert!(matches!(
        get_instruction(&[LDY_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::Y,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::Absolute {
                        mode: AM::Absolute { address: 0xcdab },
                    },
                },
            },
            0x8003,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDY_ABSOLUTE, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0xf3, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_y, 0xf3);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDY_ABSOLUTE, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_y, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn adc_absolute_x() {
    use super::opcodes::{ADC_ABSOLUTE_X, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[ADC_ABSOLUTE_X, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::AbsoluteX {
                        mode: AM::AbsoluteX { address: 0xcdab },
                    },
                },
            },
            0x8003
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ADC_ABSOLUTE_X, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf0]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_ABSOLUTE_X, 0x01, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x02;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0x72]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_ABSOLUTE_X, 0x02, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x00;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x80, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Carry));
}

#[test]
fn and_absolute_x() {
    use super::opcodes::{AND_ABSOLUTE_X, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[AND_ABSOLUTE_X, 0xab, 0xcd]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::AbsoluteX {
                        mode: AM::AbsoluteX { address: 0xcdab },
                    },
                },
            },
            0x8003
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[AND_ABSOLUTE_X, 0x01, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x1000, &[0x01, 0x02, 0xf0, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00000000);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0b11010011, AND_ABSOLUTE_X, 0x02, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory
        .load(0x0100, &[0x01, 0x02, 0x03, 0b10110001])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b10010001);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b00110111, AND_ABSOLUTE_X, 0x02, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x00;
    cpu.memory
        .load(0x0100, &[0x01, 0x02, 0b00110111, 0x04])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00110111);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
}

#[test]
fn asl_absolute_x() {
    use super::opcodes::ASL_ABSOLUTE_X;

    assert!(matches!(
        get_instruction(&[ASL_ABSOLUTE_X, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Asl {
                addressing_mode: AslAddressingMode::AslAddressAddressingMode {
                    mode: AslAddressAddressingMode::AbsoluteX {
                        mode: AM::AbsoluteX { address: 0xcdab },
                    },
                },
            },
            0x8003
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ASL_ABSOLUTE_X, 0x01, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory
        .load(0x0100, &[0x01, 0x02, 0b10000000, 0x04])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.memory.read(0x0102), 0b00000000);
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Carry));
    assert!(!cpu.status.get(Flag::Negative));

    let mut cpu = Cpu::new();
    cpu.load(&[ASL_ABSOLUTE_X, 0x03, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.register_x = 0x00;
    cpu.program_counter = 0x8000;
    cpu.memory
        .load(0x0100, &[0x01, 0x02, 0x03, 0b0101_0101])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.memory.read(0x0103), 0b1010_1010);
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Carry));
}

#[test]
fn lda_absolute_x() {
    use super::opcodes::LDA_ABSOLUTE_X;

    assert!(matches!(
        get_instruction(&[LDA_ABSOLUTE_X, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::AbsoluteX {
                        mode: AM::AbsoluteX { address: 0xcdab },
                    },
                },
            },
            0x8003,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_ABSOLUTE_X, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf4);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_ABSOLUTE_X, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn ldy_absolute_x() {
    use super::opcodes::LDY_ABSOLUTE_X;

    assert!(matches!(
        get_instruction(&[LDY_ABSOLUTE_X, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::Y,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::AbsoluteX {
                        mode: AM::AbsoluteX { address: 0xcdab },
                    },
                },
            },
            0x8003,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDY_ABSOLUTE_X, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_y, 0xf4);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDY_ABSOLUTE_X, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_y, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn adc_absolute_y() {
    use super::opcodes::{ADC_ABSOLUTE_Y, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[ADC_ABSOLUTE_Y, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::AbsoluteY {
                        mode: AM::AbsoluteY { address: 0xcdab },
                    },
                },
            },
            0x8003
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ADC_ABSOLUTE_Y, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf0]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_ABSOLUTE_Y, 0x01, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x02;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0x72]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_ABSOLUTE_Y, 0x02, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x00;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x80, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Carry));
}

#[test]
fn and_absolute_y() {
    use super::opcodes::{AND_ABSOLUTE_Y, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[AND_ABSOLUTE_Y, 0xab, 0xcd]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::AbsoluteY {
                        mode: AM::AbsoluteY { address: 0xcdab },
                    },
                },
            },
            0x8003
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[AND_ABSOLUTE_Y, 0x01, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x1000, &[0x01, 0x02, 0xf0, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00000000);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0b11010011, AND_ABSOLUTE_Y, 0x02, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory
        .load(0x0100, &[0x01, 0x02, 0x03, 0b10110001])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b10010001);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b00110111, AND_ABSOLUTE_Y, 0x02, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x00;
    cpu.memory
        .load(0x0100, &[0x01, 0x02, 0b00110111, 0x04])
        .unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00110111);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
}

#[test]
fn lda_absolute_y() {
    use super::opcodes::LDA_ABSOLUTE_Y;

    assert!(matches!(
        get_instruction(&[LDA_ABSOLUTE_Y, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::AbsoluteY {
                        mode: AM::AbsoluteY { address: 0xcdab },
                    },
                },
            },
            0x8003,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_ABSOLUTE_Y, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf4);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_ABSOLUTE_Y, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn ldx_absolute_y() {
    use super::opcodes::LDX_ABSOLUTE_Y;

    assert!(matches!(
        get_instruction(&[LDX_ABSOLUTE_Y, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::X,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::AbsoluteY {
                        mode: AM::AbsoluteY { address: 0xcdab },
                    },
                },
            },
            0x8003,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDX_ABSOLUTE_Y, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0xf4);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_ABSOLUTE_Y, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero))
}

#[test]
fn adc_indirect_x() {
    use super::opcodes::{ADC_INDIRECT_X, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[ADC_INDIRECT_X, 0xab]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::IndirectX {
                        mode: AM::IndirectX { address: 0xab },
                    },
                },
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ADC_INDIRECT_X, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0403, &[0xf0]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_INDIRECT_X, 0x00, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0302, &[0x72]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_INDIRECT_X, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x00;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0403, &[0x80]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Carry));
}

#[test]
fn and_indirect_x() {
    use super::opcodes::{AND_INDIRECT_X, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[AND_INDIRECT_X, 0xab]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::IndirectX {
                        mode: AM::IndirectX { address: 0xab },
                    },
                },
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[AND_INDIRECT_X, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0xff;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0201, &[0xf0]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00000000);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0b11010011, AND_INDIRECT_X, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0403, &[0b10110001]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b10010001);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b00110111, AND_INDIRECT_X, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x00;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0403, &[0b00110111]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00110111);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
}

#[test]
fn lda_indirect_x() {
    use super::opcodes::LDA_INDIRECT_X;

    assert!(matches!(
        get_instruction(&[LDA_INDIRECT_X, 0xab]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::IndirectX {
                        mode: AM::IndirectX { address: 0xab },
                    },
                },
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_INDIRECT_X, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0403, &[0xff]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xff);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_INDIRECT_X, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn adc_indirect_y() {
    use super::opcodes::{ADC_INDIRECT_Y, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[ADC_INDIRECT_Y, 0xab]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AdcAddressingMode::AdcAddressAddressingMode {
                    mode: AdcAddressAddressingMode::IndirectY {
                        mode: AM::IndirectY { address: 0xab },
                    },
                },
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ADC_INDIRECT_Y, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0303, &[0xf0]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(!cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_INDIRECT_Y, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x05;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0408, &[0x72]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(!cpu.status.get(Flag::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_INDIRECT_Y, 0x00, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x00;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0201, &[0x80]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
    assert!(cpu.status.get(Flag::Overflow));
    assert!(cpu.status.get(Flag::Carry));
}

#[test]
fn and_indirect_y() {
    use super::opcodes::{AND_INDIRECT_Y, LDA_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[AND_INDIRECT_Y, 0xab]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AndAddressingMode::AndAddressAddressingMode {
                    mode: AndAddressAddressingMode::IndirectY {
                        mode: AM::IndirectY { address: 0xab },
                    },
                },
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[AND_INDIRECT_Y, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0303, &[0xf0]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00000000);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0b11010011, AND_INDIRECT_Y, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x05;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0408, &[0b10110001]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b10010001);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b00110111, AND_INDIRECT_Y, 0x00, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x00;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0201, &[0b00110111]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b00110111);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));
}

#[test]
fn lda_indirect_y() {
    use super::opcodes::LDA_INDIRECT_Y;

    assert!(matches!(
        get_instruction(&[LDA_INDIRECT_Y, 0xab]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: LdAddressingMode::LdAddressAddressingMode {
                    mode: LdAddressAddressingMode::IndirectY {
                        mode: AM::IndirectY { address: 0xab },
                    },
                },
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_INDIRECT_Y, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0303, &[0xff]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xff);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_INDIRECT_Y, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn tax() {
    use super::opcodes::{LDA_IMMEDIATE, LDX_IMMEDIATE, TAX};

    assert!(matches!(
        get_instruction(&[TAX, 0x00]).unwrap(),
        (
            Instruction::Trr {
                origin: Register::A,
                destination: Register::X,
            },
            0x8001
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0xf0, TAX, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0xf0);
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_IMMEDIATE, 0x50, TAX, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

#[test]
fn inx() {
    use super::opcodes::{INX, LDX_IMMEDIATE};

    assert!(matches!(
        get_instruction(&[INX, 0x00]).unwrap(),
        (
            Instruction::In {
                destination: Register::X
            },
            0x8001
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_IMMEDIATE, 0xf1, INX, 0x00])
        .unwrap();
    assert!(cpu.status.get(Flag::Negative));
    assert!(!cpu.status.get(Flag::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_IMMEDIATE, 0xff, INX, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flag::Negative));
    assert!(cpu.status.get(Flag::Zero));
}

use super::*;

fn get_instruction(instructions: &[u8]) -> color_eyre::Result<(Instruction, u16)> {
    let mut memory = Memory::new();
    memory.load(0x8000, instructions)?;
    Instruction::get_instruction(&memory, &0x8000)
}

#[test]
fn adc_immediate() {
    assert!(matches!(
        get_instruction(&[ADC_IMMEDIATE, 0xc0]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AddressingMode::Immediate { immediate: 0xc0 }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![ADC_IMMEDIATE, 0xf0, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(!cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDA_IMMEDIATE, 0x71, ADC_IMMEDIATE, 0x72, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDA_IMMEDIATE, 0x80, ADC_IMMEDIATE, 0x80, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(cpu.status.get(Flags::Carry));
}

#[test]
fn lda_immediate() {
    assert!(matches!(
        get_instruction(&[LDA_IMMEDIATE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: AddressingMode::Immediate { immediate: 0xc0 }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDA_IMMEDIATE, 0xf0, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDA_IMMEDIATE, 0x00, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn ldx_immediate() {
    assert!(matches!(
        get_instruction(&[LDX_IMMEDIATE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::X,
                addressing_mode: AddressingMode::Immediate { immediate: 0xc0 }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDX_IMMEDIATE, 0xf0, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0xf0);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDX_IMMEDIATE, 0x00, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn lda_zero_page() {
    assert!(matches!(
        get_instruction(&[LDA_ZERO_PAGE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: AddressingMode::ZeroPage { address: 0xc0 }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(vec![LDA_ZERO_PAGE, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x00, &[0x01, 0x02, 0xf1, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf1);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDA_ZERO_PAGE, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn lda_zero_page_x() {
    assert!(matches!(
        get_instruction(&[LDA_ZERO_PAGE_X, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: AddressingMode::ZeroPageX { address: 0xc0 }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(vec![LDA_ZERO_PAGE_X, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf4);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDA_ZERO_PAGE_X, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn ldx_zero_page_y() {
    assert!(matches!(
        get_instruction(&[LDX_ZERO_PAGE_Y, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::X,
                addressing_mode: AddressingMode::ZeroPageY { address: 0xc0 }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(vec![LDX_ZERO_PAGE_Y, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0xf4);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDX_ZERO_PAGE_Y, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn lda_absolute() {
    assert!(matches!(
        get_instruction(&[LDA_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: AddressingMode::Absolute { address: 0xcdab }
            },
            0x8003,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(vec![LDA_ABSOLUTE, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0xf3, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf3);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDA_ABSOLUTE, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn lda_absolute_x() {
    assert!(matches!(
        get_instruction(&[LDA_ABSOLUTE_X, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: AddressingMode::AbsoluteX { address: 0xcdab }
            },
            0x8003,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(vec![LDA_ABSOLUTE_X, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf4);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDA_ABSOLUTE_X, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn lda_absolute_y() {
    assert!(matches!(
        get_instruction(&[LDA_ABSOLUTE_Y, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: AddressingMode::AbsoluteY { address: 0xcdab }
            },
            0x8003,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(vec![LDA_ABSOLUTE_Y, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf4);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDA_ABSOLUTE_X, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn lda_indirect_x() {
    assert!(matches!(
        get_instruction(&[LDA_INDIRECT_X, 0xab]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: AddressingMode::IndirectX { address: 0xab }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(vec![LDA_INDIRECT_X, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0403, &[0xff]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xff);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDA_INDIRECT_X, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn lda_indirect_y() {
    assert!(matches!(
        get_instruction(&[LDA_INDIRECT_Y, 0xab]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::A,
                addressing_mode: AddressingMode::IndirectY { address: 0xab }
            },
            0x8002,
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(vec![LDA_INDIRECT_Y, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    cpu.memory.load(0x0303, &[0xff]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xff);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDA_INDIRECT_Y, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn tax() {
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
    cpu.load_and_run_test(vec![LDA_IMMEDIATE, 0xf0, TAX, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0xf0);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDX_IMMEDIATE, 0x50, TAX, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn inx() {
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
    cpu.load_and_run_test(vec![LDX_IMMEDIATE, 0xf1, INX, 0x00])
        .unwrap();
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(vec![LDX_IMMEDIATE, 0xff, INX, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}
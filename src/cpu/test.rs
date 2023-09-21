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
    cpu.load_and_run_test(&[ADC_IMMEDIATE, 0xf0, 0x00]).unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(!cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0x71, ADC_IMMEDIATE, 0x72, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0x80, ADC_IMMEDIATE, 0x80, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(cpu.status.get(Flags::Carry));
}

#[test]
fn and_immediate() {
    assert!(matches!(
        get_instruction(&[AND_IMMEDIATE, 0xc0]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AddressingMode::Immediate { immediate: 0xc0 }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[AND_IMMEDIATE, 0b11010011, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0b00000000);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b11010011, AND_IMMEDIATE, 0b10110001, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0b10010001);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0b00110111, AND_IMMEDIATE, 0b00110111, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0b00110111);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
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
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0xf0, 0x00]).unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0x00, 0x00]).unwrap();
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
    cpu.load_and_run_test(&[LDX_IMMEDIATE, 0xf0, 0x00]).unwrap();
    assert_eq!(cpu.register_x, 0xf0);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_IMMEDIATE, 0x00, 0x00]).unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn ldy_immediate() {
    assert!(matches!(
        get_instruction(&[LDY_IMMEDIATE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::Y,
                addressing_mode: AddressingMode::Immediate { immediate: 0xc0 }
            },
            0x8002
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDY_IMMEDIATE, 0xf0, 0x00]).unwrap();
    assert_eq!(cpu.register_y, 0xf0);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDY_IMMEDIATE, 0x00, 0x00]).unwrap();
    assert_eq!(cpu.register_y, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn adc_zero_page() {
    assert!(matches!(
        get_instruction(&[ADC_ZERO_PAGE, 0xc0]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AddressingMode::ZeroPage { address: 0xc0 }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(!cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_ZERO_PAGE, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x72, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_ZERO_PAGE, 0x03, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x80]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(cpu.status.get(Flags::Carry));
}

#[test]
fn and_zero_page() {
    assert!(matches!(
        get_instruction(&[AND_ZERO_PAGE, 0xc0]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AddressingMode::ZeroPage { address: 0xc0 }
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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));

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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
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
    cpu.load(&[LDA_ZERO_PAGE, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x00, &[0x01, 0x02, 0xf1, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf1);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_ZERO_PAGE, 0x02, 0x00]).unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn ldx_zero_page() {
    assert!(matches!(
        get_instruction(&[LDX_ZERO_PAGE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::X,
                addressing_mode: AddressingMode::ZeroPage { address: 0xc0 }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_ZERO_PAGE, 0x02, 0x00]).unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn ldy_zero_page() {
    assert!(matches!(
        get_instruction(&[LDY_ZERO_PAGE, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::Y,
                addressing_mode: AddressingMode::ZeroPage { address: 0xc0 }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDY_ZERO_PAGE, 0x02, 0x00]).unwrap();
    assert_eq!(cpu.register_y, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn adc_zero_page_x() {
    assert!(matches!(
        get_instruction(&[ADC_ZERO_PAGE_X, 0xc0]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AddressingMode::ZeroPageX { address: 0xc0 }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(!cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_ZERO_PAGE_X, 0x02, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x72]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_ZERO_PAGE_X, 0x03, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x00;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0x80]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(cpu.status.get(Flags::Carry));
}

#[test]
fn and_zero_page_x() {
    assert!(matches!(
        get_instruction(&[AND_ZERO_PAGE_X, 0xc0]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AddressingMode::ZeroPageX { address: 0xc0 }
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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));

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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
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
    cpu.load(&[LDA_ZERO_PAGE_X, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf4);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_ZERO_PAGE_X, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn ldy_zero_page_x() {
    assert!(matches!(
        get_instruction(&[LDY_ZERO_PAGE_X, 0xc0]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::Y,
                addressing_mode: AddressingMode::ZeroPageX { address: 0xc0 }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDY_ZERO_PAGE_X, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_y, 0x00);
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
    cpu.load(&[LDX_ZERO_PAGE_Y, 0x02, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x00, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0xf4);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_ZERO_PAGE_Y, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn adc_absolute() {
    assert!(matches!(
        get_instruction(&[ADC_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AddressingMode::Absolute { address: 0xcdab }
            },
            0x8003
        )
    ));

    let mut cpu = Cpu::new();
    cpu.load(&[ADC_ABSOLUTE, 0x03, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf0]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf0);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(!cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_ABSOLUTE, 0x03, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0x72]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_ABSOLUTE, 0x02, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x80, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(cpu.status.get(Flags::Carry));
}

#[test]
fn and_absolute() {
    assert!(matches!(
        get_instruction(&[AND_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AddressingMode::Absolute { address: 0xcdab }
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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));

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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
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
    cpu.load(&[LDA_ABSOLUTE, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0xf3, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf3);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_ABSOLUTE, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn ldx_absolute() {
    assert!(matches!(
        get_instruction(&[LDX_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::X,
                addressing_mode: AddressingMode::Absolute { address: 0xcdab }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_ABSOLUTE, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn ldy_absolute() {
    assert!(matches!(
        get_instruction(&[LDY_ABSOLUTE, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::Y,
                addressing_mode: AddressingMode::Absolute { address: 0xcdab }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_ABSOLUTE, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_y, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn adc_absolute_x() {
    assert!(matches!(
        get_instruction(&[ADC_ABSOLUTE_X, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AddressingMode::AbsoluteX { address: 0xcdab }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(!cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_ABSOLUTE_X, 0x01, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x02;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0x72]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_ABSOLUTE_X, 0x02, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x00;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x80, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(cpu.status.get(Flags::Carry));
}

#[test]
fn and_absolute_x() {
    assert!(matches!(
        get_instruction(&[AND_ABSOLUTE_X, 0xab, 0xcd]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AddressingMode::AbsoluteX { address: 0xcdab }
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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));

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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
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
    cpu.load(&[LDA_ABSOLUTE_X, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf4);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_ABSOLUTE_X, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn adc_absolute_y() {
    assert!(matches!(
        get_instruction(&[ADC_ABSOLUTE_Y, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AddressingMode::AbsoluteY { address: 0xcdab }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(!cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x71, ADC_ABSOLUTE_Y, 0x01, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x02;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0x72]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, u8::wrapping_add(0x71, 0x72));
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

    let mut cpu = Cpu::new();
    cpu.load(&[LDA_IMMEDIATE, 0x80, ADC_ABSOLUTE_Y, 0x02, 0x01, 0x00])
        .unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x00;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x80, 0x04]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(cpu.status.get(Flags::Carry));
}

#[test]
fn and_absolute_y() {
    assert!(matches!(
        get_instruction(&[AND_ABSOLUTE_Y, 0xab, 0xcd]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AddressingMode::AbsoluteY { address: 0xcdab }
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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));

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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
}

#[test]
fn ldy_absolute_x() {
    assert!(matches!(
        get_instruction(&[LDY_ABSOLUTE_X, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::Y,
                addressing_mode: AddressingMode::AbsoluteX { address: 0xcdab }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDY_ABSOLUTE_X, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_y, 0x00);
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
    cpu.load(&[LDA_ABSOLUTE_Y, 0x02, 0x01, 0x00]).unwrap();
    cpu.reset().unwrap();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory.load(0x0100, &[0x01, 0x02, 0x03, 0xf4]).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf4);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDA_ABSOLUTE_Y, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn ldx_absolute_y() {
    assert!(matches!(
        get_instruction(&[LDX_ABSOLUTE_Y, 0xab, 0xcd]).unwrap(),
        (
            Instruction::Ld {
                destination: Register::X,
                addressing_mode: AddressingMode::AbsoluteY { address: 0xcdab }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_ABSOLUTE_Y, 0x00, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero))
}

#[test]
fn adc_indirect_x() {
    assert!(matches!(
        get_instruction(&[ADC_INDIRECT_X, 0xab]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AddressingMode::IndirectX { address: 0xab }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(!cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(cpu.status.get(Flags::Carry));
}

#[test]
fn and_indirect_x() {
    assert!(matches!(
        get_instruction(&[AND_INDIRECT_X, 0xab]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AddressingMode::IndirectX { address: 0xab }
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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));

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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
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
    cpu.load(&[LDA_INDIRECT_X, 0x01, 0x00]).unwrap();
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
    cpu.load_and_run_test(&[LDA_INDIRECT_X, 0x02, 0x00])
        .unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

#[test]
fn adc_indirect_y() {
    assert!(matches!(
        get_instruction(&[ADC_INDIRECT_Y, 0xab]).unwrap(),
        (
            Instruction::Adc {
                addressing_mode: AddressingMode::IndirectY { address: 0xab }
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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(!cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(!cpu.status.get(Flags::Carry));

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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
    assert!(cpu.status.get(Flags::Overflow));
    assert!(cpu.status.get(Flags::Carry));
}

#[test]
fn and_indirect_y() {
    assert!(matches!(
        get_instruction(&[AND_INDIRECT_Y, 0xab]).unwrap(),
        (
            Instruction::And {
                addressing_mode: AddressingMode::IndirectY { address: 0xab }
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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));

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
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

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
    assert!(!cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));
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
    cpu.load(&[LDA_INDIRECT_Y, 0x01, 0x00]).unwrap();
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
    cpu.load_and_run_test(&[LDA_INDIRECT_Y, 0x02, 0x00])
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
    cpu.load_and_run_test(&[LDA_IMMEDIATE, 0xf0, TAX, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0xf0);
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_IMMEDIATE, 0x50, TAX, 0x00])
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
    cpu.load_and_run_test(&[LDX_IMMEDIATE, 0xf1, INX, 0x00])
        .unwrap();
    assert!(cpu.status.get(Flags::Negative));
    assert!(!cpu.status.get(Flags::Zero));

    let mut cpu = Cpu::new();
    cpu.load_and_run_test(&[LDX_IMMEDIATE, 0xff, INX, 0x00])
        .unwrap();
    assert_eq!(cpu.register_x, 0x00);
    assert!(!cpu.status.get(Flags::Negative));
    assert!(cpu.status.get(Flags::Zero));
}

use derives::AddressingEnum;

use super::instruction::addressing_mode as AM;

use super::instruction::addressing_mode::{IntoAddress, IntoValue};

fn assert_into_address<T: IntoAddress>(_: T) {}
fn assert_into_value<T: IntoValue>(_: T) {}
fn assert_debug<T: std::fmt::Debug>(_: T) {}
fn assert_type<T>(_: T) {}

#[test]
fn accumulator_immediate() {
    #[derive(AddressingEnum)]
    enum _Foo {
        #[modes(mode = "accumulator", mode = "immediate")]
        Bar(BarAddressingMode),
    }

    let a: Option<BarAddressingMode> = None;

    if let Some(a) = a {
        assert_debug(a);
        match a {
            BarAddressingMode::Accumulator { mode } => {
                assert_type::<AM::Accumulator>(mode);
            }
            BarAddressingMode::Immediate { mode } => {
                assert_type::<AM::Immediate>(mode);
                assert_into_value(mode)
            }
        }
    }
}

#[test]
fn accumulator_address() {
    #[derive(AddressingEnum)]
    enum _Foo {
        #[modes(mode = "accumulator", mode = "zero_page")]
        Bar(BarAddressingMode),
    }

    let a: Option<BarAddressingMode> = None;
    if let Some(a) = a {
        assert_debug(a);
        match a {
            BarAddressingMode::Accumulator { mode } => {
                assert_type::<AM::Accumulator>(mode);
            }
            BarAddressingMode::ZeroPage { mode } => {
                assert_type::<AM::ZeroPage>(mode);
                assert_into_value(mode);
                assert_into_address(mode);
            }
        }
    }
}

#[test]
fn immediate_address() {
    #[derive(AddressingEnum)]
    enum _Foo {
        #[modes(mode = "immediate", mode = "zero_page_x")]
        Bar(BarAddressingMode),
    }

    let a: Option<BarAddressingMode> = None;
    if let Some(a) = a {
        assert_debug(a);
        assert_into_value(a);
        match a {
            BarAddressingMode::Immediate { mode } => {
                assert_type::<AM::Immediate>(mode);
                assert_into_value(mode)
            }
            BarAddressingMode::ZeroPageX { mode } => {
                assert_type::<AM::ZeroPageX>(mode);
                assert_into_address(mode);
                assert_into_value(mode);
            }
        }
    }
}

#[test]
fn addresses() {
    #[derive(AddressingEnum)]
    enum _Foo {
        #[modes(mode = "zero_page_y", mode = "relative")]
        Bar(BarAddressingMode),
    }

    let a: Option<BarAddressingMode> = None;
    if let Some(a) = a {
        assert_debug(a);
        assert_into_address(a);
        assert_into_value(a);
        match a {
            BarAddressingMode::ZeroPageY { mode } => {
                assert_type::<AM::ZeroPageY>(mode);
                assert_into_value(mode);
                assert_into_address(mode);
            }
            BarAddressingMode::Relative { mode } => {
                assert_type::<AM::Relative>(mode);
                assert_into_value(mode);
                assert_into_address(mode);
            }
        }
    }
}

#[test]
fn immediate_addresses() {
    #[derive(AddressingEnum)]
    enum _Foo {
        #[modes(mode = "immediate", mode = "absolute", mode = "absolute_x")]
        Bar(BarAddressingMode),
    }

    let a: Option<BarAddressingMode> = None;
    if let Some(a) = a {
        assert_debug(a);
        assert_into_value(a);
        match a {
            BarAddressingMode::Immediate { mode } => {
                assert_type::<AM::Immediate>(mode);
                assert_into_value(mode);
            }
            BarAddressingMode::BarAddressAddressingMode { mode } => {
                assert_debug(a);
                assert_into_value(mode);
                assert_into_address(mode);
                match mode {
                    BarAddressAddressingMode::Absolute { mode } => {
                        assert_type::<AM::Absolute>(mode);
                        assert_into_value(mode);
                        assert_into_address(mode);
                    }
                    BarAddressAddressingMode::AbsoluteX { mode } => {
                        assert_type::<AM::AbsoluteX>(mode);
                        assert_into_value(mode);
                        assert_into_address(mode);
                    }
                }
            }
        }
    }
}

#[test]
fn accumulator_addresses() {
    #[derive(AddressingEnum)]
    enum _Foo {
        #[modes(mode = "accumulator", mode = "absolute_y", mode = "indirect")]
        Bar(BarAddressingMode),
    }

    let a: Option<BarAddressingMode> = None;
    if let Some(a) = a {
        assert_debug(a);
        match a {
            BarAddressingMode::Accumulator { mode } => {
                assert_type::<AM::Accumulator>(mode);
            }
            BarAddressingMode::BarAddressAddressingMode { mode } => {
                assert_debug(a);
                assert_into_value(mode);
                assert_into_address(mode);
                match mode {
                    BarAddressAddressingMode::AbsoluteY { mode } => {
                        assert_type::<AM::AbsoluteY>(mode);
                        assert_into_value(mode);
                        assert_into_address(mode);
                    }
                    BarAddressAddressingMode::Indirect { mode } => {
                        assert_type::<AM::Indirect>(mode);
                        assert_into_value(mode);
                        assert_into_address(mode);
                    }
                }
            }
        }
    }
}

#[test]
fn accumulator_immediate_addresses() {
    #[derive(AddressingEnum)]
    enum _Foo {
        #[modes(
            mode = "accumulator",
            mode = "immediate",
            mode = "indirect_x",
            mode = "indirect_y"
        )]
        Baz(BazAddressingMode),
    }

    let a: Option<BazAddressingMode> = None;
    if let Some(a) = a {
        assert_debug(a);
        match a {
            BazAddressingMode::Accumulator { mode } => {
                assert_type::<AM::Accumulator>(mode);
            }
            BazAddressingMode::BazValueAddressingMode { mode } => {
                assert_debug(a);
                assert_into_value(mode);
                match mode {
                    BazValueAddressingMode::Immediate { mode } => {
                        assert_type::<AM::Immediate>(mode);
                        assert_into_value(mode);
                    }
                    BazValueAddressingMode::BazAddressAddressingMode { mode } => {
                        assert_debug(a);
                        assert_into_value(mode);
                        assert_into_address(mode);
                        match mode {
                            BazAddressAddressingMode::IndirectX { mode } => {
                                assert_type::<AM::IndirectX>(mode);
                                assert_into_value(mode);
                                assert_into_address(mode);
                            }
                            BazAddressAddressingMode::IndirectY { mode } => {
                                assert_type::<AM::IndirectY>(mode);
                                assert_into_value(mode);
                                assert_into_address(mode);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_multiple() {
    #[derive(AddressingEnum)]
    enum _Foo {
        Baz,
        #[modes(mode = "accumulator", mode = "immediate")]
        Bam(BamAddressingMode),
        #[modes(mode = "zero_page_y", mode = "relative")]
        Bar(BarAddressingMode),
    }

    let a: Option<BamAddressingMode> = None;

    if let Some(a) = a {
        assert_debug(a);
        match a {
            BamAddressingMode::Accumulator { mode } => {
                assert_type::<AM::Accumulator>(mode);
            }
            BamAddressingMode::Immediate { mode } => {
                assert_type::<AM::Immediate>(mode);
                assert_into_value(mode)
            }
        }
    }

    let a: Option<BarAddressingMode> = None;
    if let Some(a) = a {
        assert_debug(a);
        assert_into_address(a);
        assert_into_value(a);
        match a {
            BarAddressingMode::ZeroPageY { mode } => {
                assert_type::<AM::ZeroPageY>(mode);
                assert_into_value(mode);
                assert_into_address(mode);
            }
            BarAddressingMode::Relative { mode } => {
                assert_type::<AM::Relative>(mode);
                assert_into_value(mode);
                assert_into_address(mode);
            }
        }
    }
}

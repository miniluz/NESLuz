use quote::quote;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum AddressingMode {
    Accumulator(Accumulator),
    ValueAddressingMode(ValueAddressingMode),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Accumulator {}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Immediate {}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ValueAddressingMode {
    Immediate(Immediate),
    AddressAddressingMode(AddressAddressingMode),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum AddressAddressingMode {
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

/// From AddressingMode into Accumulator and ValueAddressingMode and viceversa

impl TryFrom<AddressingMode> for Accumulator {
    type Error = &'static str;

    fn try_from(value: AddressingMode) -> Result<Self, Self::Error> {
        match value {
            AddressingMode::Accumulator(accumulator) => Ok(accumulator),
            _ => Err("Expected Accumulator variant."),
        }
    }
}

impl TryFrom<AddressingMode> for ValueAddressingMode {
    type Error = &'static str;

    fn try_from(value: AddressingMode) -> Result<Self, Self::Error> {
        match value {
            AddressingMode::ValueAddressingMode(addressing_mode) => Ok(addressing_mode),
            _ => Err("Expected ValueAddressingMode variant."),
        }
    }
}

impl TryFrom<ValueAddressingMode> for Immediate {
    type Error = &'static str;

    fn try_from(value: ValueAddressingMode) -> Result<Self, Self::Error> {
        match value {
            ValueAddressingMode::Immediate(value) => Ok(value),
            _ => Err("Expected AddressAddressingMode variant."),
        }
    }
}

impl TryFrom<ValueAddressingMode> for AddressAddressingMode {
    type Error = &'static str;

    fn try_from(value: ValueAddressingMode) -> Result<Self, Self::Error> {
        match value {
            ValueAddressingMode::AddressAddressingMode(addressing_mode) => Ok(addressing_mode),
            _ => Err("Expected AddressAddressingMode variant."),
        }
    }
}

// impl TryFrom<AddressingMode> for AddressAddressingMode {
//     type Error = &'static str;
//
//     fn try_from(value: AddressingMode) -> Result<Self, Self::Error> {
//         ValueAddressingMode::try_from(value)?.try_into()
//     }
// }

impl From<ValueAddressingMode> for AddressingMode {
    fn from(value: ValueAddressingMode) -> Self {
        AddressingMode::ValueAddressingMode(value)
    }
}

impl From<Accumulator> for AddressingMode {
    fn from(value: Accumulator) -> Self {
        AddressingMode::Accumulator(value)
    }
}

/// From String to AddressingMode

impl TryFrom<String> for AddressingMode {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use AddressAddressingMode::*;
        use AddressingMode::ValueAddressingMode as VAD;
        use ValueAddressingMode::AddressAddressingMode as AAD;
        match value.as_ref() {
            "accumulator" => Ok(AddressingMode::Accumulator(Accumulator {  })),
            "immediate" => Ok(VAD(ValueAddressingMode::Immediate (Immediate{}))),
            "zero_page" => Ok(VAD(AAD(ZeroPage))),
            "zero_page_x" => Ok(VAD(AAD(ZeroPageX))),
            "zero_page_y" => Ok(VAD(AAD(ZeroPageY))),
            "relative" => Ok(VAD(AAD(Relative))),
            "absolute" => Ok(VAD(AAD(Absolute))),
            "absolute_x" => Ok(VAD(AAD(AbsoluteX))),
            "absolute_y" => Ok(VAD(AAD(AbsoluteY))),
            "indirect" => Ok(VAD(AAD(Indirect))),
            "indirect_x" => Ok(VAD(AAD(IndirectX))),
            "indirect_y" => Ok(VAD(AAD(IndirectY))),
            _ => Err("Invalid type; expected accumulator, immediate, zero_page, zero_page_x, zero_page_y, relative, absolute, absolute_x, absolute_y, indirect, indirect_x or indirect_y"),
        }
    }
}

/// From Accumulator and ValueAddressingMode to TokenStream

impl From<Accumulator> for proc_macro2::TokenStream {
    fn from(_value: Accumulator) -> Self {
        quote!(Accumulator)
    }
}

impl From<Immediate> for proc_macro2::TokenStream {
    fn from(_value: Immediate) -> Self {
        quote!(Immediate)
    }
}

impl From<AddressAddressingMode> for proc_macro2::TokenStream {
    fn from(value: AddressAddressingMode) -> Self {
        use AddressAddressingMode::*;
        match value {
            ZeroPage => {
                quote!(ZeroPage)
            }
            ZeroPageX => quote!(ZeroPageX),
            ZeroPageY => quote!(ZeroPageY),
            Relative => {
                quote!(Relative)
            }
            Absolute => {
                quote!(Absolute)
            }
            AbsoluteX => quote!(AbsoluteX),
            AbsoluteY => quote!(AbsoluteY),
            Indirect => {
                quote!(Indirect)
            }
            IndirectX => quote!(IndirectX),
            IndirectY => quote!(IndirectY),
        }
    }
}

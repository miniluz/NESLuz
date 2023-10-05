use std::collections::HashSet;

use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AddressingEnum, attributes(modes))]
pub fn derive_addressing_enums(input: TokenStream) -> TokenStream {
    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    enum AddressingMode {
        Accumulator(Accumulator),
        ValueAddressingMode(ValueAddressingMode),
    }

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    enum Accumulator {
        Accumulator,
    }

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    enum ValueAddressingMode {
        Immediate,
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
            use Accumulator::Accumulator as Acc;
            use AddressingMode::ValueAddressingMode as VAD;
            use ValueAddressingMode::*;
            match value.as_ref() {
                "accumulator" => Ok(AddressingMode::Accumulator(Acc)),
                "immediate" => Ok(VAD(Immediate)),
                "zero_page" => Ok(VAD(ZeroPage)),
                "zero_page_x" => Ok(VAD(ZeroPageX)),
                "zero_page_y" => Ok(VAD(ZeroPageY)),
                "relative" => Ok(VAD(Relative)),
                "absolute" => Ok(VAD(Absolute)),
                "absolute_x" => Ok(VAD(AbsoluteX)),
                "absolute_y" => Ok(VAD(AbsoluteY)),
                "indirect" => Ok(VAD(Indirect)),
                "indirect_x" => Ok(VAD(IndirectX)),
                "indirect_y" => Ok(VAD(IndirectY)),
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

    impl From<ValueAddressingMode> for proc_macro2::TokenStream {
        fn from(value: ValueAddressingMode) -> Self {
            use ValueAddressingMode as VAM;
            match value {
                VAM::Immediate => quote!(Immediate),
                VAM::ZeroPage => {
                    quote!(ZeroPage)
                }
                VAM::ZeroPageX => quote!(ZeroPageX),
                VAM::ZeroPageY => quote!(ZeroPageY),
                VAM::Relative => {
                    quote!(Relative)
                }
                VAM::Absolute => {
                    quote!(Absolute)
                }
                VAM::AbsoluteX => quote!(AbsoluteX),
                VAM::AbsoluteY => quote!(AbsoluteY),
                VAM::Indirect => {
                    quote!(Indirect)
                }
                VAM::IndirectX => quote!(IndirectX),
                VAM::IndirectY => quote!(IndirectY),
            }
        }
    }

    fn map_to_adressing_mode(string: String) -> AddressingMode {
        string.try_into().unwrap()
    }

    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(modes))]
    struct AddressingOptions {
        ident: syn::Ident,
        #[darling(multiple, rename="mode", map=map_to_adressing_mode)]
        modes: Vec<AddressingMode>,
    }

    let input = parse_macro_input!(input as DeriveInput);

    let options: AddressingOptions = FromDeriveInput::from_derive_input(&input).unwrap();
    let modes: HashSet<AddressingMode> = options.modes.into_iter().collect();

    let implicit_mode = modes
        .iter()
        .filter_map(|mode| Accumulator::try_from(*mode).ok())
        .next();

    let value_modes = modes
        .iter()
        .filter_map(|mode| ValueAddressingMode::try_from(*mode).ok());

    fn get_value_enum(
        name: &Ident,
        value_modes: impl Iterator<Item = ValueAddressingMode>,
    ) -> proc_macro2::TokenStream {
        let value_modes: Vec<proc_macro2::TokenStream> =
            value_modes.map(|mode| mode.into()).collect();
        quote!(
            #[automatically_derived]
            #[derive(Debug)]
            enum #name {
                #( #value_modes { mode: crate::cpu::instruction::addressing_mode::#value_modes },)*
            }

            impl crate::cpu::instruction::addressing_mode::TryIntoValue for #name {
                fn try_into_value(&self, cpu: &crate::cpu::Cpu) -> Result<u8, crate::cpu::CpuError> {
                    let result = match self {
                        #( #name::#value_modes {mode} => mode.try_into_value(cpu)?, )*
                    };
                    Ok(result)
                }
            }
        )
    }

    fn only_first_capitalized(string: &str) -> String {
        let mut c = string.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    let struct_name = only_first_capitalized(&options.ident.to_string());
    let enum_name = format_ident!("{}AddressingMode", struct_name);

    let output = if let Some(implicit_mode) = implicit_mode {
        let implicit_variant = proc_macro2::TokenStream::from(implicit_mode);
        let value_name = format_ident!("{}ValueAddressingMode", struct_name);

        let value_enum = get_value_enum(&value_name, value_modes);

        quote!(
            #value_enum

            #[derive(Debug)]
            enum #enum_name {
                #implicit_variant,
                ValueAddressingMode (#value_name),
            }
        )
    } else {
        get_value_enum(&enum_name, value_modes)
    };

    output.into()
}

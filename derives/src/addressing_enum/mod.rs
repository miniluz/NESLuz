use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Ident};

mod parse_enum;
use parse_enum::*;

mod types;
use types::*;

pub fn parse_variant(input: Variant) -> TokenStream {
    fn get_address_enum(
        enum_name: &Ident,
        address_modes: Vec<AddressAddressingMode>,
    ) -> TokenStream {
        let address_modes: Vec<TokenStream> =
            address_modes.into_iter().map(|mode| mode.into()).collect();

        quote!(
            #[automatically_derived]
            #[derive(Debug)]
            enum #enum_name {
                #( #address_modes { mode: crate::cpu::instruction::addressing_mode::#address_modes },)*
            }

            impl crate::cpu::instruction::addressing_mode::IntoAddress for #enum_name {
                fn into_address(&self, cpu: &crate::cpu::Cpu) -> u16 {
                    let result = match self {
                        #( Self::#address_modes {mode} => mode.into_address(cpu), )*
                    };
                    result
                }
            }
        )
    }

    fn get_value_enum<'a>(
        enum_name: &'a Ident,
        address_enum_name: &'a Ident,
        has_accumulator: bool,
        value_modes: Vec<ValueAddressingMode>,
    ) -> (TokenStream, &'a Ident) {
        let value_mode = value_modes
            .iter()
            .filter_map(|mode| Immediate::try_from(*mode).ok())
            .next();
        let address_modes = value_modes
            .iter()
            .filter_map(|mode| AddressAddressingMode::try_from(*mode).ok())
            .collect::<Vec<_>>();

        if let Some(value_mode) = value_mode {
            let value_variant: TokenStream = value_mode.into();

            if address_modes.len() == 1 {
                let address_variant: TokenStream = address_modes[0].into();
                (
                    quote!(
                        enum #enum_name {
                            #value_variant { mode: crate::cpu::instruction::addressing_mode::#value_variant },
                            #address_variant { mode: crate::cpu::instruction::addressing_mode::#address_variant },
                        }

                        impl crate::cpu::instruction::addressing_mode::IntoValue for #enum_name {
                            fn into_value(&self, cpu: &crate::cpu::Cpu) -> u8 {
                                match self {
                                    Self::#value_variant { mode } => mode.into_value(cpu),
                                    Self::#address_variant { mode } => mode.into_value(cpu),
                                }
                            }
                        }
                    ),
                    enum_name,
                )
            } else {
                let address_enum = get_address_enum(address_enum_name, address_modes);

                (
                    quote!(
                        #address_enum

                        #[automatically_derived]
                        #[derive(Debug)]
                        enum #enum_name {
                            #value_variant { mode: crate::cpu::instruction::addressing_mode::#value_variant },
                            #address_enum_name { mode: #address_enum_name }
                        }

                        impl crate::cpu::instruction::addressing_mode::IntoValue for #enum_name {
                        fn into_value(&self, cpu: &crate::cpu::Cpu) -> u8 {
                            let result = match self {
                                Self::#value_variant { mode } => mode.into_value(cpu),
                                Self::#address_enum_name { mode } => mode.into_value(cpu),
                            };
                            result
                        }
                    }
                    ),
                    enum_name,
                )
            }
        } else {
            if has_accumulator {
                (
                    get_address_enum(address_enum_name, address_modes),
                    address_enum_name,
                )
            } else {
                (get_address_enum(enum_name, address_modes), enum_name)
            }
        }
    }

    fn get_enum(name: String, modes: HashSet<AddressingMode>) -> TokenStream {
        let implicit_mode = modes
            .iter()
            .filter_map(|mode| Accumulator::try_from(*mode).ok())
            .next();

        let value_modes = modes
            .iter()
            .filter_map(|mode| ValueAddressingMode::try_from(*mode).ok())
            .collect::<Vec<_>>();

        let enum_name = format_ident!("{}AddressingMode", name);

        if let Some(implicit_mode) = implicit_mode {
            let implicit_variant: TokenStream = implicit_mode.into();
            if value_modes.len() == 1 {
                let value_variant: TokenStream = match value_modes[0] {
                    ValueAddressingMode::Immediate(immediate) => immediate.into(),
                    ValueAddressingMode::AddressAddressingMode(mode) => mode.into(),
                };
                quote!(
                    enum #enum_name {
                        #implicit_variant { mode: crate::cpu::instruction::addressing_mode::#implicit_variant },
                        #value_variant { mode: crate::cpu::instruction::addressing_mode::#value_variant },
                    }
                )
            } else {
                let value_enum_name = format_ident!("{}ValueAddressingMode", name);
                let address_enum_name = format_ident!("{}AddressAddressingMode", name);
                let (value_enum, value_enum_name) =
                    get_value_enum(&value_enum_name, &address_enum_name, true, value_modes);

                quote!(
                    #value_enum

                    enum #enum_name {
                        #implicit_variant { mode: crate::cpu::instruction::addressing_mode::#implicit_variant },
                        #value_enum_name { mode: #value_enum_name }
                    }
                )
            }
        } else {
            let address_enum_name = format_ident!("{}AddressAddressingMode", name);
            let (enum_stream, _) =
                get_value_enum(&enum_name, &address_enum_name, false, value_modes);
            return enum_stream;
        }
    }

    let name = input.ident;
    let modes: HashSet<AddressingMode> = input
        .modes
        .into_iter()
        .map(|mode| mode.try_into().unwrap())
        .collect();
    if modes.len() == 1 {
        panic!("Please select at least 2 modes.");
    } else if modes.len() == 0 {
        return quote!();
    }

    get_enum(name.to_string(), modes)
}

pub fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let input = match &input.data {
        Data::Enum(data) => Enum::from_syn(&input, data),
        _ => panic!("Was expecting enum"),
    }?;

    let enums = input
        .variants
        .into_iter()
        .map(parse_variant)
        .collect::<Vec<_>>();

    Ok(quote!(#(#enums)*))
}

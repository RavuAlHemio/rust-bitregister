mod model;


use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote};
use serde_yaml;

use crate::model::{Block, Field, Group, Register, RegisterOrReserved, VariableField};


#[derive(Debug, Parser)]
struct Opts {
    /// The model definition file, used as an input.
    pub model_file: PathBuf,

    /// The Rust source file, used as an output.
    pub rust_file: PathBuf,
}

fn integer_type_for_bit_count(bit_count: u8) -> &'static str {
    match bit_count {
        1 => "bool",
        2..=8 => "u8",
        9..=16 => "u16",
        17..=32 => "u32",
        33..=64 => "u64",
        _ => panic!("no good integer type for field of {} bits", bit_count),
    }
}

fn serialize_field_reader(field: &VariableField) -> TokenStream {
    let field_name_lower = Ident::new(&field.name.to_lowercase(), Span::call_site());
    let value_type = integer_type_for_bit_count(field.bit_count);
    let value_type_ident = Ident::new(value_type, Span::call_site());
    let shift_count = Literal::u8_unsuffixed(field.start_bit);
    let bit_mask_at_lsb = Literal::u64_unsuffixed((1 << field.bit_count) - 1);

    let conversion_tokens = if value_type == "bool" {
        quote! { != 0 }
    } else {
        quote! { as #value_type_ident }
    };

    quote! {
        #[inline(always)]
        pub fn #field_name_lower (&self) -> #value_type_ident {
            ( ( self.register.value >> #shift_count ) & #bit_mask_at_lsb ) #conversion_tokens
        }
    }
}

fn serialize_field_writer(register_backing_type: &Ident, field: &VariableField) -> TokenStream {
    let field_name_lower = Ident::new(&field.name.to_lowercase(), Span::call_site());
    let value_type = integer_type_for_bit_count(field.bit_count);
    let value_type_ident = Ident::new(value_type, Span::call_site());
    let shift_count = Literal::u8_unsuffixed(field.start_bit);

    let bit_mask_value: u64 = (1 << field.bit_count) - 1;
    let bit_mask_in_position = Literal::u64_unsuffixed(bit_mask_value << field.start_bit);

    let value_numeric = if value_type == "bool" {
        quote! { (if value { 1 as #register_backing_type } else { 0 as #register_backing_type }) }
    } else {
        quote! { (value as #register_backing_type ) }
    };

    quote! {
        #[inline(always)]
        pub fn #field_name_lower (&mut self, value: #value_type_ident ) -> &mut Self {
            self.register.value = (self.register.value & ( #register_backing_type :: MAX ^ #bit_mask_in_position ))
                | (( #value_numeric << #shift_count ) & #bit_mask_in_position);
            self
        }
    }
}

fn serialize_register_def(register: &Register) -> TokenStream {
    let register_name_upper = Ident::new(&register.name.to_uppercase(), Span::call_site());
    let register_reader_upper = format_ident!("{}_R", register.name.to_uppercase());
    let register_writer_upper = format_ident!("{}_W", register.name.to_uppercase());

    let register_backing_type: Ident = match register.size_bytes {
        1 => Ident::new("u8", Span::call_site()),
        2 => Ident::new("u16", Span::call_site()),
        4 => Ident::new("u32", Span::call_site()),
        8 => Ident::new("u64", Span::call_site()),
        other => panic!("unsupported register size {}", other),
    };

    let register_reader_funcs: Vec<TokenStream> = register.fields.iter()
        .filter_map(|field| match field {
            Field::Variable(f) => Some(serialize_field_reader(f)),
            Field::Fixed(_) => None,
        })
        .collect();
    let register_writer_fields: Vec<TokenStream> = register.fields.iter()
        .filter_map(|field| match field {
            Field::Variable(f) => Some(serialize_field_writer(&register_backing_type, f)),
            Field::Fixed(_) => None,
        })
        .collect();

    let (write_func, const_default_func, default_impl) = if let Some(default_value) = register.default_value {
        let default_value_token = Literal::u64_unsuffixed(default_value);
        let wf = quote! {
            #[inline(always)]
            pub fn write<'a>(&'a mut self) -> #register_writer_upper <'a> {
                self.value = Self::const_default().value;
                #register_writer_upper { register: self }
            }
        };
        let cd = quote! {
            #[inline(always)]
            pub const fn const_default() -> Self { Self { value: #default_value_token } }
        };
        let d = quote! {
            impl Default for #register_name_upper {
                fn default() -> Self { Self::const_default() }
            }
        };
        (wf, cd, d)
    } else {
        let wf = quote! {};
        let cd = quote! {};
        let d = quote! {};
        (wf, cd, d)
    };

    quote! {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct #register_name_upper {
            value: #register_backing_type ,
        }
        impl #register_name_upper {
            #[inline(always)]
            pub fn read<'a>(&'a self) -> #register_reader_upper <'a> {
                #register_reader_upper { register: self }
            }

            #[inline(always)]
            pub fn modify<'a>(&'a mut self) -> #register_writer_upper <'a> {
                #register_writer_upper { register: self }
            }

            #write_func

            #const_default_func
        }
        #default_impl

        pub struct #register_reader_upper <'a> {
            register: &'a #register_name_upper ,
        }
        impl<'a> #register_reader_upper <'a> {
            #( #register_reader_funcs )*
        }

        pub struct #register_writer_upper <'a> {
            register: &'a mut #register_name_upper ,
        }
        impl<'a> #register_writer_upper <'a> {
            #( #register_writer_fields )*
        }
    }
}

fn serialize_register_field(block_name_lower: &Ident, register: &RegisterOrReserved, reserved_counter: &mut usize) -> TokenStream {
    match register {
        RegisterOrReserved::Register(r) => {
            let register_name_upper = Ident::new(&r.name.to_uppercase(), Span::call_site());
            let register_name_lower = Ident::new(&r.name.to_lowercase(), Span::call_site());
            quote! {
                pub #register_name_lower : #block_name_lower :: #register_name_upper ,
            }
        },
        RegisterOrReserved::Reserved(r) => {
            let reserved_ident = format_ident!("_reserved{}", *reserved_counter);
            *reserved_counter += 1;
            let size_bytes: usize = r.size_bytes.into();
            quote! {
                #reserved_ident : [u8; #size_bytes],
            }
        },
    }
}

fn serialize_block_def(block: &Block) -> TokenStream {
    let block_name_upper = Ident::new(&block.name.to_uppercase(), Span::call_site());
    let block_name_lower = Ident::new(&block.name.to_lowercase(), Span::call_site());

    let mut current_bytes: usize = 0;
    let register_defs: Vec<TokenStream> = block.registers.iter()
        .filter_map(|ror| {
            current_bytes += usize::from(ror.size_bytes());
            match ror {
                RegisterOrReserved::Register(r) => Some(serialize_register_def(r)),
                RegisterOrReserved::Reserved(_) => None,
            }
        })
        .collect();
    let mut reserved_counter: usize = 0;
    let register_fields: Vec<TokenStream> = block.registers.iter()
        .map(|ror| serialize_register_field(&block_name_lower, ror, &mut reserved_counter))
        .collect();

    let impl_default = if block.has_default() {
        reserved_counter = 0;
        let register_default_entries: Vec<TokenStream> = block.registers.iter()
            .filter_map(|ror| match ror {
                RegisterOrReserved::Register(r) => {
                    if r.default_value.is_some() {
                        let register_name_lower = Ident::new(&r.name.to_lowercase(), Span::call_site());
                        let register_name_upper = Ident::new(&r.name.to_uppercase(), Span::call_site());
                        Some(quote! { #register_name_lower : #block_name_lower :: #register_name_upper :: const_default () , })
                    } else {
                        None
                    }
                },
                RegisterOrReserved::Reserved(r) => {
                    let ident = format_ident!("_reserved{}", reserved_counter);
                    let size = usize::from(r.size_bytes);
                    reserved_counter += 1;
                    Some(quote! { #ident : [ 0u8 ; #size ] , })
                },
            })
            .collect();
        quote! {
            impl #block_name_upper {
                #[inline(always)]
                pub const fn const_default() -> Self {
                    Self {
                        #( #register_default_entries )*
                    }
                }
            }
            impl Default for #block_name_upper {
                fn default() -> Self { Self::const_default() }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        pub mod #block_name_lower {
            #( #register_defs )*
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(C)]
        pub struct #block_name_upper {
            #( #register_fields )*
        }

        #impl_default
    }
}

fn serialize_block_field(group_name_lower: &Ident, block: &Block) -> TokenStream {
    let block_name_upper = Ident::new(&block.name.to_uppercase(), Span::call_site());
    let block_name_lower = Ident::new(&block.name.to_lowercase(), Span::call_site());

    quote! {
        pub #block_name_lower : #group_name_lower :: #block_name_upper ,
    }
}

fn serialize_group(group: &Group) -> TokenStream {
    let group_name_upper = Ident::new(&group.name.to_uppercase(), Span::call_site());
    let group_name_lower = Ident::new(&group.name.to_lowercase(), Span::call_site());

    let group_block_defs: Vec<TokenStream> = group.blocks.iter()
        .map(|b| serialize_block_def(b))
        .collect();
    let group_block_fields: Vec<TokenStream> = group.blocks.iter()
        .map(|b| serialize_block_field(&group_name_lower, b))
        .collect();

    let impl_default = if group.has_default() {
        let block_defaults = group.blocks.iter()
            .map(|b| {
                let block_name_upper = Ident::new(&b.name.to_uppercase(), Span::call_site());
                let block_name_lower = Ident::new(&b.name.to_lowercase(), Span::call_site());

                quote! {
                    #block_name_lower : #group_name_lower :: #block_name_upper :: const_default () ,
                }
            });
        quote! {
            impl #group_name_upper {
                #[inline(always)]
                pub const fn const_default() -> Self {
                    Self {
                        #( #block_defaults )*
                    }
                }
            }
            impl Default for #group_name_upper {
                fn default() -> Self { Self::const_default() }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #![allow(non_camel_case_types)]

        pub mod #group_name_lower {
            #( #group_block_defs )*
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(C)]
        pub struct #group_name_upper {
            #( #group_block_fields )*
        }

        #impl_default
    }
}


fn main() {
    let opts = Opts::parse();

    let group: Group = {
        let f = File::open(&opts.model_file)
            .expect("failed to open model file");
        serde_yaml::from_reader(f)
            .expect("failed to parse model file")
    };

    let group_tokens = serialize_group(&group);

    {
        let mut f = File::create(&opts.rust_file)
            .expect("failed to create Rust file");
        write!(f, "{}", group_tokens)
            .expect("failed to write to Rust file");
    }
}

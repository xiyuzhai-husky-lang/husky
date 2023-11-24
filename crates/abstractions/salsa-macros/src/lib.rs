//! This crate provides salsa's macros and attributes.

#![recursion_limit = "256"]

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

macro_rules! parse_quote {
    ($($inp:tt)*) => {
        syn::parse2(quote!{$($inp)*}).unwrap_or_else(|err| {
            panic!("failed to parse at {}:{}:{}: {}", file!(), line!(), column!(), err)
        })
    }
}

macro_rules! parse_quote_spanned {
    ($($inp:tt)*) => {
        syn::parse2(quote_spanned!{$($inp)*}).unwrap_or_else(|err| {
            panic!("failed to parse at {}:{}:{}: {}", file!(), line!(), column!(), err)
        })
    }
}

/// Convert a single Ident to Literal: useful when &'static str is needed.
pub(crate) fn literal(ident: &proc_macro2::Ident) -> proc_macro2::Literal {
    proc_macro2::Literal::string(&ident.to_string())
}

mod accumulator;
mod configuration;
mod db;
mod debug_with_db;
mod input;
mod interned;
mod jar;
mod options;
mod salsa_struct;
mod test_db;
mod tracked;
mod tracked_fn;
mod tracked_struct;
mod wrap_id;

#[proc_macro_attribute]
pub fn accumulator(args: TokenStream, input: TokenStream) -> TokenStream {
    accumulator::accumulator(args, input)
}

#[proc_macro_attribute]
pub fn jar(args: TokenStream, input: TokenStream) -> TokenStream {
    jar::jar(args, input)
}

#[proc_macro_attribute]
pub fn debug_with_db(args: TokenStream, input: TokenStream) -> TokenStream {
    debug_with_db::debug_with_db(args, input)
}

#[proc_macro_attribute]
pub fn wrap_id(args: TokenStream, input: TokenStream) -> TokenStream {
    wrap_id::wrap_id(args, input)
}

#[proc_macro_attribute]
pub fn db(args: TokenStream, input: TokenStream) -> TokenStream {
    db::db(args, input)
}

#[proc_macro_attribute]
pub fn test_db(args: TokenStream, input: TokenStream) -> TokenStream {
    test_db::test_db(args, input)
}

#[proc_macro_attribute]
pub fn interned(args: TokenStream, input: TokenStream) -> TokenStream {
    interned::interned(args, input)
}

#[proc_macro_attribute]
pub fn input(args: TokenStream, input: TokenStream) -> TokenStream {
    input::input(args, input)
}

#[proc_macro_attribute]
pub fn tracked(args: TokenStream, input: TokenStream) -> TokenStream {
    tracked::tracked(args, input)
}

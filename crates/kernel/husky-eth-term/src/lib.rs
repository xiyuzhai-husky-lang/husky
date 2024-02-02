#![feature(impl_trait_in_assoc_type)]
#![feature(result_flattening)]
#![doc = include_str ! ("../README.md")]
#![feature(let_chains)]
#![feature(if_let_guard)]
// #![deny(unsafe_code, missing_docs, clippy::unwrap_used)]
mod context;
mod conversion;
mod db;
mod error;
mod helpers;
pub mod instantiation;
mod menu;
mod rewrite;
mod template_parameter;
mod term;
#[cfg(test)]
mod tests;
mod ty;

pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::substitute::*;
pub use self::template_parameter::*;
pub use self::term::*;
pub use self::ty::*;

use self::context::*;
use either::*;
use husky_coword::Ident;
use husky_dec_signature::*;
use husky_dec_term::{term::*, *};
use husky_entity_path::*;
use husky_term_prelude::*;
use husky_vfs::*;
use salsa::DebugWithDb;
use smallvec::*;

#[salsa::jar]
pub struct EthTermJar(
    // symbol
    EthSymbol,
    // rune
    EthRune,
    // curry
    EthCurry,
    term_curry_from_declarative,
    crate::term::curry::curry_parameter_count,
    // curry_parameter_count,
    // ritchie
    EthRitchie,
    ethereal_term_ritchie_from_declarative_term_ritchie,
    // abstraction
    EthAbstraction,
    // application
    EthApplication,
    application_expansion_salsa,
    ethereal_term_from_declarative_term_application,
    ethereal_term_application_declarative_ty,
    // - application reduction
    crate::term::application::reduction::reduce_term_application,
    // - application expansion
    EtherealApplicationArguments,
    // ty as trait associated item
    EthTypeAsTraitItem,
    // trait constraint
    EthTraitConstraint,
    term_menu,
    // other
    ethereal_term_from_application_or_ritchie_call_declarative_term,
    ethereal_term_from_list_declarative_term,
    ethereal_term_from_declarative_term_wrapper,
    // // trai
    // trai_side_trai_for_ty_impl_blocks_aux,
    // ty_side_trai_for_ty_impl_blocks_aux,
    // trai_for_type_impl_template_from_impl_block,
    // // template
    // TemplateParameters,
    // ty_path_template_parameters,
    // utils
    helpers::ethereal_term_curry_toolchain,
    helpers::ethereal_term_application_toolchain,
    helpers::ethereal_term_ritchie_toolchain,
    helpers::final_destination::application_ethereal_term_final_destination,
    helpers::final_destination::curry_ethereal_term_final_destination,
);

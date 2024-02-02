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
pub mod term;
#[cfg(test)]
mod tests;
mod ty;

pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::substitute::*;
pub use self::template_parameter::*;
pub use self::ty::*;

use self::context::*;
use self::term::EthTerm;
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
    crate::term::symbol::EthSymbol,
    // rune
    crate::term::rune::EthRune,
    // curry
    crate::term::curry::EthCurry,
    crate::term::curry::term_curry_from_declarative,
    crate::term::curry::curry_parameter_count,
    // curry_parameter_count,
    // ritchie
    crate::term::ritchie::EthRitchie,
    crate::term::ritchie::ethereal_term_ritchie_from_declarative_term_ritchie,
    // abstraction
    crate::term::abstraction::EthAbstraction,
    // application
    crate::term::application::EthApplication,
    crate::term::application::application_expansion_salsa,
    crate::term::application::ethereal_term_from_declarative_term_application,
    crate::term::application::ethereal_term_application_declarative_ty,
    // - application reduction
    crate::term::application::reduction::reduce_term_application,
    // - application expansion
    crate::term::application::EtherealApplicationArguments,
    // ty as trait item
    crate::term::ty_as_trai_item::EthTypeAsTraitItem,
    // trait constraint
    crate::term::trai_constraint::EthTraitConstraint,
    term_menu,
    // other
    crate::term::ethereal_term_from_application_or_ritchie_call_declarative_term,
    crate::term::ethereal_term_from_list_declarative_term,
    crate::term::ethereal_term_from_declarative_term_wrapper,
    // // trai
    // trai_side_trai_for_ty_impl_blocks_aux,
    // ty_side_trai_for_ty_impl_blocks_aux,
    // trai_for_type_impl_template_from_impl_block,
    // // template
    // TemplateParameters,
    // ty_path_template_parameters,
    // utils
    crate::helpers::ethereal_term_curry_toolchain,
    crate::helpers::ethereal_term_application_toolchain,
    crate::helpers::ethereal_term_ritchie_toolchain,
    crate::helpers::final_destination::application_ethereal_term_final_destination,
    crate::helpers::final_destination::curry_ethereal_term_final_destination,
);

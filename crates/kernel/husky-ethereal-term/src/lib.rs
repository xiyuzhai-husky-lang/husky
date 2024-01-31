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
pub use self::rewrite::*;
pub use self::template_parameter::*;
pub use self::term::*;
pub use self::ty::*;

use self::context::*;
use either::*;
use husky_coword::Ident;
use husky_declarative_signature::*;
use husky_declarative_term::{term::*, *};
use husky_entity_path::*;
use husky_print_utils::p;
use husky_term_prelude::*;
use husky_vfs::*;
use salsa::DebugWithDb;
use smallvec::*;

#[salsa::jar]
pub struct EtherealTermJar(
    // symbol
    SymbolEtherealTerm,
    // - symbols
    EtherealTermSymbols,
    // curry
    term_curry_symbols,
    curry_parameter_count,
    // ritchie
    term_ritchie_symbols,
    term_application_symbols,
    // hole
    RuneEtherealTerm,
    // curry
    CurryEtherealTerm,
    term_curry_from_declarative,
    // curry_parameter_count,
    // ritchie
    RitchieEtherealTerm,
    ethereal_term_ritchie_from_declarative_term_ritchie,
    // abstraction
    AbstractionEtherealTerm,
    // application
    ApplicationEtherealTerm,
    application_expansion_salsa,
    ethereal_term_from_declarative_term_application,
    ethereal_term_application_declarative_ty,
    // - application reduction
    reduce_term_application,
    // - application expansion
    EtherealApplicationArguments,
    // ty as trait associated item
    TypeAsTraitItemEtherealTerm,
    // trait constraint
    EtherealTermTraitConstraint,
    term_menu,
    // other
    ethereal_term_from_declarative_term_explicit_application_or_ritchie_call,
    ethereal_term_from_declarative_term_list,
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
    helpers::final_destination::ethereal_term_application_final_destination,
    helpers::final_destination::ethereal_term_curry_final_destination,
);

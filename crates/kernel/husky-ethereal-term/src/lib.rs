#![feature(result_flattening)]
#![doc = include_str ! ("../README.md")]
#![feature(trait_upcasting)]
#![feature(let_chains)]

// #![deny(unsafe_code, missing_docs, clippy::unwrap_used)]
mod context;
mod conversion;
mod db;
mod error;
mod menu;
mod rewrite;
mod template;
mod term;
#[cfg(test)]
mod tests;
mod trai;
mod ty;
mod utils;

pub use self::context::*;
pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::*;
pub use self::term::*;
pub use self::trai::*;
pub use self::ty::*;

use self::template::*;
#[cfg(test)]
use self::tests::*;
use either::*;
use husky_declarative_term::*;
use husky_entity_path::*;
use husky_print_utils::p;
use husky_term_prelude::*;
use husky_ty_expectation::TermTypeExpectation;
use husky_vfs::*;
use husky_word::Ident;
use salsa::DebugWithDb;
use smallvec::*;

#[salsa::jar(db = EtherealTermDb)]
pub struct EtherealTermJar(
    // symbol
    EtherealTermSymbol,
    // - symbols
    EtherealTermSymbols,
    // curry
    term_curry_symbols,
    total_number_of_curry_parameters,
    // ritchie
    term_ritchie_symbols,
    term_application_symbols,
    // hole
    EtherealTermVariable,
    // curry
    EtherealTermCurry,
    term_curry_from_declarative,
    // total_number_of_curry_parameters,
    // ritchie
    EtherealTermRitchie,
    ethereal_term_ritchie_from_declarative_term_ritchie,
    // abstraction
    EtherealTermAbstraction,
    // application
    EtherealTermApplication,
    application_expansion_salsa,
    ethereal_term_from_declarative_term_application,
    ethereal_term_application_declarative_ty,
    parameter_ty_declarative_term_curry_to_argument_ty_expectation,
    parameter_ty_declarative_term_application_to_argument_ty_expectation,
    // - application reduction
    reduce_term_application,
    // - application expansion
    EtherealApplicationArguments,
    // subentity
    EtherealTermSubentity,
    // as trait subentity
    EtherealTermAsTraitSubentity,
    // trait constraint
    EtherealTermTraitConstraint,
    term_menu,
    // other
    ethereal_term_from_declarative_term_explicit_application_or_ritchie_call,
    ethereal_term_from_declarative_term_list,
    ethereal_term_from_declarative_term_wrapper,
    utils::curry_term_toolchain,
    utils::application_term_toolchain,
    utils::ritchie_term_toolchain,
    // trai
    trai_side_trai_for_ty_impl_blocks_aux,
    ty_side_trai_for_ty_impl_blocks_aux,
    trai_for_type_impl_template_from_impl_block,
    // template
    TemplateParameters,
    ty_path_template_parameters,
);

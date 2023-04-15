#![feature(result_flattening)]
#![doc = include_str ! ("../README.md")]
#![feature(trait_upcasting)]
#![feature(let_chains)]

// #![deny(unsafe_code, missing_docs, clippy::unwrap_used)]
mod card;
mod context;
mod db;
mod error;
pub mod has_ty;
mod menu;
mod rewrite;
mod template;
mod term;
#[cfg(test)]
mod tests;
mod trai;
mod ty;
mod utils;

pub use self::card::*;
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
use husky_entity_path::*;
use husky_print_utils::p;
use husky_raw_term::*;
use husky_term_prelude::*;
use husky_ty_expectation::TermTypeExpectation;
use husky_vfs::*;
use husky_word::Ident;
use salsa::DebugWithDb;
use smallvec::*;

#[salsa::jar(db = TermDb)]
pub struct TermJar(
    // symbol
    TermSymbol,
    // - symbols
    TermSymbols,
    term_curry_symbols,
    term_ritchie_symbols,
    term_application_symbols,
    // hole
    TermPlaceholder,
    // curry
    TermCurry,
    term_curry_from_raw_unchecked,
    check_term_curry_validity,
    // total_number_of_curry_parameters,
    // ritchie
    TermRitchie,
    term_ritchie_from_raw_unchecked,
    check_term_ritchie_validity,
    // abstraction
    TermAbstraction,
    check_term_abstraction_validity,
    // application
    TermApplication,
    check_term_application_validity,
    application_expansion_salsa,
    term_uncheck_from_raw_term_application,
    term_application_raw_ty,
    parameter_ty_raw_term_curry_to_argument_ty_expectation,
    parameter_ty_raw_term_application_to_argument_ty_expectation,
    // - application reduction
    reduce_term_application,
    // - application expansion
    ApplicationArguments,
    // subentity
    TermSubentity,
    check_term_subentity_validity,
    // as trait subentity
    TermAsTraitSubentity,
    check_term_as_trai_subentity_validity,
    // trait constraint
    TermTraitConstraint,
    check_term_trai_constraint_validity,
    term_menu,
    // other
    term_from_raw_term_explicit_application_or_ritchie_call_unchecked,
    term_from_raw_term_list_unchecked,
    // card
    // method ty
    term_application_ty_method_card,
    ty_path_ty_method_cards_aux,
    TypeMethodFnCard,
    TraitForTypeMethodFnCard,
    TypeAssociatedFnCard,
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
    // field
    term_application_field_ty,
);

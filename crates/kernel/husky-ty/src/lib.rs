#![feature(trait_upcasting)]
mod card;
mod checked;
mod db;
mod error;
mod field;
mod has_ty;
mod method;

pub use self::card::*;
pub use self::checked::*;
pub use self::db::*;
pub use self::error::*;
pub use self::field::*;
pub use self::has_ty::*;
pub use self::method::*;

use either::*;
use husky_entity_path::*;
use husky_term::*;
use husky_term_prelude::*;
use husky_ty_expectation::*;
use husky_word::*;
use smallvec::*;

#[salsa::jar(db = TypeDb)]
pub struct TypeJar(
    // cards
    // method ty
    term_application_ty_method_card,
    ty_path_ty_method_cards_aux,
    TypeMethodFnCard,
    TraitForTypeMethodFnCard,
    TypeAssociatedFnCard,
    // field
    term_application_field_ty,
    // checked
    check_term_abstraction_validity,
    check_term_application_validity,
    check_term_curry_validity,
    check_term_ritchie_validity,
    check_term_subentity_validity,
    check_term_as_trai_subentity_validity,
    check_term_trai_constraint_validity,
);

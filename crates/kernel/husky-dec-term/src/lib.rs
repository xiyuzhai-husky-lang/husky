#![feature(if_let_guard)]
#![feature(let_chains)]
mod context;
mod db;
mod error;
pub mod helpers;
pub mod instantiation;
mod menu;
mod rewrite;
pub mod term;

pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::*;

use self::context::*;
use self::term::*;
use husky_coword::*;
use husky_term_prelude::*;
use husky_vfs::Toolchain;

#[salsa::jar]
pub struct DecTermJar(
    SymbolDecTerm,
    DecTermSymbols,
    declarative_term_curry_symbols,
    declarative_term_ritchie_symbols,
    application_declarative_term_symbols,
    RuneDecTerm,
    DecTermRunes,
    declarative_term_curry_placeholders,
    declarative_term_ritchie_runes,
    declarative_term_application_runes,
    CurryDecTerm,
    curry_parameter_count,
    RitchieDecTerm,
    crate::term::abstraction::AbstractionDecTerm,
    crate::term::application::ApplicationDecTerm,
    crate::term::application_or_ritchie_call::ApplicationOrRitchieCallDecTerm,
    crate::term::associated_item::AssociatedItemDecTerm,
    crate::term::ty_as_trai_item::TypeAsTraitItemDecTerm,
    crate::term::constraint::TraitConstraintDecTerm,
    crate::term::list::ListDecTerm,
    declarative_term_menu,
    WrapperDecTerm,
);

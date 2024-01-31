#![feature(if_let_guard)]
#![feature(let_chains)]
mod context;
mod db;
mod error;
pub mod helpers;
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
pub struct DeclarativeTermJar(
    SymbolDeclarativeTerm,
    DeclarativeTermSymbols,
    declarative_term_curry_symbols,
    declarative_term_ritchie_symbols,
    application_declarative_term_symbols,
    RuneDeclarativeTerm,
    DeclarativeTermRunes,
    declarative_term_curry_placeholders,
    declarative_term_ritchie_variables,
    declarative_term_application_variables,
    CurryDeclarativeTerm,
    curry_parameter_count,
    RitchieDeclarativeTerm,
    crate::term::abstraction::AbstractionDeclarativeTerm,
    crate::term::application::ApplicationDeclarativeTerm,
    crate::term::application_or_ritchie_call::ApplicationOrRitchieCallDeclarativeTerm,
    crate::term::associated_item::AssociatedItemDeclarativeTerm,
    crate::term::ty_as_trai_associated_item::TypeAsTraitAssociatedItemDeclarativeTerm,
    crate::term::constraint::TraitConstraintDeclarativeTerm,
    crate::term::list::ListDeclarativeTerm,
    declarative_term_menu,
    WrapperDeclarativeTerm,
);

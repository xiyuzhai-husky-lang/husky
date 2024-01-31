#![feature(if_let_guard)]
#![feature(let_chains)]
mod context;
mod db;
mod error;
pub mod helpers;
mod menu;
mod rewrite;
mod term;

pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::*;
pub use self::term::*;

use self::context::*;
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
    AbstractionDeclarativeTerm,
    ApplicationDeclarativeTerm,
    ApplicationOrRitchieCallDeclarativeTerm,
    AssociatedItemDeclarativeTerm,
    TypeAsTraitAssociatedItemDeclarativeTerm,
    TraitConstraintDeclarativeTerm,
    ListDeclarativeTerm,
    declarative_term_menu,
    WrapperDeclarativeTerm,
);

#![feature(trait_upcasting)]
#![feature(let_chains)]
mod context;
mod db;
mod error;
mod menu;
mod rewrite;
mod term;
mod utils;

pub use self::context::*;
pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::*;
pub use self::term::*;

use husky_term_prelude::*;
use husky_vfs::Toolchain;
use husky_word::*;

#[salsa::jar(db =  DeclarativeTermDb)]
pub struct DeclarativeTermJar(
    DeclarativeTermSymbol,
    DeclarativeTermSymbols,
    raw_term_curry_symbols,
    raw_term_ritchie_symbols,
    raw_term_application_symbols,
    DeclarativeTermPlaceholder,
    DeclarativeTermPlaceholders,
    raw_term_curry_placeholders,
    raw_term_ritchie_variables,
    raw_term_application_variables,
    DeclarativeTermCurry,
    total_number_of_curry_parameters,
    DeclarativeTermRitchie,
    DeclarativeTermAbstraction,
    DeclarativeTermExplicitApplication,
    DeclarativeTermExplicitApplicationOrRitchieCall,
    DeclarativeTermSubentity,
    DeclarativeTermAsTraitSubentity,
    DeclarativeTermTraitConstraint,
    DeclarativeTermList,
    declarative_term_menu,
);

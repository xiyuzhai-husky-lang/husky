#![feature(if_let_guard)]
#![feature(let_chains)]
mod db;
mod error;
mod fmt;
pub mod helpers;
pub mod instantiation;
mod menu;
pub mod name;
mod rewrite;
pub mod term;

pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::*;

use self::term::*;
use husky_coword::*;
use husky_term_prelude::*;
use husky_vfs::Toolchain;

#[salsa::jar]
pub struct DecTermJar(
    DecSymbol,
    DecTermSymbols,
    declarative_term_curry_symbols,
    declarative_term_ritchie_symbols,
    application_declarative_term_symbols,
    DecRune,
    DecTermRunes,
    declarative_term_curry_placeholders,
    declarative_term_ritchie_runes,
    declarative_term_application_runes,
    DecCurry,
    curry_parameter_count,
    DecRitchie,
    crate::term::abstraction::DecAbstraction,
    crate::term::application::DecApplication,
    crate::term::application_or_ritchie_call::DecApplicationOrRitchieCall,
    crate::term::associated_item::DecAssocItem,
    crate::term::ty_as_trai_item::DecTypeAsTraitItem,
    crate::term::constraint::DecTraitConstraint,
    crate::term::list::DecList,
    declarative_term_menu,
    DecWrapper,
);

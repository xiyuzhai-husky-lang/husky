#![feature(trait_upcasting)]
#![feature(let_chains)]
mod context;
mod db;
mod error;
mod menu;
mod rewrite;
mod term;

pub use self::context::*;
pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::*;
pub use self::term::*;

use husky_term_prelude::*;
use husky_vfs::Toolchain;
use husky_word::*;

#[salsa::jar(db =  RawTermDb)]
pub struct RawTermJar(
    RawTermSymbol,
    RawTermCurry,
    total_number_of_curry_parameters,
    RawTermRitchie,
    RawTermAbstraction,
    RawTermApplication,
    RawTermSubentity,
    RawTermAsTraitSubentity,
    RawTermTraitConstraint,
    RawTermInteger128,
    RawTermInteger256,
    RawTermNaturalNumber,
    raw_term_menu,
    is_ty_path_lifetime_ty,
);

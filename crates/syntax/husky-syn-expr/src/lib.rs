#![feature(result_flattening)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
mod context;
mod db;
mod entity_path;
mod error;
mod expr;
pub mod helpers;
mod obelisks;
mod parser;
mod pattern;
mod precedence;
mod range;
mod region;
mod snippet;
mod stmt;
pub mod symbol;
#[cfg(test)]
mod tests;

pub use self::context::*;
pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::obelisks::*;
pub use self::parser::*;
pub use self::pattern::*;
pub use self::range::*;
pub use self::region::*;
pub use self::stmt::*;
pub use self::symbol::*;

use husky_coword::*;
use husky_entity_path::{
    EntityPath, ItemPath, MajorEntityPath, MajorItemPath, PrincipalEntityPath,
};
use husky_entity_syn_tree::{helpers::tokra_region::*, *};
use husky_opr::*;
use husky_regional_token::*;
use husky_term_prelude::*;
use husky_token_data::*;
use precedence::*;
use range::*;
use smallvec::SmallVec;
use snippet::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = SynExprDb)]
pub struct SynExprJar(SynExprRegion, parse_expr_from_snippet, expr_range_region);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPath {
    None,
    Some(EntityPath),
    Uncertain {
        inclination: BaseEntityPathInclination,
    },
    SelfType,
    Err,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPathInclination {
    GlobalValue,
    FunctionOrLocalValue,
    TypeOrVariant,
}

impl BaseEntityPathInclination {
    pub fn from_case(case: IdentCase) -> Self {
        match case {
            IdentCase::SingleCapital | IdentCase::PascalCase => {
                BaseEntityPathInclination::TypeOrVariant
            }
            IdentCase::AllCapital => BaseEntityPathInclination::GlobalValue,
            IdentCase::SnakeCase => BaseEntityPathInclination::FunctionOrLocalValue,
            IdentCase::CamelCase => BaseEntityPathInclination::TypeOrVariant,
        }
    }
}

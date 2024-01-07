#![feature(result_flattening)]
#![feature(let_chains)]
mod context;
mod db;
mod entity_path;
mod error;
mod expr;
pub mod helpers;
mod parser;
mod pattern;
mod range;
mod region;
pub mod snippet;
mod stmt;
pub mod symbol;
mod syndicates;
#[cfg(test)]
mod tests;

pub use self::context::*;
pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::parser::*;
pub use self::pattern::*;
pub use self::range::*;
pub use self::region::*;
pub use self::stmt::*;
pub use self::symbol::*;
pub use self::syndicates::*;

use self::snippet::*;
use husky_coword::*;
use husky_entity_path::{EntityPath, MajorItemPath, PrincipalEntityPath};
use husky_entity_syn_tree::{helpers::tokra_region::*, *};
use husky_regional_token::*;
use husky_syn_opr::*;
use husky_term_prelude::*;
use husky_token_data::*;
use smallvec::SmallVec;

#[salsa::jar(db = SynExprDb)]
pub struct SynExprJar(
    SynExprRegion,
    parse_expr_from_snippet,
    syn_expr_range_region,
);

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

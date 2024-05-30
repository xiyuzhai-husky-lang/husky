#![feature(result_flattening)]
#![feature(let_chains)]
mod context;
pub mod entity_path;
mod error;
mod expr;
pub mod helpers;
pub mod jar;
mod parser;
mod pattern;
mod range;
mod region;
pub mod snippet;
mod stmt;
mod syndicates;
#[cfg(test)]
mod tests;
pub mod variable;

pub use self::context::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::jar::*;
pub use self::parser::*;
pub use self::pattern::*;
pub use self::range::*;
pub use self::region::*;
pub use self::stmt::*;
pub use self::syndicates::*;
pub use self::variable::*;

use self::entity_path::*;
use self::jar::SynExprJar as Jar;
#[cfg(test)]
use self::tests::*;
use husky_coword::*;
use husky_entity_path::path::{major_item::MajorItemPath, EntityPath, PrincipalEntityPath};
use husky_regional_token::*;
use husky_syn_opr::*;
use husky_term_prelude::*;
use husky_token_data::*;
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPath {
    None,
    Some(EntityPath),
    UncertainDueToError {
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

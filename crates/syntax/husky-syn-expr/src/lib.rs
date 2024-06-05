#![feature(result_flattening)]
#![feature(let_chains)]
pub mod context;
pub mod entity_path;
pub mod error;
pub mod expr;
pub mod helpers;
pub mod jar;
pub mod parser;
pub mod pattern;
pub mod range;
pub mod region;
pub mod snippet;
pub mod stmt;
pub mod syndicates;
#[cfg(test)]
mod tests;
pub mod variable;

use self::context::*;
use self::entity_path::*;
use self::error::*;
use self::expr::*;
use self::jar::SynExprJar as Jar;
use self::parser::*;
use self::pattern::*;
use self::region::*;
use self::stmt::*;
use self::syndicates::*;
#[cfg(test)]
use self::tests::*;
use self::variable::*;
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

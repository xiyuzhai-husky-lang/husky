#![feature(trait_upcasting)]
mod ast;
mod children;
mod db;
mod error;
mod helpers;
mod parser;
mod range;
mod sheet;
mod specs;
#[cfg(feature = "test_utils")]
pub mod test_utils;
#[cfg(test)]
mod tests;
mod utils;

pub use self::ast::*;
pub use self::children::*;
pub use self::db::AstDb;
pub use self::error::*;
pub use self::range::*;
pub use self::sheet::*;
pub use self::specs::*;

use self::parser::*;

use husky_coword::*;
use husky_entity_kind::EntityKind;
use husky_entity_path::{ItemPath, TypeVariantPath};

use husky_scope_expr::VisibilityExpr;
use husky_token::{IdentToken, TokenGroupIdx, TokenStreamState, VerticalToken};
use husky_token_data::*;
use husky_vfs::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

#[salsa::jar(db = AstDb)]
pub struct AstJar(ast_sheet, ast_token_idx_range_sheet);

mod context;
mod entrance;
mod error;
mod field;
mod query;
mod stmt;
mod transform;
mod variant;
mod xml;

pub use crate::error::{AstError, AstErrorVariant, AstResult, AstResultArc};
pub use context::*;
pub use entrance::*;
pub use field::*;
use husky_token_storage::TokenIdxRange;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
pub use query::{AstDb, AstText};
pub use stmt::*;
pub use transform::*;
pub use variant::*;
pub use xml::*;

use error::*;
use husky_check_utils::*;
use husky_defn_head::*;
use husky_dev_utils::*;
use husky_entity_kind::*;
use husky_entity_path::EntityPath;
use husky_expr_syntax::*;
use husky_identifier::IdentDict;
use husky_identifier::*;
use husky_init_syntax::InitKind;
use husky_liason_semantics::*;
use husky_opn_syntax::*;
use husky_pattern_syntax::RawPattern;
use husky_print_utils::*;
use husky_term::Ty;
use husky_text::*;
use salsa::DbWithJar;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeprecatedAst {
    pub variant: DeprecatedAstVariant,
    pub range: TextRange,
}

pub struct Ast {
    tokens: TokenIdxRange,
    kind: AstKind,
}

pub enum AstKind {
    Use,
    Comment,
    Stmt,
    DefnHead,
}

pub type AstArena = Arena<Ast>;
pub type AstIdx = ArenaIdx<Ast>;
pub type AstIdxRange = ArenaIdxRange<Ast>;
pub type AstMap<V> = ArenaMap<Ast, V>;

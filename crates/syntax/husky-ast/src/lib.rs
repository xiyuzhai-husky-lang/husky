mod context;
mod defn;
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
pub use defn::*;
pub use entrance::*;
pub use field::*;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstSalsaQueryGroup, AstText};
pub use stmt::*;
pub use transform::*;
pub use variant::*;
pub use xml::*;

use error::*;
use husky_check_utils::*;
use husky_defn_head::*;
use husky_dev_utils::*;
use husky_entity_kind::*;
use husky_entity_path::EntityPathItd;
use husky_expr_syntax::*;
use husky_init_syntax::InitKind;
use husky_liason_semantics::*;
use husky_opn_syntax::*;
use husky_path::PathItd;
use husky_pattern_syntax::RawPattern;
use husky_print_utils::*;
use husky_term::Ty;
use husky_text::*;
use husky_word::*;
use husky_word::{CustomIdentifier, IdentDict, Identifier, Paradigm, StmtKeyword};
use std::sync::Arc;
use timed_salsa::DbWithJar;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub variant: DeprecatedAstVariant,
    pub range: TextRange,
}

#[timed_salsa::jar(db = AstDb)]
pub struct Jar(Defn);

pub trait AstDb: DbWithJar<Jar> {}

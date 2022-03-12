mod decl;
mod impr;
mod parser;

pub(crate) use decl::parse_decl_stmts;
pub use decl::{DeclBranchGroupKind, DeclBranchKind, DeclStmt, DeclStmtKind};
pub(crate) use impr::parse_impr_stmts;
pub use impr::{Boundary, ImprStmt, ImprStmtKind, LoopKind};

use crate::error::{err, not_none};
use crate::expr::{BinaryOpnKind, ExprParser, Opn};
use crate::query::infer::InferQueryGroup;
use crate::SemanticResult;
use crate::*;
use ast::*;
use common::p;
use file::FilePtr;
use scope::{ScopeKind, ScopePtr};
use syntax_types::Opr;
use text::TextRange;
use word::{BuiltinIdentifier, CustomIdentifier};

use self::decl::DeclBranch;

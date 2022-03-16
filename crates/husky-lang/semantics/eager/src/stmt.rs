mod decl;
mod impr;
mod parser;

pub use decl::parse_decl_stmts;
pub use decl::{DeclBranchGroupKind, DeclBranchKind, DeclStmt, DeclStmtKind};
pub use impr::parse_impr_stmts;
pub use impr::{Boundary, ImprStmt, ImprStmtKind, LoopKind};

use self::decl::DeclBranch;
use crate::expr::{EagerExprParser, EagerOpnKind};
use crate::SemanticResult;
use crate::*;
use ast::*;
use common::p;
use file::FilePtr;
use scope::{ScopeKind, ScopePtr};
use semantics_error::{err, not_none};
use syntax_types::Opr;
use text::TextRange;
use word::{BuiltinIdentifier, CustomIdentifier};

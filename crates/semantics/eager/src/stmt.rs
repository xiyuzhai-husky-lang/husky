mod func;
mod parser;
mod proc;

pub use func::parse_decl_stmts;
pub use func::{DeclBranchGroupKind, DeclBranchKind, FuncStmt, FuncStmtVariant};
pub use proc::parse_impr_stmts;
pub use proc::{Boundary, LoopVariant, ProcStmt, ProcStmtVariant};

use crate::expr::EagerExprParser;
use crate::*;
use ast::*;
use entity_route::EntityRoutePtr;
use file::FilePtr;
use print_utils::*;
use semantics_error::{err, not_none};
use text::TextRange;
use word::{CustomIdentifier, RootIdentifier};

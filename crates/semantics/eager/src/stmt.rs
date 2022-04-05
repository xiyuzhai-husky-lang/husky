mod func;
mod parser;
mod proc;

pub use func::parse_decl_stmts;
pub use func::{DeclBranchGroupKind, DeclBranchKind, FuncStmt, FuncStmtKind};
pub use proc::parse_impr_stmts;
pub use proc::{Boundary, LoopKind, ProcStmt, ProcStmtKind};

use crate::expr::EagerExprParser;
use crate::*;
use ast::*;
use entity_route::EntityRoutePtr;
use file::FilePtr;
use print_utils::*;
use semantics_error::{err, not_none};
use text::TextRange;
use word::{BuiltinIdentifier, CustomIdentifier};

use common::*;
use either::Either;
use hir::{PathResolution, Semantics};
use ide_db::{
    file_db::FileID,
    defs::Definition,
    search::{FileReference, UsageSearchResult},
    IdeDatabase,
};
use syntax::{ast, SyntaxElement};

use crate::{
    assist_context::{AssistContext, Assists},
    AssistId, AssistKind,
};

// Assist: inline_local_variable
//
// Inlines a local variable.
//
// ```
// fn main() {
//     let x$0 = 1 + 2;
//     x * 4;
// }
// ```
// ->
// ```
// fn main() {
//     (1 + 2) * 4;
// }
// ```
pub(crate) fn inline_local_variable(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

struct InlineData {
    let_stmt: ast::LetStmt,
    delete_let: bool,
    target: ast::NameOrNameRef,
    references: Vec<FileReference>,
}

fn inline_let(
    sema: &Semantics<IdeDatabase>,
    let_stmt: ast::LetStmt,
    range: TextRange,
    file_id: FileID,
) -> Option<InlineData> {
    todo!()
}

fn inline_usage(
    sema: &Semantics<IdeDatabase>,
    path_expr: ast::PathExpr,
    range: TextRange,
    file_id: FileID,
) -> Option<InlineData> {
    todo!()
}

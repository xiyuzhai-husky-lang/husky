use either::Either;
use hir::{db::HirDatabase, PathResolution, Semantics};
use husky_lang_db::{
    defs::Definition,
    helpers::{insert_use::remove_path_if_in_use_stmt, node_ext::expr_as_name_ref},
    path_transform::PathTransform,
    search::{FileReference, SearchScope},
    vfs::{SourceFileId, SourceFileRange},
    HuskyLangDatabase,
};
use itertools::{izip, Itertools};
use syntax::ast::{self, edit_in_place::Indent, PathExpr};

use crate::{
    assist_context::{AssistContext, Assists},
    AssistId, AssistKind,
};

// Assist: inline_into_callers
//
// Inline a function or method body into all of its callers where possible, creating a `let` statement per parameter
// unless the parameter can be inlined. The parameter will be inlined either if it the supplied argument is a simple local
// or if the parameter is only accessed inside the function body once.
// If all calls can be inlined the function will be removed.
//
// ```
// fn print(_: &str) {}
// fn foo$0(word: &str) {
//     if !word.is_empty() {
//         print(word);
//     }
// }
// fn bar() {
//     foo("안녕하세요");
//     foo("여러분");
// }
// ```
// ->
// ```
// fn print(_: &str) {}
//
// fn bar() {
//     {
//         let word = "안녕하세요";
//         if !word.is_empty() {
//             print(word);
//         }
//     };
//     {
//         let word = "여러분";
//         if !word.is_empty() {
//             print(word);
//         }
//     };
// }
// ```
pub(crate) fn inline_into_callers(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

// Assist: inline_call
//
// Inlines a function or method body creating a `let` statement per parameter unless the parameter
// can be inlined. The parameter will be inlined either if it the supplied argument is a simple local
// or if the parameter is only accessed inside the function body once.
//
// ```
// # //- minicore: option
// fn foo(name: Option<&str>) {
//     let name = name.unwrap$0();
// }
// ```
// ->
// ```
// fn foo(name: Option<&str>) {
//     let name = match name {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         };
// }
// ```
pub(crate) fn inline_call(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

struct CallInfo {
    node: ast::CallableExpr,
    arguments: Vec<ast::Expr>,
    generic_arg_list: Option<ast::GenericArgList>,
}

impl CallInfo {
    fn from_name_ref(name_ref: ast::NameRef) -> Option<CallInfo> {
        todo!()
    }
}

fn get_fn_params(
    db: &dyn HirDatabase,
    function: hir::Function,
    param_list: &ast::ParamList,
) -> Option<Vec<(ast::Pat, Option<ast::Type>, hir::Param)>> {
    todo!()
}

fn inline(
    sema: &Semantics<HuskyLangDatabase>,
    function_def_file_id: SourceFileId,
    function: hir::Function,
    fn_body: &ast::BlockExpr,
    params: &[(ast::Pat, Option<ast::Type>, hir::Param)],
    CallInfo {
        node,
        arguments,
        generic_arg_list,
    }: &CallInfo,
) -> ast::Expr {
    todo!()
}

fn path_expr_as_record_field(usage: &PathExpr) -> Option<ast::RecordExprField> {
    todo!()
}

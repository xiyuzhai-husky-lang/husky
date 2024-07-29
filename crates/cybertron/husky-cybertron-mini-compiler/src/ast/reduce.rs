pub mod opr;

use super::*;
use husky_cybertron::{
    debug::{is_debug, set_debug},
    prelude::*,
    seq::any::AnySeq,
};
use husky_print_utils::p;
use indexmap::IndexMap;

fn add_pre_asts(
    pre_asts: Seq<Option<PreAst>>,
    new_asts: Seq<Option<AstData>>,
) -> Seq<Option<PreAst>> {
    add_pre_ast.apply(pre_asts, new_asts)
}

fn add_pre_ast(pre_ast: Option<PreAst>, new_ast: Option<AstData>) -> Option<PreAst> {
    pre_ast.or(new_ast.map(PreAst::Ast))
}

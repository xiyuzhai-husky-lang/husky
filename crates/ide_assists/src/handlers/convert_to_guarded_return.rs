use std::iter::once;

use syntax::ast::{self, edit::IndentLevel};

use crate::{
    assist_context::{AssistContext, Assists},
    utils::invert_boolean_expression,
    AssistId, AssistKind,
};

pub(crate) fn convert_to_guarded_return(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

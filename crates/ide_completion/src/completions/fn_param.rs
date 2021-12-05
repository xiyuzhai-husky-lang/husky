//! See [`complete_fn_param`].

use rustc_hash::FxHashMap;
use syntax::ast;

use crate::{
    context::{ParamKind, PatternContext},
    CompletionContext, CompletionItem, CompletionItemKind, Completions,
};

/// Complete repeated parameters, both name and type. For example, if all
/// functions in a file have a `spam: &mut Spam` parameter, a completion with
/// `spam: &mut Spam` insert text/label and `spam` lookup string will be
/// suggested.
pub(crate) fn complete_fn_param(acc: &mut Completions, ctx: &CompletionContext) -> Option<()> {
    todo!()
}

fn add_new_item_to_acc(
    ctx: &CompletionContext,
    acc: &mut Completions,
    label: String,
    lookup: String,
) {
    todo!()
}

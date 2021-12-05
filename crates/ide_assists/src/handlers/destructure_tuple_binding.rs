use ide_db::{
    assists::{AssistId, AssistKind},
    defs::Definition,
    search::{FileReference, SearchScope, UsageSearchResult},
};
use itertools::Itertools;
use syntax::{
    ast::{self, FieldExpr, IdentPat, MethodCallExpr},
    TextRange,
};

use crate::assist_context::{AssistBuilder, AssistContext, Assists};

pub(crate) fn destructure_tuple_binding(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    destructure_tuple_binding_impl(acc, ctx, false)
}

pub(crate) fn destructure_tuple_binding_impl(
    acc: &mut Assists,
    ctx: &AssistContext,
    with_sub_pattern: bool,
) -> Option<()> {
    todo!()
}

fn collect_data(ident_pat: IdentPat, ctx: &AssistContext) -> Option<TupleData> {
    todo!()
}

fn generate_name(
    _ctx: &AssistContext,
    index: usize,
    _tuple_name: &str,
    _ident_pat: &IdentPat,
    _usages: &Option<UsageSearchResult>,
) -> String {
    // FIXME: detect if name already used
    format!("_{}", index)
}

enum RefType {
    ReadOnly,
    Mutable,
}
struct TupleData {
    ident_pat: IdentPat,
    // name: String,
    range: TextRange,
    ref_type: Option<RefType>,
    field_names: Vec<String>,
    // field_types: Vec<Type>,
    usages: Option<UsageSearchResult>,
}
fn edit_tuple_assignment(
    ctx: &AssistContext,
    builder: &mut AssistBuilder,
    data: &TupleData,
    in_sub_pattern: bool,
) {
    todo!()
}

fn edit_tuple_usages(
    data: &TupleData,
    builder: &mut AssistBuilder,
    ctx: &AssistContext,
    in_sub_pattern: bool,
) {
    if let Some(usages) = data.usages.as_ref() {
        for (file_id, refs) in usages.iter() {
            builder.edit_file(*file_id);

            for r in refs {
                edit_tuple_usage(ctx, builder, r, data, in_sub_pattern);
            }
        }
    }
}
fn edit_tuple_usage(
    ctx: &AssistContext,
    builder: &mut AssistBuilder,
    usage: &FileReference,
    data: &TupleData,
    in_sub_pattern: bool,
) {
    match detect_tuple_index(usage, data) {
        Some(index) => edit_tuple_field_usage(ctx, builder, data, index),
        None => {
            if in_sub_pattern {
                cov_mark::hit!(destructure_tuple_call_with_subpattern);
                return;
            }

            // no index access -> make invalid -> requires handling by user
            // -> put usage in block comment
            //
            // Note: For macro invocations this might result in still valid code:
            //   When a macro accepts the tuple as argument, as well as no arguments at all,
            //   uncommenting the tuple still leaves the macro call working (see `tests::in_macro_call::empty_macro`).
            //   But this is an unlikely case. Usually the resulting macro call will become erroneous.
            builder.insert(usage.range.start(), "/*");
            builder.insert(usage.range.end(), "*/");
        }
    }
}

fn edit_tuple_field_usage(
    ctx: &AssistContext,
    builder: &mut AssistBuilder,
    data: &TupleData,
    index: TupleIndex,
) {
    let field_name = &data.field_names[index.index];

    if data.ref_type.is_some() {
        let ref_data = handle_ref_field_usage(ctx, &index.field_expr);
        builder.replace(ref_data.range, ref_data.format(field_name));
    } else {
        builder.replace(index.range, field_name);
    }
}
struct TupleIndex {
    index: usize,
    range: TextRange,
    field_expr: FieldExpr,
}
fn detect_tuple_index(usage: &FileReference, data: &TupleData) -> Option<TupleIndex> {
    todo!()
}

struct RefData {
    range: TextRange,
    needs_deref: bool,
    needs_parentheses: bool,
}
impl RefData {
    fn format(&self, field_name: &str) -> String {
        match (self.needs_deref, self.needs_parentheses) {
            (true, true) => format!("(*{})", field_name),
            (true, false) => format!("*{}", field_name),
            (false, true) => format!("({})", field_name),
            (false, false) => field_name.to_string(),
        }
    }
}
fn handle_ref_field_usage(ctx: &AssistContext, field_expr: &FieldExpr) -> RefData {
    todo!()
}

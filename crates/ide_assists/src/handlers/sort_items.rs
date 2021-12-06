use std::cmp::Ordering;

use itertools::Itertools;

use common::*;

use syntax::ast;

use crate::{utils::get_methods, AssistContext, AssistId, AssistKind, Assists};

pub(crate) fn sort_items(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

fn add_sort_field_list_assist(acc: &mut Assists, field_list: Option<ast::FieldList>) -> Option<()> {
    todo!()
}

fn add_sort_methods_assist(acc: &mut Assists, item_list: ast::AssocItemList) -> Option<()> {
    todo!()
}

fn add_sort_fields_assist(
    acc: &mut Assists,
    record_field_list: ast::RecordFieldList,
) -> Option<()> {
    todo!()
}

fn add_sort_variants_assist(acc: &mut Assists, variant_list: ast::VariantList) -> Option<()> {
    todo!()
}

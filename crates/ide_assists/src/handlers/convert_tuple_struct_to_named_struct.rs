use either::Either;
use ide_db::defs::{Definition, NameRefClass};
use syntax::{
    ast::{self},
    SyntaxNode,
};

use crate::{assist_context::AssistBuilder, AssistContext, AssistId, AssistKind, Assists};

pub(crate) fn convert_tuple_struct_to_named_struct(
    acc: &mut Assists,
    ctx: &AssistContext,
) -> Option<()> {
    todo!()
}

fn edit_struct_def(
    ctx: &AssistContext,
    edit: &mut AssistBuilder,
    strukt: &Either<ast::Struct, ast::Variant>,
    tuple_fields: ast::TupleFieldList,
    names: Vec<ast::Name>,
) {
    todo!()
}

fn edit_struct_references(
    ctx: &AssistContext,
    edit: &mut AssistBuilder,
    strukt: Either<hir::Struct, hir::Variant>,
    names: &[ast::Name],
) {
    todo!()
}

fn edit_field_references(
    ctx: &AssistContext,
    edit: &mut AssistBuilder,
    fields: impl Iterator<Item = ast::TupleField>,
    names: &[ast::Name],
) {
    todo!()
}

fn generate_names(fields: impl Iterator<Item = ast::TupleField>) -> Vec<ast::Name> {
    todo!()
}

use std::iter;

use either::Either;
use hir::{Module, ModuleDef, Name, Variant};
use ide_db::{
    defs::Definition,
    helpers::{
        insert_use::{insert_use, ImportScope, InsertUseConfig},
        mod_path_to_ast,
    },
    search::FileReference,
    IdeDatabase,
};
use itertools::Itertools;
use rustc_hash::FxHashSet;
use syntax::{
    ast::{self, edit::IndentLevel, edit_in_place::Indent},
    SyntaxKind::*,
    SyntaxNode,
};

use crate::{assist_context::AssistBuilder, AssistContext, AssistId, AssistKind, Assists};

// Assist: extract_struct_from_enum_variant
//
// Extracts a struct from enum variant.
//
// ```
// enum A { $0One(u32, u32) }
// ```
// ->
// ```
// struct One(u32, u32);
//
// enum A { One(One) }
// ```
pub(crate) fn extract_struct_from_enum_variant(
    acc: &mut Assists,
    ctx: &AssistContext,
) -> Option<()> {
    todo!()
}

fn extract_field_list_if_applicable(
    variant: &ast::Variant,
) -> Option<Either<ast::RecordFieldList, ast::TupleFieldList>> {
    todo!()
}

fn existing_definition(db: &IdeDatabase, variant_name: &ast::Name, variant: &Variant) -> bool {
    todo!()
}

fn create_struct_def(
    variant_name: ast::Name,
    variant: &ast::Variant,
    field_list: &Either<ast::RecordFieldList, ast::TupleFieldList>,
    enum_: &ast::Enum,
) -> ast::Struct {
    todo!()
}

fn update_variant(variant: &ast::Variant, generic: Option<ast::GenericParamList>) -> Option<()> {
    todo!()
}

fn apply_references(
    insert_use_cfg: InsertUseConfig,
    segment: ast::PathSegment,
    node: SyntaxNode,
    import: Option<(ImportScope, hir::ModPath)>,
) {
    todo!()
}

fn process_references(
    ctx: &AssistContext,
    builder: &mut AssistBuilder,
    visited_modules: &mut FxHashSet<Module>,
    enum_module_def: &ModuleDef,
    variant_hir_name: &Name,
    refs: Vec<FileReference>,
) -> Vec<(
    ast::PathSegment,
    SyntaxNode,
    Option<(ImportScope, hir::ModPath)>,
)> {
    todo!()
}

fn reference_to_node(
    sema: &hir::Semantics<IdeDatabase>,
    reference: FileReference,
) -> Option<(ast::PathSegment, SyntaxNode, hir::Module)> {
    todo!()
}

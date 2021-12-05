//! Assorted functions shared by several assists.

use std::ops;

use itertools::Itertools;

use hir::{db::HirDatabase, HasSource, HirDisplay};
use ide_db::{
    helpers::FamousDefs, helpers::SnippetCap, path_transform::PathTransform, RootDatabase,
};
use stdx::format_to;
use syntax::{
    ast, Direction, SingleFileParseTree, SmolStr, SyntaxKind::*, SyntaxNode, TextRange, TextSize,
};

use crate::assist_context::{AssistBuilder, AssistContext};

pub(crate) mod suggest_name;

pub(crate) fn unwrap_trivial_block(block_expr: ast::BlockExpr) -> ast::Expr {
    todo!()
}

pub fn extract_trivial_expression(block_expr: &ast::BlockExpr) -> Option<ast::Expr> {
    todo!()
}

/// This is a method with a heuristics to support test methods annotated with custom test annotations, such as
/// `#[test_case(...)]`, `#[tokio::test]` and similar.
/// Also a regular `#[test]` annotation is supported.
///
/// It may produce false positives, for example, `#[wasm_bindgen_test]` requires a different command to run the test,
/// but it's better than not to have the runnables for the tests at all.
pub fn test_related_attribute(fn_def: &ast::Fn) -> Option<ast::Attr> {
    todo!()
}

#[derive(Copy, Clone, PartialEq)]
pub enum DefaultMethods {
    Only,
    No,
}

pub fn filter_assoc_items(
    db: &RootDatabase,
    items: &[hir::AssocItem],
    default_methods: DefaultMethods,
) -> Vec<ast::AssocItem> {
    todo!()
}

pub fn add_trait_assoc_items_to_impl(
    sema: &hir::Semantics<ide_db::RootDatabase>,
    items: Vec<ast::AssocItem>,
    trait_: hir::Trait,
    impl_: ast::Impl,
    target_scope: hir::SemanticsScope,
) -> (ast::Impl, ast::AssocItem) {
    todo!()
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Cursor<'a> {
    Replace(&'a SyntaxNode),
    Before(&'a SyntaxNode),
}

impl<'a> Cursor<'a> {
    fn node(self) -> &'a SyntaxNode {
        match self {
            Cursor::Replace(node) | Cursor::Before(node) => node,
        }
    }
}

pub(crate) fn render_snippet(_cap: SnippetCap, node: &SyntaxNode, cursor: Cursor) -> String {
    todo!()
}

pub(crate) fn vis_offset(node: &SyntaxNode) -> TextSize {
    todo!()
}

pub(crate) fn invert_boolean_expression(expr: ast::Expr) -> ast::Expr {
    todo!()
}

fn invert_special_case(expr: &ast::Expr) -> Option<ast::Expr> {
    todo!()
}

pub(crate) fn next_prev() -> impl Iterator<Item = Direction> {
    [Direction::Next, Direction::Prev].into_iter()
}

pub(crate) fn does_pat_match_variant(pat: &ast::Pat, var: &ast::Pat) -> bool {
    todo!()
}

pub(crate) fn does_nested_pattern(pat: &ast::Pat) -> bool {
    let depth = calc_depth(pat, 0);

    if 1 < depth {
        return true;
    }
    false
}

fn calc_depth(pat: &ast::Pat, depth: usize) -> usize {
    todo!()
}

// Uses a syntax-driven approach to find any impl blocks for the struct that
// exist within the module/file
//
// Returns `None` if we've found an existing fn
//
// FIXME: change the new fn checking to a more semantic approach when that's more
// viable (e.g. we process proc macros, etc)
// FIXME: this partially overlaps with `find_impl_block_*`
pub(crate) fn find_struct_impl(
    ctx: &AssistContext,
    adt: &ast::Adt,
    name: &str,
) -> Option<Option<ast::Impl>> {
    todo!()
}

fn has_fn(imp: &ast::Impl, rhs_name: &str) -> bool {
    todo!()
}

/// Find the start of the `impl` block for the given `ast::Impl`.
//
// FIXME: this partially overlaps with `find_struct_impl`
pub(crate) fn find_impl_block_start(impl_def: ast::Impl, buf: &mut String) -> Option<TextSize> {
    todo!()
}

/// Find the end of the `impl` block for the given `ast::Impl`.
//
// FIXME: this partially overlaps with `find_struct_impl`
pub(crate) fn find_impl_block_end(impl_def: ast::Impl, buf: &mut String) -> Option<TextSize> {
    todo!()
}

// Generates the surrounding `impl Type { <code> }` including type and lifetime
// parameters
pub(crate) fn generate_impl_text(adt: &ast::Adt, code: &str) -> String {
    todo!()
}

// Generates the surrounding `impl <trait> for Type { <code> }` including type
// and lifetime parameters
pub(crate) fn generate_trait_impl_text(adt: &ast::Adt, trait_text: &str, code: &str) -> String {
    todo!()
}

fn generate_impl_text_inner(adt: &ast::Adt, trait_text: Option<&str>, code: &str) -> String {
    todo!()
}

pub(crate) fn add_method_to_adt(
    builder: &mut AssistBuilder,
    adt: &ast::Adt,
    impl_def: Option<ast::Impl>,
    method: &str,
) {
    todo!()
}

#[derive(Debug)]
pub(crate) struct ReferenceConversion {
    conversion: ReferenceConversionType,
    ty: hir::Type,
}

#[derive(Debug)]
enum ReferenceConversionType {
    // reference can be stripped if the type is Copy
    Copy,
    // &String -> &str
    AsRefStr,
    // &Vec<T> -> &[T]
    AsRefSlice,
    // &Box<T> -> &T
    Dereferenced,
    // &Option<T> -> Option<&T>
    Option,
    // &Result<T, E> -> Result<&T, &E>
    Result,
}

impl ReferenceConversion {
    pub(crate) fn convert_type(&self, db: &dyn HirDatabase) -> String {
        todo!()
    }

    pub(crate) fn getter(&self, field_name: String) -> String {
        match self.conversion {
            ReferenceConversionType::Copy => format!("self.{}", field_name),
            ReferenceConversionType::AsRefStr
            | ReferenceConversionType::AsRefSlice
            | ReferenceConversionType::Dereferenced
            | ReferenceConversionType::Option
            | ReferenceConversionType::Result => format!("self.{}.as_ref()", field_name),
        }
    }
}

// FIXME: It should return a new hir::Type, but currently constructing new types is too cumbersome
//        and all users of this function operate on string type names, so they can do the conversion
//        itself themselves.
pub(crate) fn convert_reference_type(
    ty: hir::Type,
    db: &RootDatabase,
    famous_defs: &FamousDefs,
) -> Option<ReferenceConversion> {
    handle_copy(&ty, db)
        .or_else(|| handle_as_ref_str(&ty, db, famous_defs))
        .or_else(|| handle_as_ref_slice(&ty, db, famous_defs))
        .or_else(|| handle_dereferenced(&ty, db, famous_defs))
        .or_else(|| handle_option_as_ref(&ty, db, famous_defs))
        .or_else(|| handle_result_as_ref(&ty, db, famous_defs))
        .map(|conversion| ReferenceConversion { ty, conversion })
}

fn handle_copy(ty: &hir::Type, db: &dyn HirDatabase) -> Option<ReferenceConversionType> {
    todo!()
}

fn handle_as_ref_str(
    ty: &hir::Type,
    db: &dyn HirDatabase,
    famous_defs: &FamousDefs,
) -> Option<ReferenceConversionType> {
    todo!()
}

fn handle_as_ref_slice(
    ty: &hir::Type,
    db: &dyn HirDatabase,
    famous_defs: &FamousDefs,
) -> Option<ReferenceConversionType> {
    todo!()
}

fn handle_dereferenced(
    ty: &hir::Type,
    db: &dyn HirDatabase,
    famous_defs: &FamousDefs,
) -> Option<ReferenceConversionType> {
    todo!()
}

fn handle_option_as_ref(
    ty: &hir::Type,
    db: &dyn HirDatabase,
    famous_defs: &FamousDefs,
) -> Option<ReferenceConversionType> {
    todo!()
}

fn handle_result_as_ref(
    ty: &hir::Type,
    db: &dyn HirDatabase,
    famous_defs: &FamousDefs,
) -> Option<ReferenceConversionType> {
    todo!()
}

pub(crate) fn get_methods(items: &ast::AssocItemList) -> Vec<ast::Fn> {
    todo!()
}

/// Trim(remove leading and trailing whitespace) `initial_range` in `source_file`, return the trimmed range.
pub(crate) fn trimmed_text_range(
    source_file: &SingleFileParseTree,
    initial_range: TextRange,
) -> TextRange {
    todo!()
}

/// Convert a list of function params to a list of arguments that can be passed
/// into a function call.
pub(crate) fn convert_param_list_to_arg_list(list: ast::ParamList) -> ast::ArgList {
    todo!()
}

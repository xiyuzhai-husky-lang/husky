use std::cmp::Ordering;

use hir::Semantics;
use syntax::{
    ast::{self, PathSegmentKind},
    SyntaxNode, SyntaxToken,
};

use crate::{
    helpers::merge_imports::{
        common_prefix, eq_attrs, eq_visibility, try_merge_imports, use_tree_path_cmp, MergeBehavior,
    },
    IdeDatabase,
};

pub use hir::PrefixKind;

/// How imports should be grouped into use statements.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImportGranularity {
    /// Do not change the granularity of any imports and preserve the original structure written by the developer.
    Preserve,
    /// Merge imports from the same crate into a single use statement.
    Crate,
    /// Merge imports from the same module into a single use statement.
    Module,
    /// Flatten imports so that each has its own use statement.
    Item,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InsertUseConfig {
    pub granularity: ImportGranularity,
    pub enforce_granularity: bool,
    pub prefix_kind: PrefixKind,
    pub group: bool,
    pub skip_glob_imports: bool,
}

#[derive(Debug, Clone)]
pub enum ImportScope {
    File(ast::SingleFileParseTree),
    Module(ast::ItemList),
    Block(ast::StmtList),
}

impl ImportScope {
    /// Determines the containing syntax node in which to insert a `use` statement affecting `position`.
    /// Returns the original source node inside attributes.
    pub fn find_insert_use_container(
        position: &SyntaxNode,
        sema: &Semantics<'_, IdeDatabase>,
    ) -> Option<Self> {
        todo!()
    }

    pub fn as_syntax_node(&self) -> &SyntaxNode {
        todo!()
    }

    pub fn clone_for_update(&self) -> Self {
        todo!()
    }
}

/// Insert an import path into the given file/node. A `merge` value of none indicates that no import merging is allowed to occur.
pub fn insert_use(scope: &ImportScope, path: ast::Path, cfg: &InsertUseConfig) {
    todo!()
}

pub fn remove_path_if_in_use_stmt(path: &ast::Path) {
    todo!()
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum ImportGroup {
    // the order here defines the order of new group inserts
    Std,
    ExternCrate,
    ThisCrate,
    ThisModule,
    SuperModule,
}

impl ImportGroup {
    fn new(path: &ast::Path) -> ImportGroup {
        todo!()
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
enum ImportGranularityGuess {
    Unknown,
    Item,
    Module,
    ModuleOrItem,
    Crate,
    CrateOrModule,
}

fn guess_granularity_from_scope(scope: &ImportScope) -> ImportGranularityGuess {
    todo!()
}

fn insert_use_(
    scope: &ImportScope,
    insert_path: &ast::Path,
    group_imports: bool,
    use_item: ast::Use,
) {
    todo!()
}

fn is_inner_attribute(node: SyntaxNode) -> bool {
    todo!()
}

fn is_inner_comment(token: SyntaxToken) -> bool {
    todo!()
}

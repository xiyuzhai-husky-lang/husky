//! See `CompletionContext` structure.

use common::*;

use base_db::SourceDatabaseExt;
use hir::{Local, Name, ScopeDef, Semantics, SemanticsScope, Type};
use ide_db::{
    active_parameter::ActiveParameter,
    base_db::{FilePosition, SourceDatabase},
    IdeDatabase,
};
use syntax::{
    ast,
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken,
};
use text_edit::Indel;

use crate::CompletionConfig;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum PatternRefutability {
    Refutable,
    Irrefutable,
}

#[derive(Debug)]
pub(super) enum PathKind {
    Expr,
    Type,
}
pub struct CompletionContext<'a> {
    phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Debug)]
pub(crate) struct PathCompletionContext {
    /// If this is a call with () already there
    call_kind: Option<CallKind>,
    /// A single-indent path, like `foo`. `::foo` should not be considered a trivial path.
    pub(super) is_trivial_path: bool,
    /// If not a trivial path, the prefix (qualifier).
    pub(super) qualifier: Option<ast::Path>,
    /// Whether the qualifier comes from a use tree parent or not
    pub(super) use_tree_parent: bool,
    pub(super) kind: Option<PathKind>,
    /// Whether the path segment has type args or not.
    pub(super) has_type_args: bool,
    /// `true` if we are a statement or a last expr in the block.
    pub(super) can_be_stmt: bool,
    pub(super) in_loop_body: bool,
}

#[derive(Debug)]
pub(super) struct PatternContext {
    pub(super) refutability: PatternRefutability,
    pub(super) is_param: Option<ParamKind>,
    pub(super) has_type_ascription: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum CallKind {
    Pat,
    Mac,
    Expr,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum ParamKind {
    Function,
    Closure,
}
fn find_node_with_range(syntax: &SyntaxNode, range: TextRange) -> Option<syntax::SyntaxNode> {
    todo!()
}

fn path_or_use_tree_qualifier(path: &ast::Path) -> Option<(ast::Path, bool)> {
    todo!()
}

fn has_ref(token: &SyntaxToken) -> bool {
    todo!()
}

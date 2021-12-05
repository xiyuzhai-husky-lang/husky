//! Handle syntactic aspects of merging UseTrees.
use std::cmp::Ordering;

use itertools::{EitherOrBoth, Itertools};
use syntax::ast::{self, PathSegmentKind};

use crate::helpers::node_ext::vis_eq;

/// What type of merges are allowed.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MergeBehavior {
    /// Merge imports from the same crate into a single use statement.
    Crate,
    /// Merge imports from the same module into a single use statement.
    Module,
}

impl MergeBehavior {
    fn is_tree_allowed(&self, tree: &ast::UseTree) -> bool {
        todo!()
    }
}

pub fn try_merge_imports(
    lhs: &ast::Use,
    rhs: &ast::Use,
    merge_behavior: MergeBehavior,
) -> Option<ast::Use> {
    todo!()
}

pub fn try_merge_trees(
    lhs: &ast::UseTree,
    rhs: &ast::UseTree,
    merge: MergeBehavior,
) -> Option<ast::UseTree> {
    todo!()
}

fn try_merge_trees_mut(lhs: &ast::UseTree, rhs: &ast::UseTree, merge: MergeBehavior) -> Option<()> {
    todo!()
}

/// Recursively merges rhs to lhs
#[must_use]
fn recursive_merge(lhs: &ast::UseTree, rhs: &ast::UseTree, merge: MergeBehavior) -> Option<()> {
    todo!()
}

/// Traverses both paths until they differ, returning the common prefix of both.
pub fn common_prefix(lhs: &ast::Path, rhs: &ast::Path) -> Option<(ast::Path, ast::Path)> {
    todo!()
}

/// Orders paths in the following way:
/// the sole self token comes first, after that come uppercase identifiers, then lowercase identifiers
// FIXME: rustfmt sorts lowercase idents before uppercase, in general we want to have the same ordering rustfmt has
// which is `self` and `super` first, then identifier imports with lowercase ones first, then glob imports and at last list imports.
// Example foo::{self, foo, baz, Baz, Qux, *, {Bar}}
fn path_cmp_for_sort(a: Option<ast::Path>, b: Option<ast::Path>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(ref a), Some(ref b)) => match (path_is_self(a), path_is_self(b)) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => path_cmp_short(a, b),
        },
    }
}

/// Path comparison func for binary searching for merging.
fn path_cmp_bin_search(lhs: Option<ast::Path>, rhs: Option<&ast::Path>) -> Ordering {
    todo!()
}

/// Short circuiting comparison, if both paths are equal until one of them ends they are considered
/// equal
fn path_cmp_short(a: &ast::Path, b: &ast::Path) -> Ordering {
    todo!()
}

/// Compares two paths, if one ends earlier than the other the has_tl parameters decide which is
/// greater as a a path that has a tree list should be greater, while one that just ends without
/// a tree list should be considered less.
pub(super) fn use_tree_path_cmp(
    a: &ast::Path,
    a_has_tl: bool,
    b: &ast::Path,
    b_has_tl: bool,
) -> Ordering {
    todo!()
}

fn path_segment_cmp(a: &ast::PathSegment, b: &ast::PathSegment) -> Ordering {
    todo!()
}

pub fn eq_visibility(vis0: Option<ast::Visibility>, vis1: Option<ast::Visibility>) -> bool {
    match (vis0, vis1) {
        (None, None) => true,
        (Some(vis0), Some(vis1)) => vis_eq(&vis0, &vis1),
        _ => false,
    }
}

pub fn eq_attrs(
    attrs0: impl Iterator<Item = ast::Attr>,
    attrs1: impl Iterator<Item = ast::Attr>,
) -> bool {
    todo!()
}

fn path_is_self(path: &ast::Path) -> bool {
    todo!()
}

fn path_len(path: ast::Path) -> usize {
    todo!()
}

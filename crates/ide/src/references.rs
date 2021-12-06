//! This module implements a reference search.
//! First, the element at the cursor position must be either an `ast::Name`
//! or `ast::NameRef`. If it's an `ast::NameRef`, at the classification step we
//! try to resolve the direct tree parent of this element, otherwise we
//! already have a definition and just need to get its HIR together with
//! some information that is needed for further steps of searching.
//! After that, we collect files that might contain references and look
//! for text occurrences of the identifier. If there's an `ast::NameRef`
//! at the index that the match starts at and its tree parent is
//! resolved to the search element definition, we get a reference.

use common::*;

use hir::{PathResolution, Semantics};
use ide_db::{
    base_db::FileID,
    defs::{Definition, NameClass, NameRefClass},
    search::{ReferenceCategory, SearchScope, UsageSearchResult},
    IdeDatabase,
};
use rustc_hash::FxHashMap;
use syntax::{ast, SyntaxNode};

use crate::{FilePosition, NavigationTarget, TryToNav};

#[derive(Debug, Clone)]
pub struct ReferenceSearchResult {
    pub declaration: Option<Declaration>,
    pub references: FxHashMap<FileID, Vec<(TextRange, Option<ReferenceCategory>)>>,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub nav: NavigationTarget,
    pub is_mut: bool,
}

// Feature: Find All References
//
// Shows all references of the item at the cursor location
//
// |===
// | Editor  | Shortcut
//
// | VS Code | kbd:[Shift+Alt+F12]
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113020670-b7c34f00-917a-11eb-8003-370ac5f2b3cb.gif[]
pub(crate) fn find_all_refs(
    sema: &Semantics<IdeDatabase>,
    position: FilePosition,
    search_scope: Option<SearchScope>,
) -> Option<Vec<ReferenceSearchResult>> {
    todo!()
}

pub(crate) fn decl_mutability(def: &Definition, syntax: &SyntaxNode, range: TextRange) -> bool {
    todo!()
}

/// Filter out all non-literal usages for adt-defs
fn retain_adt_literal_usages(
    usages: &mut UsageSearchResult,
    def: Definition,
    sema: &Semantics<IdeDatabase>,
) {
    todo!()
}

/// Returns `Some` if the cursor is at a position for an item to search for all its constructor/literal usages
fn name_for_constructor_search(syntax: &SyntaxNode, position: FilePosition) -> Option<ast::Name> {
    todo!()
}

fn is_enum_lit_name_ref(
    sema: &Semantics<IdeDatabase>,
    enum_: hir::Enum,
    name_ref: &ast::NameRef,
) -> bool {
    todo!()
}

fn path_ends_with(path: Option<ast::Path>, name_ref: &ast::NameRef) -> bool {
    todo!()
}

fn is_lit_name_ref(name_ref: &ast::NameRef) -> bool {
    todo!()
}

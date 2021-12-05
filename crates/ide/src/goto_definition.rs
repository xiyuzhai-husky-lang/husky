use std::convert::TryInto;

use crate::{doc_links::token_as_doc_comment, FilePosition, NavigationTarget, RangeInfo, TryToNav};
use hir::{AsAssocItem, Semantics};
use ide_db::{
    base_db::{AnchoredPath, FileID, FileLoader},
    defs::Definition,
    helpers::pick_best_token,
    RootDatabase,
};
use itertools::Itertools;
use syntax::{ast, SyntaxKind::*, SyntaxToken, TextRange};

// Feature: Go to Definition
//
// Navigates to the definition of an identifier.
//
// |===
// | Editor  | Shortcut
//
// | VS Code | kbd:[F12]
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113065563-025fbe00-91b1-11eb-83e4-a5a703610b23.gif[]
pub(crate) fn goto_definition(
    db: &RootDatabase,
    position: FilePosition,
) -> Option<RangeInfo<Vec<NavigationTarget>>> {
    todo!()
}

fn try_lookup_include_path(
    sema: &Semantics<RootDatabase>,
    tt: ast::TokenTree,
    token: SyntaxToken,
    file_id: FileID,
) -> Option<Vec<NavigationTarget>> {
    todo!()
}

/// finds the trait definition of an impl'd item
/// e.g.
/// ```rust
/// trait A { fn a(); }
/// struct S;
/// impl A for S { fn a(); } // <-- on this function, will get the location of a() in the trait
/// ```
fn try_find_trait_item_definition(
    db: &RootDatabase,
    def: &Definition,
) -> Option<Vec<NavigationTarget>> {
    todo!()
}

fn def_to_nav(db: &RootDatabase, def: Definition) -> Vec<NavigationTarget> {
    def.try_to_nav(db).map(|it| vec![it]).unwrap_or_default()
}

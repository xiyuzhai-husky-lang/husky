use hir::{AsAssocItem, Impl, Semantics};
use ide_db::{
    defs::{Definition, NameClass, NameRefClass},
    helpers::pick_best_token,
    IdeDatabase,
};
use itertools::Itertools;
use syntax::{ast, SyntaxKind::*};

use crate::{FilePosition, NavigationTarget, RangeInfo, TryToNav};

// Feature: Go to Implementation
//
// Navigates to the impl block of structs, enums or traits. Also implemented as a code lens.
//
// |===
// | Editor  | Shortcut
//
// | VS Code | kbd:[Ctrl+F12]
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113065566-02f85480-91b1-11eb-9288-aaad8abd8841.gif[]
pub(crate) fn goto_implementation(
    db: &IdeDatabase,
    position: FilePosition,
) -> Option<RangeInfo<Vec<NavigationTarget>>> {
    todo!()
}

fn impls_for_ty(sema: &Semantics<IdeDatabase>, ty: hir::Type) -> Vec<NavigationTarget> {
    todo!()
}

fn impls_for_trait(sema: &Semantics<IdeDatabase>, trait_: hir::Trait) -> Vec<NavigationTarget> {
    todo!()
}

fn impls_for_trait_item(
    sema: &Semantics<IdeDatabase>,
    trait_: hir::Trait,
    fun_name: hir::Name,
) -> Vec<NavigationTarget> {
    todo!()
}

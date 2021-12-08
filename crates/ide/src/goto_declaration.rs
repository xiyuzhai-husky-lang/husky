use hir::Semantics;
use husky_lang_db::{
    defs::{Definition, NameClass, NameRefClass},
    HuskyLangDatabase,
};
use syntax::{ast, SyntaxKind::*};

use crate::{FilePosition, NavigationTarget, RangeInfo};

// Feature: Go to Declaration
//
// Navigates to the declaration of an identifier.
pub(crate) fn goto_declaration(
    db: &HuskyLangDatabase,
    position: FilePosition,
) -> Option<RangeInfo<Vec<NavigationTarget>>> {
    todo!()
}

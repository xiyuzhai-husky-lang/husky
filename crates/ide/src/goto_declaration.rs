use hir::Semantics;
use ide_db::{
    defs::{Definition, NameClass, NameRefClass},
    IdeDatabase,
};
use syntax::{ast, SyntaxKind::*};

use crate::{FilePosition, NavigationTarget, RangeInfo};

// Feature: Go to Declaration
//
// Navigates to the declaration of an identifier.
pub(crate) fn goto_declaration(
    db: &IdeDatabase,
    position: FilePosition,
) -> Option<RangeInfo<Vec<NavigationTarget>>> {
    todo!()
}

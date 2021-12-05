use hir::Semantics;
use ide_db::{
    defs::{Definition, NameClass, NameRefClass},
    RootDatabase,
};
use syntax::{ast, SyntaxKind::*};

use crate::{FilePosition, NavigationTarget, RangeInfo};

// Feature: Go to Declaration
//
// Navigates to the declaration of an identifier.
pub(crate) fn goto_declaration(
    db: &RootDatabase,
    position: FilePosition,
) -> Option<RangeInfo<Vec<NavigationTarget>>> {
    todo!()
}

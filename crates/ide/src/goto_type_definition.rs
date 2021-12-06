use ide_db::{defs::Definition, helpers::pick_best_token, IdeDatabase};
use syntax::{ast, SyntaxKind::*, SyntaxToken};

use crate::{FilePosition, NavigationTarget, RangeInfo, TryToNav};

// Feature: Go to Type Definition
//
// Navigates to the type of an identifier.
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Go to Type Definition*
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113020657-b560f500-917a-11eb-9007-0f809733a338.gif[]
pub(crate) fn goto_type_definition(
    db: &IdeDatabase,
    position: FilePosition,
) -> Option<RangeInfo<Vec<NavigationTarget>>> {
    todo!()
}

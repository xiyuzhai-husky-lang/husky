use husky_lang_db::{defs::Definition, helpers::pick_best_token, HuskyLangDatabase};
use syntax::{ast, SyntaxKind::*, SyntaxToken};

use crate::{NavigationTarget, RangeInfo, SourceFilePosition, TryToNav};

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
    db: &HuskyLangDatabase,
    position: SourceFilePosition,
) -> Option<RangeInfo<Vec<NavigationTarget>>> {
    todo!()
}

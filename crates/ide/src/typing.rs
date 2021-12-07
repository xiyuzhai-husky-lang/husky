//! This module handles auto-magic editing actions applied together with users
//! edits. For example, if the user typed
//!
//! ```text
//!     foo
//!         .bar()
//!         .baz()
//!     |   // <- cursor is here
//! ```
//!
//! and types `.` next, we want to indent the dot.
//!
//! Language server executes such typing assists synchronously. That is, they
//! block user's typing and should be pretty fast for this reason!

mod on_enter;

use common::*;

use ide_db::{
    file_db::{FileDatabase, FilePosition},
    IdeDatabase,
};
use syntax::{
    ast::{self, edit::IndentLevel},
    ParseResult, SingleFileParseTree, SyntaxKind,
};

use text_edit::{Indel, TextEdit};

use crate::SourceChange;

pub(crate) use on_enter::on_enter;

// Don't forget to add new trigger characters to `server_capabilities` in `caps.rs`.
pub(crate) const TRIGGER_CHARS: &str = ".=>{";

// Feature: On Typing Assists
//
// Some features trigger on typing certain characters:
//
// - typing `let =` tries to smartly add `;` if `=` is followed by an existing expression
// - typing `.` in a chain method call auto-indents
// - typing `{` in front of an expression inserts a closing `}` after the expression
//
// VS Code::
//
// Add the following to `settings.json`:
// [source,json]
// ----
// "editor.formatOnType": true,
// ----
//
// image::https://user-images.githubusercontent.com/48062697/113166163-69758500-923a-11eb-81ee-eb33ec380399.gif[]
// image::https://user-images.githubusercontent.com/48062697/113171066-105c2000-923f-11eb-87ab-f4a263346567.gif[]
pub(crate) fn on_char_typed(
    db: &IdeDatabase,
    position: FilePosition,
    char_typed: char,
) -> Option<SourceChange> {
    todo!()
}

fn on_char_typed_inner(
    file: &ParseResult<SingleFileParseTree>,
    offset: TextSize,
    char_typed: char,
) -> Option<TextEdit> {
    todo!()
}

/// Inserts a closing `}` when the user types an opening `{`, wrapping an existing expression in a
/// block, or a part of a `use` item.
fn on_opening_brace_typed(
    file: &ParseResult<SingleFileParseTree>,
    offset: TextSize,
) -> Option<TextEdit> {
    todo!()
}

/// Returns an edit which should be applied after `=` was typed. Primarily,
/// this works when adding `let =`.
// FIXME: use a snippet completion instead of this hack here.
fn on_eq_typed(file: &SingleFileParseTree, offset: TextSize) -> Option<TextEdit> {
    todo!()
}

/// Returns an edit which should be applied when a dot ('.') is typed on a blank line, indenting the line appropriately.
fn on_dot_typed(file: &SingleFileParseTree, offset: TextSize) -> Option<TextEdit> {
    todo!()
}

/// Adds a space after an arrow when `fn foo() { ... }` is turned into `fn foo() -> { ... }`
fn on_arrow_typed(file: &SingleFileParseTree, offset: TextSize) -> Option<TextEdit> {
    todo!()
}

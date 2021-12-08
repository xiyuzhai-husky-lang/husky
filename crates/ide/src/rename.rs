//! Renaming functionality.
//!
//! This is mostly front-end for [`husky_lang_db::rename`], but it also includes the
//! tests. This module also implements a couple of magic tricks, like renaming
//! `self` and to `self` (to switch between associated function and method).

use hir::{AsAssocItem, InFile, Semantics};
use husky_lang_db::{
    vfs::FileId,
    defs::{Definition, NameClass, NameRefClass},
    rename::{bail, format_err, source_edit_from_references, IdentifierKind},
    HuskyLangDatabase,
};
use itertools::Itertools;
use stdx::{always, never};
use syntax::{ast, SyntaxNode};

use text_edit::TextEdit;

use crate::{FilePosition, RangeInfo, SourceChange};

pub use husky_lang_db::rename::RenameError;

type RenameResult<T> = Result<T, RenameError>;

/// Prepares a rename. The sole job of this function is to return the TextRange of the thing that is
/// being targeted for a rename.
pub(crate) fn prepare_rename(
    db: &HuskyLangDatabase,
    position: FilePosition,
) -> RenameResult<RangeInfo<()>> {
    todo!()
}

// Feature: Rename
//
// Renames the item below the cursor and all of its references
//
// |===
// | Editor  | Shortcut
//
// | VS Code | kbd:[F2]
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113065582-055aae80-91b1-11eb-8ade-2b58e6d81883.gif[]
pub(crate) fn rename(
    db: &HuskyLangDatabase,
    position: FilePosition,
    new_name: &str,
) -> RenameResult<SourceChange> {
    todo!()
}

/// Called by the client when it is about to rename a file.
pub(crate) fn will_rename_file(
    db: &HuskyLangDatabase,
    file_id: FileId,
    new_name_stem: &str,
) -> Option<SourceChange> {
    todo!()
}

fn rename_to_self(sema: &Semantics<HuskyLangDatabase>, local: hir::Local) -> RenameResult<SourceChange> {
    todo!()
}

fn rename_self_to_param(
    sema: &Semantics<HuskyLangDatabase>,
    local: hir::Local,
    self_param: hir::SelfParam,
    new_name: &str,
) -> RenameResult<SourceChange> {
    todo!()
}

fn text_edit_from_self_param(self_param: &ast::SelfParam, new_name: &str) -> Option<TextEdit> {
    todo!()
}

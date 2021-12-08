//! This module allows building an SSR MatchFinder by parsing the SSR rule
//! from a comment.

use common::*;

use husky_lang_db::{
    vfs::{FilePosition, FileRange, VirtualFileSystem},
    HuskyLangDatabase,
};
use syntax::ast;

use crate::MatchFinder;

/// Attempts to build an SSR MatchFinder from a comment at the given file
/// range. If successful, returns the MatchFinder and a TextRange covering
/// comment.
pub fn ssr_from_comment(
    db: &HuskyLangDatabase,
    frange: FileRange,
) -> Option<(MatchFinder, TextRange)> {
    todo!()
}

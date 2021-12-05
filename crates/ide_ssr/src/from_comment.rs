//! This module allows building an SSR MatchFinder by parsing the SSR rule
//! from a comment.

use ide_db::{
    base_db::{FilePosition, FileRange, SourceDatabase},
    RootDatabase,
};
use syntax::{ast, TextRange};

use crate::MatchFinder;

/// Attempts to build an SSR MatchFinder from a comment at the given file
/// range. If successful, returns the MatchFinder and a TextRange covering
/// comment.
pub fn ssr_from_comment(db: &RootDatabase, frange: FileRange) -> Option<(MatchFinder, TextRange)> {
    todo!()
}

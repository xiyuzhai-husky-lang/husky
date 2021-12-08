use std::{fmt, iter::FromIterator, sync::Arc};

use husky_lang_db::vfs::{FileId, VirtualFileSystem};
use husky_lang_db::{symbol_index::SymbolIndex, HuskyLangDatabase};
use itertools::Itertools;
use profile::{memory_usage, Bytes};
use rustc_hash::FxHashMap;
use std::env;
use stdx::format_to;
use syntax::{ast, ParseResult, SyntaxNode};

fn syntax_tree_stats(db: &HuskyLangDatabase) -> SyntaxTreeStats {
    todo!()
}

// Feature: Status
//
// Shows internal statistic about memory usage of husky-lang-server.
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: Status**
// |===
// image::https://user-images.githubusercontent.com/48062697/113065584-05f34500-91b1-11eb-98cc-5c196f76be7f.gif[]
pub(crate) fn status(db: &HuskyLangDatabase, file_id: Option<FileId>) -> String {
    todo!()
}

#[derive(Default)]
struct FilesStats {
    total: usize,
    size: Bytes,
}

impl fmt::Display for FilesStats {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} of files", self.size)
    }
}

#[derive(Default)]
pub(crate) struct SyntaxTreeStats {
    total: usize,
    pub(crate) retained: usize,
}

impl fmt::Display for SyntaxTreeStats {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} trees, {} preserved", self.total, self.retained)
    }
}

#[derive(Default)]
struct LibrarySymbolsStats {
    total: usize,
    size: Bytes,
}

impl fmt::Display for LibrarySymbolsStats {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} of index symbols ({})", self.size, self.total)
    }
}

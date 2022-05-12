use std::path::{Path, PathBuf};

use check_utils::should_eq;
use print_utils::p;
use word::CustomIdentifier;

use crate::line_map::LineMap;

pub(crate) fn apply_document_changes(
    old_text: &mut String,
    content_changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) {
    let mut line_map = LineMap::new(old_text);

    let mut valid_range = LineMapValidRange::All;
    for change in content_changes {
        match change.range {
            Some(range) => {
                if !valid_range.covers(range.end.line) {
                    line_map = LineMap::new(old_text);
                }
                valid_range = LineMapValidRange::UpToLineExclusive(range.start.line);
                let range = LineMap::string_range(&line_map, range);
                old_text.replace_range(range, &change.text);
            }
            None => {
                *old_text = change.text;
                valid_range = LineMapValidRange::UpToLineExclusive(0);
            }
        }
    }
}

// The changes we got must be applied sequentially, but can cross lines so we
// have to keep our line index updated.
// Some clients (e.g. Code) sort the ranges in reverse. As an optimization, we
// remember the last valid line in the index and only rebuild it if needed.
// The VFS will normalize the end of lines to `\n`.
enum LineMapValidRange {
    All,
    UpToLineExclusive(u32),
}

impl LineMapValidRange {
    fn covers(&self, line: u32) -> bool {
        match *self {
            LineMapValidRange::UpToLineExclusive(to) => to > line,
            _ => true,
        }
    }
}

// pub struct LineMap {}

// impl LineMap {
//     pub fn new(text: &str) -> LineMap {
//         todo!()
//     }

//     pub fn string_range(&self, text_range: lsp_types::Range) -> Range {
//         todo!()
//     }
// }

pub fn get_submodule_file(module_file: &Path, ident: CustomIdentifier) -> Option<PathBuf> {
    should_eq!(module_file.extension().unwrap(), "hsk");
    if module_file.file_name().unwrap() == "mod.hsk"
        || module_file.file_name().unwrap() == "main.hsk"
    {
        let maybe_submodule_file = module_file.with_file_name(format!("{}.hsk", ident));
        if maybe_submodule_file.exists() {
            Some(maybe_submodule_file)
        } else {
            let maybe_submodule_file = module_file.with_file_name(format!("{}.hsk", ident));
            if maybe_submodule_file.exists() {
                Some(maybe_submodule_file)
            } else {
                None
            }
        }
    } else {
        p!(module_file);
        todo!()
    }
}

use ide_db::{base_db::FileID, source_change::SourceChange};
use itertools::Itertools;
use syntax::{ast, SyntaxNode, TextRange};
use text_edit::TextEdit;

use crate::{fix, Diagnostic, Severity};

// Diagnostic: unnecessary-braces
//
// Diagnostic for unnecessary braces in `use` items.
pub(crate) fn useless_braces(
    acc: &mut Vec<Diagnostic>,
    file_id: FileID,
    node: &SyntaxNode,
) -> Option<()> {
    todo!()
}

fn remove_braces(single_use_tree: &ast::UseTree) -> Option<TextEdit> {
    todo!()
}

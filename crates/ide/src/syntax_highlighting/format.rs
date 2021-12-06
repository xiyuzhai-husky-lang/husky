//! Syntax highlighting for format macro strings.
use common::*;

use ide_db::SymbolKind;
use syntax::ast::{self, FormatSpecifier, HasFormatSpecifier};

use crate::{syntax_highlighting::highlights::Highlights, HlRange, HlTag};

pub(super) fn highlight_format_string(
    stack: &mut Highlights,
    string: &ast::String,
    expanded_string: &ast::String,
    range: TextRange,
) {
    todo!()
}

fn is_format_string(string: &ast::String) -> Option<()> {
    todo!()
}

fn highlight_format_specifier(kind: FormatSpecifier) -> Option<HlTag> {
    todo!()
}

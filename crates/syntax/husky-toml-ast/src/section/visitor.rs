use super::*;

pub struct TomlSectionVisitor<'a> {
    toml_ast_sheet: &'a TomlAstSheet,
    section_idx: TomlSectionIdx,
}

impl<'a> TomlSectionVisitor<'a> {
    pub(crate) fn new(toml_ast_sheet: &'a TomlAstSheet, section_idx: TomlSectionIdx) -> Self {
        Self {
            toml_ast_sheet,
            section_idx,
        }
    }
}

use super::*;
use vec_like::VecSet;

pub struct TomlSectionVisitor<'a> {
    toml_ast_sheet: &'a TomlAstSheet,
    section_idx: TomlSectionIdx,
    section: &'a TomlSection,
    visits: VecSet<Word>,
}

impl<'a> TomlSectionVisitor<'a> {
    pub(crate) fn new(toml_ast_sheet: &'a TomlAstSheet, section_idx: TomlSectionIdx) -> Self {
        Self {
            toml_ast_sheet,
            section_idx,
            section: &toml_ast_sheet.section_sheet[section_idx],
            visits: Default::default(),
        }
    }

    pub(crate) fn toml_ast_sheet(&self) -> &'a TomlAstSheet {
        self.toml_ast_sheet
    }

    pub(crate) fn visit(&mut self, key: Word) -> Option<TomlExprIdx> {
        self.visits.insert_new(key).unwrap();
        self.section.get_entry(key)?.value()
    }
}

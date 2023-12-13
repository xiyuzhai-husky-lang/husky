use super::*;
use vec_like::VecSet;

pub struct TomlSectionVisitor<'a> {
    toml_ast_sheet: &'a TomlAstSheet,
    section_idx: TomlSectionIdx,
    section: &'a TomlSection,
    visits: VecSet<Coword>,
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

    pub(crate) fn visit(&mut self, key: Coword) -> Option<TomlExprIdx> {
        self.visits.insert_new(key).unwrap();
        self.section.get_entry(key)?.value()
    }

    pub fn section_idx(&self) -> ArenaIdx<TomlSection> {
        self.section_idx
    }

    // should only use this once!!
    pub(crate) fn all_entries(&self) -> &'a [TomlSectionEntry] {
        assert!(self.visits.is_empty());
        &self.section.entries
    }
}

pub struct TomlSectionEntryVisitor<'a> {
    toml_ast_sheet: &'a TomlAstSheet,
    section_idx: TomlSectionIdx,
    section_entry: &'a TomlSectionEntry,
    visited: bool,
}

impl<'a> TomlSectionEntryVisitor<'a> {
    pub(crate) fn new(
        toml_ast_sheet: &'a TomlAstSheet,
        section_idx: TomlSectionIdx,
        section_entry: &'a TomlSectionEntry,
    ) -> Self {
        Self {
            toml_ast_sheet,
            section_idx,
            section_entry,
            visited: false,
        }
    }

    pub(crate) fn toml_ast_sheet(&self) -> &'a TomlAstSheet {
        self.toml_ast_sheet
    }

    pub fn section_idx(&self) -> TomlSectionIdx {
        self.section_idx
    }

    pub fn key(&self) -> Coword {
        self.section_entry.key
    }

    pub fn line_group_idx(&self) -> TomlLineGroupIdx {
        self.section_entry.line_group_idx
    }

    pub fn visit(&mut self) -> Option<TomlExprIdx> {
        assert!(!self.visited);
        self.visited = true;
        self.section_entry.value
    }
}

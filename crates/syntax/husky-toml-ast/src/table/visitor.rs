use super::*;
use vec_like::VecSet;

pub struct TomlTableVisitor<'a> {
    toml_ast_sheet: &'a TomlAstSheet,
    table: &'a TomlTable,
    visits: VecSet<Coword>,
}

impl<'a> TomlTableVisitor<'a> {
    /// get the value for the key
    /// todo: mark the entry as visited
    pub fn visit(&mut self, key: Coword) -> Option<&'a TomlTableValue> {
        self.visits.insert_new(key).unwrap();
        self.table.get(key)
    }

    pub fn toml_ast_sheet(&self) -> &'a TomlAstSheet {
        self.toml_ast_sheet
    }
}

impl TomlAstSheet {
    pub(crate) fn root_visitor(&self) -> TomlVisitor<TomlTable> {
        TomlTableVisitor {
            toml_ast_sheet: self,
            table: &self.table,
            visits: Default::default(),
        }
    }
}

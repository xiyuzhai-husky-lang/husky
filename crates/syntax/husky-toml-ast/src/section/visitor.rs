use super::*;

pub struct TomlSectionAstVisitor<'a> {
    section_arena: &'a TomlSectionAstSheet,
}

impl<'a> TomlSectionAstVisitor<'a> {
    pub fn find_by_title(
        &mut self,
        title: TomlSectionTitle,
    ) -> Option<(TomlSectionAstIdx, &'a TomlSection)> {
        self.section_arena
            .arena
            .indexed_iter()
            .find(|(_, toml_section_ast)| toml_section_ast.title == title)
    }
}

impl<'a> VisitTomlAst<'a> for TomlSectionAstVisitor<'a> {
    type Ast = TomlSection;
}

impl TomlAstSheet {
    pub fn section_visitor(&self) -> TomlAstVisitor<TomlSection> {
        TomlSectionAstVisitor {
            section_arena: &self.section_arena,
        }
    }
}

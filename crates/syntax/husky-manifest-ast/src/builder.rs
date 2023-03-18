use crate::*;

pub(crate) struct ManifestAstBuilder<'a, A: TomlAst> {
    db: &'a dyn ManifestAstDb,
    toml_ast: &'a TomlAstSheet,
    toml_ast_visitor: TomlAstVisitor<'a, A>,
    menu: &'a ManifestAstMenu,
}

impl<'a, A: TomlAst> ManifestAstBuilder<'a, A> {
    pub(crate) fn new(
        db: &'a dyn ManifestAstDb,
        toml_ast: &'a TomlAstSheet,
        toml_ast_visitor: TomlAstVisitor<'a, A>,
    ) -> Self {
        Self {
            db,
            toml_ast,
            toml_ast_visitor,
            menu: manifest_ast_menu(db),
        }
    }

    pub(crate) fn subbuilder<B>(
        &self,
        toml_ast_visitor: TomlAstVisitor<'a, B>,
    ) -> ManifestAstBuilder<'a, B>
    where
        B: TomlAst + TomlSubAst<A>,
    {
        ManifestAstBuilder {
            db: self.db,
            toml_ast: self.toml_ast,
            toml_ast_visitor,
            menu: self.menu,
        }
    }

    pub(crate) fn menu(&self) -> &ManifestAstMenu {
        self.menu
    }
}

impl<'a> ManifestAstBuilder<'a, TomlSection> {
    pub(crate) fn visit_new_normal_section_ast(
        &mut self,
        title: TomlSectionTitle,
    ) -> ManifestAstResult<Option<(TomlSectionAstIdx, &'a TomlSection)>> {
        let Some((idx, section_ast)) = self.toml_ast_visitor.find_by_title(title) else {
            return Ok(None)
        };
        if section_ast.kind() != TomlSectionKind::Normal {
            return Err(todo!("report error"));
        }
        Ok(Some((idx, section_ast)))
    }
}

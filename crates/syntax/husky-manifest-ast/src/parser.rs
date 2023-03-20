use husky_word::Word;

use crate::*;

pub(crate) struct ManifestAstParser<'a, A: TomlAst> {
    db: &'a dyn ManifestAstDb,
    toml_ast_sheet: &'a TomlAstSheet,
    toml_ast_visitor: TomlVisitor<'a, A>,
    menu: &'a ManifestAstMenu,
}

impl<'a, A: TomlAst> ManifestAstParser<'a, A> {
    pub(crate) fn new(
        db: &'a dyn ManifestAstDb,
        toml_ast_sheet: &'a TomlAstSheet,
        toml_ast_visitor: TomlVisitor<'a, A>,
    ) -> Self {
        Self {
            db,
            toml_ast_sheet,
            toml_ast_visitor,
            menu: manifest_ast_menu(db),
        }
    }

    pub(crate) fn subparser<B>(
        &self,
        toml_ast_visitor: TomlVisitor<'a, B>,
    ) -> ManifestAstParser<'a, B>
    where
        B: TomlAst + TomlSubAst<A>,
    {
        ManifestAstParser {
            db: self.db,
            toml_ast_sheet: self.toml_ast_sheet,
            toml_ast_visitor,
            menu: self.menu,
        }
    }

    pub(crate) fn menu(&self) -> &ManifestAstMenu {
        self.menu
    }
}

impl<'a> ManifestAstParser<'a, TomlTable> {
    pub(crate) fn visit_normal_section_ast(
        &mut self,
        key: Word,
        errors: &mut Vec<ManifestAstError>,
    ) -> ManifestAstResult<Option<(TomlSectionIdx, &'a TomlSection)>> {
        match self.toml_ast_visitor.visit(key) {
            Some(TomlTableValue::Section(_)) => todo!(),
            Some(_) => todo!(),
            None => todo!(),
        }
        // let Some((idx, section_ast)) = self.toml_ast_visitor.find_by_title(title) else {
        //     return Ok(None)
        // };
        // if section_ast.kind() != TomlSectionKind::Normal {
        //     return Err(todo!("report error"));
        // }
        // Ok(Some((idx, section_ast)))
    }

    pub(crate) fn normal_section_parser(
        &mut self,
        key: Word,
        errors: &mut Vec<ManifestAstError>,
    ) -> ManifestAstResult<Option<ManifestAstParser<'a, TomlSection>>> {
        let Some((idx, dependencies_section_ast)) = self
            .visit_normal_section_ast(self.menu().dependencies_word(), errors)? else {
            return Ok(None)
        };
        todo!()
    }
}

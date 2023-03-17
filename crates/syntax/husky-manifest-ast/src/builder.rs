use crate::*;

pub(crate) struct ManifestAstBuilder<'a, A: TomlAst> {
    db: &'a dyn ManifestAstDb,
    toml_ast: &'a TomlAstSheet,
    toml_ast_visitor: TomlAstVisitor<'a, A>,
    errors: Vec<ManifestAstError>,
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
            errors: vec![],
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
            errors: vec![],
        }
    }

    fn build_all(mut self) -> ManifestAst {
        todo!()
    }

    pub(crate) fn finish(self) -> Vec<ManifestAstError> {
        self.errors
    }
}

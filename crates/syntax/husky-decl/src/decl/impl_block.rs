use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ImplBlockDecl {
    Type(TypeImplBlockDecl),
    TypeAsTrait(TypeAsTraitImplBlockDecl),
}

#[salsa::tracked(jar = DeclJar)]
pub struct TypeImplBlockDecl {}

#[salsa::tracked(jar = DeclJar)]
pub struct TypeAsTraitImplBlockDecl {}

impl ImplBlockDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        todo!()
    }

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        todo!()
    }

    pub fn expr_sheet(self, db: &dyn DeclDb) -> ExprSheet {
        todo!()
    }
}

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for ImplBlockDecl {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclJar>>::as_jar_db(db);
        todo!()
    }
}

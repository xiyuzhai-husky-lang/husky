use crate::*;
use husky_entity_path::{EntityPathDb, EntityPathJar, EntityPathMenuPlace};
use husky_package_path::PackagePathJar;
use husky_toolchain::*;
use husky_word::WordDb;
use husky_word::WordJar;
use std::{collections::HashMap, sync::Arc};

#[salsa::db(crate::TermJar, EntityPathJar, PackagePathJar, ToolchainJar, WordJar)]
pub(crate) struct TermTestsDb {
    storage: salsa::Storage<TermTestsDb>,
    ty_decls: HashMap<Term, Arc<TyDecl>>,
}

impl TermTestsDb {
    pub fn new() -> Self {
        Self {
            storage: Default::default(),
            ty_decls: Default::default(),
        }
        .init()
    }

    fn init(mut self) -> Self {
        let menu = self.term_menu();
        self.ty_decls.extend(menu.primitive_ty_decls());
        self
    }
}

impl salsa::Database for TermTestsDb {}

impl AskDecl for TermTestsDb {
    fn ask_namespace_decl(&self, _namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
        todo!()
    }

    fn ask_ty_decl(&self, ty: Term) -> TermResultArc<TyDecl> {
        Ok(self.ty_decls[&ty].clone())
    }

    fn ask_decl(&self, _entity_path: EntityPath) -> TermResultArc<Decl> {
        todo!()
    }
}

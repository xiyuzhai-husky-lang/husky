use crate::*;
use husky_entity_path::{EntityPathDb, EntityPathJar};
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
    }
}

impl salsa::Database for TermTestsDb {}

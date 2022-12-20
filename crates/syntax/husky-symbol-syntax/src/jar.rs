use once_cell::sync::OnceCell;

use crate::{Symbol, SymbolDb};

pub struct SymbolJar(OnceCell<Vec<Symbol>>);

impl<'salsa_db> salsa::jar::Jar<'salsa_db> for SymbolJar {
    type DynDb = dyn SymbolDb + 'salsa_db;
    fn create_jar<DB>(_routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        Self(Default::default())
    }
}

impl SymbolJar {
    pub(crate) fn preludes_place(&self) -> &OnceCell<Vec<Symbol>> {
        &self.0
    }
}

use salsa::Database;

use super::*;

impl Upcast<dyn TermDb> for TermInferTestsDb {
    fn upcast(&self) -> &(dyn TermDb + 'static) {
        self
    }
}

impl Database for TermInferTestsDb {}

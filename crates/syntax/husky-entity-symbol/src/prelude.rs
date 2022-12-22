use crate::*;

#[salsa::tracked(jar = EntitySymbolJar)]
pub(crate) fn crate_prelude(db: &dyn EntitySymbolDb, crate_path: CratePath) -> CratePrelude {
    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CratePrelude {}

// impl CratePreludeBuilder {}

use vec_like::VecPairMap;

use crate::*;

#[salsa::tracked(jar = DefnJar, return_ref)]
pub(crate) fn defn_sheet(db: &dyn DefnDb, module_path: ModulePath) -> EntityTreeResult<DefnSheet> {
    Ok(DefnCollector::new(db, module_path)?.collect_all())
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DefnDb)]
pub struct DefnSheet {
    defns: Vec<(EntityNodePath, DeclResult<Defn>)>,
}

impl DefnSheet {
    pub fn new(defns: Vec<(EntityNodePath, DeclResult<Defn>)>) -> Self {
        Self { defns }
    }

    pub fn defns<'a>(&'a self) -> impl Iterator<Item = (EntityNodePath, DeclResult<Defn>)> + 'a {
        self.defns.iter().copied()
    }
}

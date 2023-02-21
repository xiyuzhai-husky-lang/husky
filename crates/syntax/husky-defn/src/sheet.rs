use vec_like::VecPairMap;

use crate::*;

#[salsa::tracked(jar = DefnJar, return_ref)]
pub(crate) fn defn_sheet(db: &dyn DefnDb, module_path: ModulePath) -> EntityTreeResult<DefnSheet> {
    Ok(DefnCollector::new(db, module_path)?.collect_all())
}

#[derive(Debug, PartialEq, Eq)]
pub struct DefnSheet {
    defns: Vec<Defn>,
}

impl DefnSheet {
    pub fn new(defns: Vec<Defn>) -> Self {
        Self { defns }
    }

    pub fn defns<'a>(&'a self) -> impl Iterator<Item = Defn> + 'a {
        self.defns.iter().copied()
    }
}

impl<Db: DefnDb + ?Sized> salsa::DebugWithDb<Db> for DefnSheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<DefnJar>>::as_jar_db(db);
        f.debug_struct("DefnSheet")
            .field("defns", &self.defns.debug(db))
            .finish()
    }
}

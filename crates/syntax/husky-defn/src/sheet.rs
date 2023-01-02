use vec_like::VecPairMap;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DefnSheet {
    defns: VecPairMap<EntityPath, Defn>,
}

impl DefnSheet {
    pub fn new(defns: VecPairMap<EntityPath, Defn>) -> Self {
        Self { defns }
    }

    pub fn defns<'a>(&'a self) -> impl Iterator<Item = Defn> + 'a {
        self.defns.iter().map(|(_, defn)| *defn)
    }
}

impl<Db: DefnDb + ?Sized> salsa::DebugWithDb<Db> for DefnSheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<DefnJar>>::as_jar_db(db);
        f.debug_struct("DefnSheet")
            .field("defns", &(&self.defns.data()).debug(db))
            .finish()
    }
}

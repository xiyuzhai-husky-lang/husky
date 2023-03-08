use vec_like::VecPairMap;

use crate::*;

pub(crate) fn collect_defn_sheet(
    db: &dyn DefnDb,
    module_path: ModulePath,
) -> EntityTreeResult<DefnSheet> {
    Ok(DefnCollector::new(db, module_path)?.collect_all())
}

#[derive(Debug, PartialEq, Eq)]
pub struct DefnSheet<'a> {
    defns: VecPairMap<DefnRegionPath, DeclResultRef<'a, Defn>>,
}

impl<'a> DefnSheet<'a> {
    pub fn new(defns: VecPairMap<DefnRegionPath, DeclResultRef<'a, Defn>>) -> Self {
        Self { defns }
    }

    pub fn defns<'b>(
        &'b self,
    ) -> impl Iterator<Item = (DefnRegionPath, DeclResultRef<'a, Defn>)> + 'b {
        self.defns.iter().copied()
    }
}

impl<'a, Db: DefnDb + ?Sized> salsa::DebugWithDb<Db> for DefnSheet<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<DefnJar>>::as_jar_db(db);
        f.debug_struct("DefnSheet")
            .field("defns", &self.defns.debug(db))
            .finish()
    }
}

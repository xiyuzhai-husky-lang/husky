use crate::*;

use salsa::DbWithJar;

pub trait DefnDb: DbWithJar<DefnJar> + DeclDb {
    fn defn_sheet<'a>(&'a self, module_path: ModulePath) -> EntityTreeResultRef<'a, &'a DefnSheet>;
}

impl<Db> DefnDb for Db
where
    Db: DbWithJar<DefnJar> + DeclDb,
{
    fn defn_sheet<'a>(&'a self, module_path: ModulePath) -> EntityTreeResultRef<'a, &'a DefnSheet> {
        defn_sheet(self, module_path).as_ref()
    }
}

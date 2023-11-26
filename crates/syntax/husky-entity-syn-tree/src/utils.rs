use crate::*;

pub(crate) fn entity_syn_tree_db<Db>(db: &Db) -> &dyn EntitySynTreeDb
where
     + EntitySynTreeDb,
{
    salsa::DbWithJar::<EntitySynTreeJar>::as_jar_db(db)
}

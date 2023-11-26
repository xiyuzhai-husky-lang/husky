use crate::*;

pub(crate) fn entity_syn_tree_db<Db>(db: &::salsa::Db,) -> &::salsa::Db
where
     + EntitySynTreeDb,
{
    salsa::DbWithJar::<EntitySynTreeJar>::as_jar_db(db)
}

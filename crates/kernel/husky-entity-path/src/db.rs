use husky_identifier::IdentifierDb;

use crate::*;
use std::sync::Arc;

pub trait EntityPathDb: DbWithJar<EntityPathJar> + IdentifierDb {
    fn entity_path_menu(&self) -> Arc<EntityPathMenu>;
}

impl<T> EntityPathDb for T
where
    T: DbWithJar<EntityPathJar> + IdentifierDb,
{
    fn entity_path_menu(&self) -> Arc<EntityPathMenu> {
        todo!()
    }
}

fn entity_path_menu(db: &dyn EntityPathDb) -> Arc<EntityPathMenu> {
    Arc::new(EntityPathMenu::new(db))
}

impl dyn EntityPathDb + '_ {
    pub(crate) fn it_root_entity_path(&self, ident: &str) -> EntityPath {
        todo!()
        // self.it_entity_path(EntityPath::root(self.it_ident(ident)))
    }
    pub(crate) fn it_child_entity_path(&self, parent: EntityPath, ident: &str) -> EntityPath {
        todo!()
        // self.it_entity_path(EntityPath {
        //     ident: self.it_ident(ident),
        //     variant: EntityPathVariant::Childpath { parent },
        // })
    }
}

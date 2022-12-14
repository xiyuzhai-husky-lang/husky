use husky_package_path::PackagePathDb;
use husky_word::WordDb;
use place::SingleAssignPlace;
use salsa::storage::HasJar;

use crate::*;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct EntityPathMenuPlace(Arc<once_cell::sync::OnceCell<EntityPathMenu>>);

pub trait EntityPathDb: DbWithJar<EntityPathJar> + PackagePathDb + WordDb {
    fn entity_path_menu(&self) -> &EntityPathMenu;
    fn it_entity_path(&self, data: EntityPathData) -> EntityPath;
    fn dt_entity_path(&self, path: EntityPath) -> EntityPathData;
    fn entity_package(&self, path: EntityPath) -> PackagePath;
    fn is_builtin_entity(&self, path: EntityPath) -> bool;
}

impl<T> EntityPathDb for T
where
    T: DbWithJar<EntityPathJar> + PackagePathDb + WordDb,
{
    fn entity_path_menu(&self) -> &EntityPathMenu {
        <Self as HasJar<EntityPathJar>>::jar(self)
            .0
            .entity_path_menu_place()
            .0
            .get_or_init(|| EntityPathMenu::new(self))
    }

    fn it_entity_path(&self, data: EntityPathData) -> EntityPath {
        EntityPath::new(self, data)
    }

    fn dt_entity_path(&self, entity: EntityPath) -> EntityPathData {
        entity.data(self)
    }

    fn entity_package(&self, entity: EntityPath) -> PackagePath {
        entity_package(self, entity)
    }

    fn is_builtin_entity(&self, entity: EntityPath) -> bool {
        is_builtin_entity(self, entity)
    }
}

impl dyn EntityPathDb + '_ {
    pub(crate) fn it_builtin_lib_path(&self, ident: &str) -> Option<EntityPath> {
        let ident = self.it_ident_borrowed(ident)?;
        Some(self.it_entity_path(EntityPathData::Crate {
            package: self.builtin_package_path(ident)?,
            kind: CratePathKind::Library,
        }))
    }
    pub(crate) fn it_child_entity_path(
        &self,
        parent: EntityPath,
        ident: &str,
    ) -> Option<EntityPath> {
        Some(self.it_entity_path(EntityPathData::Childpath {
            parent,
            ident: self.it_ident_borrowed(ident)?,
        }))
    }
}

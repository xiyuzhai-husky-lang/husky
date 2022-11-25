use husky_identifier::IdentifierDb;
use place::SingleAssignPlace;
use salsa::storage::HasJar;

use crate::*;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct EntityPathMenuPlace(Arc<once_cell::sync::OnceCell<EntityPathMenu>>);

pub trait EntityPathDb: DbWithJar<EntityPathJar> + IdentifierDb {
    fn entity_path_menu(&self) -> &EntityPathMenu;
    fn it_entity_path(&self, ident: Identifier, variant: EntityPathVariant) -> EntityPath;
}

impl<T> EntityPathDb for T
where
    T: DbWithJar<EntityPathJar> + IdentifierDb,
{
    fn entity_path_menu(&self) -> &EntityPathMenu {
        <Self as HasJar<EntityPathJar>>::jar(self)
            .0
             .2
             .0
            .get_or_init(|| EntityPathMenu::new(self))
    }

    fn it_entity_path(&self, ident: Identifier, variant: EntityPathVariant) -> EntityPath {
        EntityPath::new(self, ident, variant)
    }
}

impl dyn EntityPathDb + '_ {
    pub(crate) fn it_builtin_lib_path(&self, ident: &str) -> EntityPath {
        self.it_entity_path(
            self.it_ident_borrowed(ident),
            EntityPathVariant::Crate {
                package: PackagePathData::Builtin {
                    toolchain: Toolchain::new_ad_hoc(),
                },
                kind: CratePathKind::Library,
            },
        )
    }
    pub(crate) fn it_child_entity_path(&self, parent: EntityPath, ident: &str) -> EntityPath {
        self.it_entity_path(
            self.it_ident_borrowed(ident),
            EntityPathVariant::Childpath { parent },
        )
    }
}

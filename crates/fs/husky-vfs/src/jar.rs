use crate::*;
use place::SingleAssignPlace;

pub struct VfsJar(
    <PathBufItd as salsa::storage::IngredientsFor>::Ingredients,
    <SourceFile as salsa::storage::IngredientsFor>::Ingredients,
    SourcePathMap,
    SingleAssignPlace<VfsWatcher>,
);

impl VfsJar {
    pub(crate) fn husky_file_cache(&self) -> &SourcePathMap {
        &self.2
    }

    pub(crate) fn watcher_place(&self) -> &SingleAssignPlace<VfsWatcher> {
        &self.3
    }

    pub(crate) fn watcher_place_mut(&mut self) -> &mut SingleAssignPlace<VfsWatcher> {
        &mut self.3
    }
}

impl salsa::storage::HasIngredientsFor<PathBufItd> for VfsJar {
    fn ingredient(&self) -> &<PathBufItd as salsa::storage::IngredientsFor>::Ingredients {
        &self.0
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <PathBufItd as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.0
    }
}

impl salsa::storage::HasIngredientsFor<SourceFile> for VfsJar {
    fn ingredient(&self) -> &<SourceFile as salsa::storage::IngredientsFor>::Ingredients {
        &self.1
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <SourceFile as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.1
    }
}

impl<'salsa_db> salsa::jar::Jar<'salsa_db> for VfsJar {
    type DynDb = dyn VfsDb + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <PathBufItd as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i1 = <SourceFile as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(i0, i1, Default::default(), Default::default())
    }
}

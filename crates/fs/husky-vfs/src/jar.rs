use crate::*;
use place::SingleAssignPlace;

pub struct VfsJar(
    <PathBufItd as salsa::storage::IngredientsFor>::Ingredients,
    <File as salsa::storage::IngredientsFor>::Ingredients,
    <absolute_path_from_source_path as salsa::storage::IngredientsFor>::Ingredients,
    <source_path_from_absolute_path as salsa::storage::IngredientsFor>::Ingredients,
    <package_absolute_path as salsa::storage::IngredientsFor>::Ingredients,
    VfsCache,
    SingleAssignPlace<VfsWatcher>,
);

impl VfsJar {
    pub(crate) fn cache(&self) -> &VfsCache {
        &self.5
    }

    pub(crate) fn watcher_place(&self) -> &SingleAssignPlace<VfsWatcher> {
        &self.6
    }

    pub(crate) fn watcher_place_mut(&mut self) -> &mut SingleAssignPlace<VfsWatcher> {
        &mut self.6
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

impl salsa::storage::HasIngredientsFor<File> for VfsJar {
    fn ingredient(&self) -> &<File as salsa::storage::IngredientsFor>::Ingredients {
        &self.1
    }
    fn ingredient_mut(&mut self) -> &mut <File as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.1
    }
}

impl salsa::storage::HasIngredientsFor<absolute_path_from_source_path> for VfsJar {
    fn ingredient(
        &self,
    ) -> &<absolute_path_from_source_path as salsa::storage::IngredientsFor>::Ingredients {
        &self.2
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <absolute_path_from_source_path as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.2
    }
}

impl salsa::storage::HasIngredientsFor<source_path_from_absolute_path> for VfsJar {
    fn ingredient(
        &self,
    ) -> &<source_path_from_absolute_path as salsa::storage::IngredientsFor>::Ingredients {
        &self.3
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <source_path_from_absolute_path as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.3
    }
}

impl salsa::storage::HasIngredientsFor<package_absolute_path> for VfsJar {
    fn ingredient(
        &self,
    ) -> &<package_absolute_path as salsa::storage::IngredientsFor>::Ingredients {
        &self.4
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <package_absolute_path as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.4
    }
}

impl<'salsa_db> salsa::jar::Jar<'salsa_db> for VfsJar {
    type DynDb = dyn VfsDb + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <PathBufItd as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i1 = <File as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i2 =
            <absolute_path_from_source_path as salsa::storage::IngredientsFor>::create_ingredients(
                routes,
            );
        let i3 =
            <source_path_from_absolute_path as salsa::storage::IngredientsFor>::create_ingredients(
                routes,
            );
        let i4 =
            <package_absolute_path as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(i0, i1, i2, i3, i4, Default::default(), Default::default())
    }
}

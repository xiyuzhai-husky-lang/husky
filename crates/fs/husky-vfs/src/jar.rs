use crate::*;

pub struct VfsJar(
    <PathBufItd as salsa::storage::IngredientsFor>::Ingredients,
    <File as salsa::storage::IngredientsFor>::Ingredients,
    <package_dir as salsa::storage::IngredientsFor>::Ingredients,
    <module_absolute_path as salsa::storage::IngredientsFor>::Ingredients,
    <package_manifest_file as salsa::storage::IngredientsFor>::Ingredients,
    <module_file as salsa::storage::IngredientsFor>::Ingredients,
    VfsCache,
);

impl VfsJar {
    pub(crate) fn cache(&self) -> &VfsCache {
        &self.6
    }

    pub(crate) fn set_watcher(&mut self, watcher: VfsWatcher) {
        self.6.set_watcher(watcher)
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

impl salsa::storage::HasIngredientsFor<package_dir> for VfsJar {
    fn ingredient(&self) -> &<package_dir as salsa::storage::IngredientsFor>::Ingredients {
        &self.2
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <package_dir as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.2
    }
}

impl salsa::storage::HasIngredientsFor<module_absolute_path> for VfsJar {
    fn ingredient(&self) -> &<module_absolute_path as salsa::storage::IngredientsFor>::Ingredients {
        &self.3
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <module_absolute_path as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.3
    }
}

impl salsa::storage::HasIngredientsFor<package_manifest_file> for VfsJar {
    fn ingredient(
        &self,
    ) -> &<package_manifest_file as salsa::storage::IngredientsFor>::Ingredients {
        &self.4
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <package_manifest_file as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.4
    }
}

impl salsa::storage::HasIngredientsFor<module_file> for VfsJar {
    fn ingredient(&self) -> &<module_file as salsa::storage::IngredientsFor>::Ingredients {
        &self.5
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <module_file as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.5
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
        let i2 = <package_dir as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i3 =
            <module_absolute_path as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i4 =
            <package_manifest_file as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i5 = <module_file as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(i0, i1, i2, i3, i4, i5, Default::default())
    }
}

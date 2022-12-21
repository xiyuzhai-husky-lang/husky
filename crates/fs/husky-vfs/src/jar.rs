use crate::*;

pub struct VfsJar(
    <PackagePath as salsa::storage::IngredientsFor>::Ingredients,
    <CratePath as salsa::storage::IngredientsFor>::Ingredients,
    <ModulePath as salsa::storage::IngredientsFor>::Ingredients,
    <apparent_ancestry as salsa::storage::IngredientsFor>::Ingredients,
    <path_menu as salsa::storage::IngredientsFor>::Ingredients,
    <AbsolutePath as salsa::storage::IngredientsFor>::Ingredients,
    <File as salsa::storage::IngredientsFor>::Ingredients,
    <package_dir as salsa::storage::IngredientsFor>::Ingredients,
    <module_absolute_path as salsa::storage::IngredientsFor>::Ingredients,
    <package_manifest_file as salsa::storage::IngredientsFor>::Ingredients,
    <module_file as salsa::storage::IngredientsFor>::Ingredients,
    <Toolchain as salsa::storage::IngredientsFor>::Ingredients,
    <PublishedToolchain as salsa::storage::IngredientsFor>::Ingredients,
    <published_toolchain_library_path as salsa::storage::IngredientsFor>::Ingredients,
    <current_toolchain as salsa::storage::IngredientsFor>::Ingredients,
    VfsCache,
);

impl salsa::storage::HasIngredientsFor<PackagePath> for VfsJar {
    fn ingredient(&self) -> &<PackagePath as salsa::storage::IngredientsFor>::Ingredients {
        &self.0
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <PackagePath as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.0
    }
}

impl salsa::storage::HasIngredientsFor<CratePath> for VfsJar {
    fn ingredient(&self) -> &<CratePath as salsa::storage::IngredientsFor>::Ingredients {
        &self.1
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <CratePath as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.1
    }
}

impl salsa::storage::HasIngredientsFor<ModulePath> for VfsJar {
    fn ingredient(&self) -> &<ModulePath as salsa::storage::IngredientsFor>::Ingredients {
        &self.2
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <ModulePath as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.2
    }
}

impl salsa::storage::HasIngredientsFor<apparent_ancestry> for VfsJar {
    fn ingredient(&self) -> &<apparent_ancestry as salsa::storage::IngredientsFor>::Ingredients {
        &self.3
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <apparent_ancestry as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.3
    }
}

impl salsa::storage::HasIngredientsFor<path_menu> for VfsJar {
    fn ingredient(&self) -> &<path_menu as salsa::storage::IngredientsFor>::Ingredients {
        &self.4
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <path_menu as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.4
    }
}

impl salsa::storage::HasIngredientsFor<AbsolutePath> for VfsJar {
    fn ingredient(&self) -> &<AbsolutePath as salsa::storage::IngredientsFor>::Ingredients {
        &self.5
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <AbsolutePath as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.5
    }
}

impl salsa::storage::HasIngredientsFor<File> for VfsJar {
    fn ingredient(&self) -> &<File as salsa::storage::IngredientsFor>::Ingredients {
        &self.6
    }
    fn ingredient_mut(&mut self) -> &mut <File as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.6
    }
}

impl salsa::storage::HasIngredientsFor<package_dir> for VfsJar {
    fn ingredient(&self) -> &<package_dir as salsa::storage::IngredientsFor>::Ingredients {
        &self.7
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <package_dir as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.7
    }
}

impl salsa::storage::HasIngredientsFor<module_absolute_path> for VfsJar {
    fn ingredient(&self) -> &<module_absolute_path as salsa::storage::IngredientsFor>::Ingredients {
        &self.8
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <module_absolute_path as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.8
    }
}

impl salsa::storage::HasIngredientsFor<package_manifest_file> for VfsJar {
    fn ingredient(
        &self,
    ) -> &<package_manifest_file as salsa::storage::IngredientsFor>::Ingredients {
        &self.9
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <package_manifest_file as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.9
    }
}

impl salsa::storage::HasIngredientsFor<module_file> for VfsJar {
    fn ingredient(&self) -> &<module_file as salsa::storage::IngredientsFor>::Ingredients {
        &self.10
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <module_file as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.10
    }
}

impl salsa::storage::HasIngredientsFor<Toolchain> for VfsJar {
    fn ingredient(&self) -> &<Toolchain as salsa::storage::IngredientsFor>::Ingredients {
        &self.11
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <Toolchain as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.11
    }
}

impl salsa::storage::HasIngredientsFor<PublishedToolchain> for VfsJar {
    fn ingredient(&self) -> &<PublishedToolchain as salsa::storage::IngredientsFor>::Ingredients {
        &self.12
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <PublishedToolchain as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.12
    }
}

impl salsa::storage::HasIngredientsFor<published_toolchain_library_path> for VfsJar {
    fn ingredient(
        &self,
    ) -> &<published_toolchain_library_path as salsa::storage::IngredientsFor>::Ingredients {
        &self.13
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <published_toolchain_library_path as salsa::storage::IngredientsFor>::Ingredients
    {
        &mut self.13
    }
}

impl salsa::storage::HasIngredientsFor<current_toolchain> for VfsJar {
    fn ingredient(&self) -> &<current_toolchain as salsa::storage::IngredientsFor>::Ingredients {
        &self.14
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <current_toolchain as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.14
    }
}

impl<'salsa_db> salsa::jar::Jar<'salsa_db> for VfsJar {
    type DynDb = dyn VfsDb + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <PackagePath as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i1 = <CratePath as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i2 = <ModulePath as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i3 = <apparent_ancestry as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i4 = <path_menu as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i5 = <AbsolutePath as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i6 = <File as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i7 = <package_dir as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i8 =
            <module_absolute_path as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i9 =
            <package_manifest_file as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i10 = <module_file as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i11 = <Toolchain as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i12 =
            <PublishedToolchain as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i13 =  <published_toolchain_library_path as salsa::storage::IngredientsFor> ::create_ingredients(routes);
        let i14 = <current_toolchain as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(
            i0,
            i1,
            i2,
            i3,
            i4,
            i5,
            i6,
            i7,
            i8,
            i9,
            i10,
            i11,
            i12,
            i13,
            i14,
            Default::default(),
        )
    }
}

impl VfsJar {
    pub(crate) fn cache(&self) -> &VfsCache {
        &self.15
    }

    pub(crate) fn set_watcher(&mut self, watcher: VfsWatcher) {
        self.15.set_watcher(watcher)
    }
}

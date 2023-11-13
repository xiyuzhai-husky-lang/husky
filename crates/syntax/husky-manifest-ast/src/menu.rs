use husky_coword::Coword;


use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestAstMenu {
    package_coword: Coword,
    dependencies_coword: Coword,
    dev_dependencies_coword: Coword,
    features_coword: Coword,
}

#[salsa::tracked(jar = ManifestAstJar, return_ref)]
pub(crate) fn manifest_ast_menu(db: &dyn ManifestAstDb) -> ManifestAstMenu {
    ManifestAstMenu::new(db)
}

impl ManifestAstMenu {
    fn new(db: &dyn ManifestAstDb) -> Self {
        let package_coword = Coword::from_ref(db, "package");
        let dependencies_coword = Coword::from_ref(db, "dependencies");
        let dev_dependencies_coword = Coword::from_ref(db, "dev-dependencies");
        let features_coword = Coword::from_ref(db, "features");
        Self {
            package_coword,
            dependencies_coword,
            dev_dependencies_coword,
            features_coword,
        }
    }

    pub(crate) fn package_coword(&self) -> Coword {
        self.package_coword
    }

    pub(crate) fn dependencies_coword(&self) -> Coword {
        self.dependencies_coword
    }

    pub(crate) fn dev_dependencies_coword(&self) -> Coword {
        self.dev_dependencies_coword
    }

    pub(crate) fn features_coword(&self) -> Coword {
        self.features_coword
    }
}

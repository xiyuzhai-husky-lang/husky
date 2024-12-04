use husky_coword::BaseCoword;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestAstMenu {
    package_coword: BaseCoword,
    dependencies_coword: BaseCoword,
    dev_dependencies_coword: BaseCoword,
    features_coword: BaseCoword,
}

#[salsa::tracked(return_ref)]
pub(crate) fn manifest_ast_menu(db: &::salsa::Db) -> ManifestAstMenu {
    ManifestAstMenu::new(db)
}

impl ManifestAstMenu {
    fn new(db: &::salsa::Db) -> Self {
        let package_coword = BaseCoword::from_ref("package", db);
        let dependencies_coword = BaseCoword::from_ref("dependencies", db);
        let dev_dependencies_coword = BaseCoword::from_ref("dev-dependencies", db);
        let features_coword = BaseCoword::from_ref("features", db);
        Self {
            package_coword,
            dependencies_coword,
            dev_dependencies_coword,
            features_coword,
        }
    }

    pub(crate) fn package_coword(&self) -> BaseCoword {
        self.package_coword
    }

    pub(crate) fn dependencies_coword(&self) -> BaseCoword {
        self.dependencies_coword
    }

    pub(crate) fn dev_dependencies_coword(&self) -> BaseCoword {
        self.dev_dependencies_coword
    }

    pub(crate) fn features_coword(&self) -> BaseCoword {
        self.features_coword
    }
}

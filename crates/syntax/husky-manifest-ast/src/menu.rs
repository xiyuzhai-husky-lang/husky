use husky_word::Word;
use smallvec::smallvec;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ManifestAstMenu {
    package_section_tile: TomlSectionTitle,
    dependencies_section_tile: TomlSectionTitle,
    dev_dependencies_section_tile: TomlSectionTitle,
    features_section_tile: TomlSectionTitle,
}

#[salsa::tracked(jar = ManifestAstJar, return_ref)]
pub(crate) fn manifest_ast_menu(db: &dyn ManifestAstDb) -> ManifestAstMenu {
    ManifestAstMenu::new(db)
}

impl ManifestAstMenu {
    fn new(db: &dyn ManifestAstDb) -> Self {
        let package = Word::from_ref(db, "package");
        let dependencies = Word::from_ref(db, "dependencies");
        let dev_dependencies = Word::from_ref(db, "dev-dependencies");
        let features = Word::from_ref(db, "features");
        Self {
            package_section_tile: TomlSectionTitle::new(db, smallvec![package]),
            dependencies_section_tile: TomlSectionTitle::new(db, smallvec![dependencies]),
            dev_dependencies_section_tile: TomlSectionTitle::new(db, smallvec![dev_dependencies]),
            features_section_tile: TomlSectionTitle::new(db, smallvec![features]),
        }
    }
}

impl ManifestAstMenu {
    pub(crate) fn package_section_tile(&self) -> TomlSectionTitle {
        self.package_section_tile
    }

    pub(crate) fn dependencies_section_tile(&self) -> TomlSectionTitle {
        self.dependencies_section_tile
    }

    pub(crate) fn dev_dependencies_section_tile(&self) -> TomlSectionTitle {
        self.dev_dependencies_section_tile
    }

    pub(crate) fn features_section_tile(&self) -> TomlSectionTitle {
        self.features_section_tile
    }
}

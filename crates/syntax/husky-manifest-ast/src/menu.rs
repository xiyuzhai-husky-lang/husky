use husky_word::Word;
use smallvec::smallvec;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestAstMenu {
    package_word: Word,
    dependencies_word: Word,
    dev_dependencies_word: Word,
    features_word: Word,
}

#[salsa::tracked(jar = ManifestAstJar, return_ref)]
pub(crate) fn manifest_ast_menu(db: &dyn ManifestAstDb) -> ManifestAstMenu {
    ManifestAstMenu::new(db)
}

impl ManifestAstMenu {
    fn new(db: &dyn ManifestAstDb) -> Self {
        let package_word = Word::from_ref(db, "package");
        let dependencies_word = Word::from_ref(db, "dependencies");
        let dev_dependencies_word = Word::from_ref(db, "dev-dependencies");
        let features_word = Word::from_ref(db, "features");
        Self {
            package_word,
            dependencies_word,
            dev_dependencies_word,
            features_word,
        }
    }

    pub(crate) fn package_word(&self) -> Word {
        self.package_word
    }

    pub(crate) fn dependencies_word(&self) -> Word {
        self.dependencies_word
    }

    pub(crate) fn dev_dependencies_word(&self) -> Word {
        self.dev_dependencies_word
    }

    pub(crate) fn features_word(&self) -> Word {
        self.features_word
    }
}

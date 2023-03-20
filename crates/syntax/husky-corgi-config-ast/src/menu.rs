use husky_word::Word;
use smallvec::smallvec;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CorgiConfigAstMenu {
    registry_word: Word,
}

#[salsa::tracked(jar = CorgiConfigAstJar, return_ref)]
pub(crate) fn corgi_config_ast_menu(db: &dyn CorgiConfigAstDb) -> CorgiConfigAstMenu {
    CorgiConfigAstMenu::new(db)
}

impl CorgiConfigAstMenu {
    fn new(db: &dyn CorgiConfigAstDb) -> Self {
        let registry_word = Word::from_ref(db, "registry");
        Self { registry_word }
    }
}

impl CorgiConfigAstMenu {
    pub(crate) fn registry_word(&self) -> Word {
        self.registry_word
    }
}

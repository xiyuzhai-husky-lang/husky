use husky_word::Word;
use smallvec::smallvec;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CorgiConfigAstMenu {
    registry_word: Word,
    path_word: Word,
}

impl CorgiConfigAstMenu {
    fn new(db: &dyn CorgiConfigAstDb) -> Self {
        let registry_word = Word::from_ref(db, "registry");
        let path_word = Word::from_ref(db, "path");
        Self {
            registry_word,
            path_word,
        }
    }
}

impl CorgiConfigAstMenu {
    pub(crate) fn registry_word(&self) -> Word {
        self.registry_word
    }

    pub(crate) fn path_word(&self) -> Word {
        self.path_word
    }
}

#[salsa::tracked(jar = CorgiConfigAstJar, return_ref)]
pub(crate) fn corgi_config_ast_menu(db: &dyn CorgiConfigAstDb) -> CorgiConfigAstMenu {
    CorgiConfigAstMenu::new(db)
}

#[test]
fn corgi_config_ast_menu_works() {
    let db = DB::default();
    let menu = corgi_config_ast_menu(&db);
    assert_eq!(menu.registry_word().data(&db), "registry");
    assert_eq!(menu.path_word().data(&db), "path");
}

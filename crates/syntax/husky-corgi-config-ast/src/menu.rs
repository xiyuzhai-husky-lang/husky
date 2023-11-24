use husky_coword::Coword;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CorgiConfigAstMenu {
    registry_coword: Coword,
    path_coword: Coword,
}

impl CorgiConfigAstMenu {
    fn new(db: &dyn CorgiConfigAstDb) -> Self {
        let registry_coword = Coword::from_ref(db, "registry");
        let path_coword = Coword::from_ref(db, "path");
        Self {
            registry_coword,
            path_coword,
        }
    }
}

impl CorgiConfigAstMenu {
    pub(crate) fn registry_coword(&self) -> Coword {
        self.registry_coword
    }

    pub(crate) fn path_coword(&self) -> Coword {
        self.path_coword
    }
}

#[salsa::tracked(jar = CorgiConfigAstJar, return_ref)]
pub(crate) fn corgi_config_ast_menu(db: &dyn CorgiConfigAstDb) -> CorgiConfigAstMenu {
    CorgiConfigAstMenu::new(db)
}

#[test]
fn corgi_config_ast_menu_works() {
    let db = DB::default();
    let db = &*db;
    let menu = corgi_config_ast_menu(db);
    assert_eq!(menu.registry_coword().data(db), "registry");
    assert_eq!(menu.path_coword().data(db), "path");
}

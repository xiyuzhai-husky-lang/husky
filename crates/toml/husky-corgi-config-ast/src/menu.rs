use husky_coword::BaseCoword;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CorgiConfigAstMenu {
    registry_coword: BaseCoword,
    path_coword: BaseCoword,
}

impl CorgiConfigAstMenu {
    fn new(db: &::salsa::Db) -> Self {
        let registry_coword = BaseCoword::from_ref("registry", db);
        let path_coword = BaseCoword::from_ref("path", db);
        Self {
            registry_coword,
            path_coword,
        }
    }
}

impl CorgiConfigAstMenu {
    pub(crate) fn registry_coword(&self) -> BaseCoword {
        self.registry_coword
    }

    pub(crate) fn path_coword(&self) -> BaseCoword {
        self.path_coword
    }
}

#[salsa::tracked(jar = CorgiConfigAstJar, return_ref)]
pub(crate) fn corgi_config_ast_menu(db: &::salsa::Db) -> CorgiConfigAstMenu {
    CorgiConfigAstMenu::new(db)
}

#[test]
fn corgi_config_ast_menu_works() {
    let db = DB::default();
    let db = &*db;
    let menu = corgi_config_ast_menu(db);
    assert_eq!(menu.registry_coword().data(), "registry");
    assert_eq!(menu.path_coword().data(), "path");
}

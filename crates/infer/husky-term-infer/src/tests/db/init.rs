use husky_term::TyFamily;

use super::*;

impl TermInferTestsDb {
    pub(super) fn init(&mut self) {
        let entity_path_menu = self.entity_path_menu();
        let term_menu = self.term_menu();
        // self.init_entity_tys(&entity_path_menu, &term_menu);
        // self.init_decls(&entity_path_menu);
        // self.init_prelude_symbols(&entity_path_menu)
    }
}

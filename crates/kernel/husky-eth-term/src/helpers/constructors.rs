use crate::{term::EthTerm, EthTermDb};
use husky_entity_path::menu::item_path_menu;

impl EthTerm {
    pub fn leashed(self, db: &::salsa::Db) -> Self {
        EthTerm::new_ty_ontology(
            db,
            item_path_menu(
                db,
                self.toolchain(db)
                    .expect("any term valid as an argument to leash has a toolchain"),
            )
            .leash_ty_path(),
            [self],
        )
        .unwrap()
    }
}

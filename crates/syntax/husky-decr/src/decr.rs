mod derive;

use crate::*;
pub use derive::*;
use husky_ast::{AstIdx, DecrId};
use husky_entity_tree::EntityTreeResult;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DecrDb)]
pub enum Decr {
    Derive(DeriveDecr),
}

impl Decr {
    fn new(
        db: &dyn DecrDb,
        entity_path: EntityPath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        decr_id: DecrId,
    ) -> DecrResult<Self> {
        let word_menu = db.word_menu();
        let decr_ident = decr_id.ident();
        Ok(if decr_ident == word_menu.derive_ident() {
            Decr::Derive(DeriveDecr::new(
                db,
                entity_path,
                ast_idx,
                token_group_idx,
                decr_id,
            )?)
        } else {
            todo!()
        })
    }
}

pub trait HasDecrs: Copy {
    fn decrs<'a>(self, db: &'a dyn DecrDb) -> DecrResultRef<&'a [Decr]>;
}

impl HasDecrs for TypePath {
    fn decrs<'a>(self, db: &'a dyn DecrDb) -> DecrResultRef<&'a [Decr]> {
        ty_path_decrs(db, self).as_ref().map(|v| v as &[_])
    }
}

#[salsa::tracked(jar = DecrJar, return_ref)]
pub(crate) fn ty_path_decrs(db: &dyn DecrDb, path: TypePath) -> DecrResult<Vec<Decr>> {
    let ident = path.ident(db);
    let module_path = path.module_path(db);
    let module_entity_tree = db.entity_tree_sheet(module_path)?;
    let ast_sheet = db.ast_sheet(module_path)?;
    let Some(entity_symbol) = module_entity_tree
        .module_symbols()
        .resolve_ident(ident)
        else {
            use salsa::DisplayWithDb;
            panic!(r#"
Path `{}` is invalid!
This is very likely caused by expect item in standard library.
"#, path.display(db))
        };
    let module_item_symbol = entity_symbol.module_item_symbol().unwrap();
    let ast_idx: AstIdx = module_item_symbol.ast_idx(db);
    ast_sheet.decrs(ast_idx, |ast_idx, token_group_idx, decr_id| {
        Decr::new(db, path.into(), ast_idx, token_group_idx, decr_id)
    })
}

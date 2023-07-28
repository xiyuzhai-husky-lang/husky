mod derive;

pub use derive::*;

use crate::*;
use husky_ast::{AstIdx, DecrId};
use husky_entity_syn_tree::{HasSynNodePath, ItemSynTreeResult};
use husky_print_utils::p;
use husky_scope::ReferenceModulePath;
use salsa::DebugWithDb;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = DecrDb)]
pub enum Decr {
    Derive(DeriveDecr),
}

impl Decr {
    fn new(
        db: &dyn DecrDb,
        item_path: ItemPath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        decr_id: DecrId,
    ) -> DecrResult<Self> {
        let coword_menu = db.coword_menu();
        let decr_ident = decr_id.ident();
        Ok(if decr_ident == coword_menu.derive_ident() {
            Decr::Derive(DeriveDecr::new(
                db,
                item_path,
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

#[salsa::tracked(jar = SynDecrJar, return_ref)]
pub(crate) fn ty_path_decrs(db: &dyn DecrDb, path: TypePath) -> DecrResult<Vec<Decr>> {
    let module_path = path.module_path(db);
    let ast_sheet = db.ast_sheet(module_path)?;
    let ast_idx = path.syn_node_path(db).node(db).ast_idx(db);
    ast_sheet.decrs(
        ast_idx,
        |ast_idx, token_group_idx, decr_id| {
            Decr::new(db, path.into(), ast_idx, token_group_idx, decr_id)
        },
        || DerivedDecrError::InvalidParent.into(),
    )
}

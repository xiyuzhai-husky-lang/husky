use std::sync::Arc;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu {
    i32: TermPtr,
    i64: TermPtr,
}

pub(crate) fn term_menu(db: &dyn TermQuery) -> Arc<TermMenu> {
    Arc::new(TermMenu {
        i32: TermEntity::i32(db),
        i64: TermEntity::i64(db),
    })
}

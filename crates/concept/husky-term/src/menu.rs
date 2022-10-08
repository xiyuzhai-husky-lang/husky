use std::sync::Arc;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu {
    i32: Ty,
    i64: Ty,
    i32_literal_0: TermPtr,
    i32_literal_1: TermPtr,
    i64_literal_0: TermPtr,
    i64_literal_1: TermPtr,
}

pub(crate) fn term_menu(db: &dyn TermQuery) -> Arc<TermMenu> {
    let i32 = Ty::i32(db);
    let i64 = Ty::i64(db);
    Arc::new(TermMenu {
        i32,
        i64,
        i32_literal_0: TermLiteral::i32_literal(db, 0, i32),
        i32_literal_1: TermLiteral::i32_literal(db, 1, i32),
        i64_literal_0: TermLiteral::i64_literal(db, 0, i64),
        i64_literal_1: TermLiteral::i64_literal(db, 1, i64),
    })
}

mod menu0;
mod menu1;
mod menu2;

pub use menu0::*;
pub use menu1::*;
pub use menu2::*;

use crate::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu {
    parent: TermMenu2,
    i32_literal_0: TermPtr,
    i32_literal_1: TermPtr,
    i64_literal_0: TermPtr,
    i64_literal_1: TermPtr,
}
impl std::ops::Deref for TermMenu {
    type Target = TermMenu2;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
pub(crate) fn term_menu(db: &dyn TermDb) -> Arc<TermMenu> {
    let menu0 = TermMenu0::new(db);
    let menu1 = TermMenu1::new(db, menu0);
    let menu2 = TermMenu2::new(db, menu1);
    Arc::new(TermMenu {
        i32_literal_0: TermLiteral::i32_literal(db, 0, &menu2),
        i32_literal_1: TermLiteral::i32_literal(db, 1, &menu2),
        i64_literal_0: TermLiteral::i64_literal(db, 0, &menu2),
        i64_literal_1: TermLiteral::i64_literal(db, 1, &menu2),
        parent: menu2,
    })
}

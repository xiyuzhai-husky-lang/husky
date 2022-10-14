use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu3 {
    i32_literal_0: TermPtr,
    i32_literal_1: TermPtr,
    i64_literal_0: TermPtr,
    i64_literal_1: TermPtr,
    parent: TermMenu2,
}

impl std::ops::Deref for TermMenu3 {
    type Target = TermMenu2;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu3 {
    pub(crate) fn new(db: &dyn TermDb, menu2: TermMenu2) -> Self {
        let menu0 = TermMenu0::new(db);
        let menu1 = TermMenu1::new(db, menu0);
        let menu2 = TermMenu2::new(db, menu1);
        TermMenu3 {
            i32_literal_0: TermLiteral::i32_literal(db, 0, &menu2),
            i32_literal_1: TermLiteral::i32_literal(db, 1, &menu2),
            i64_literal_0: TermLiteral::i64_literal(db, 0, &menu2),
            i64_literal_1: TermLiteral::i64_literal(db, 1, &menu2),
            parent: menu2,
        }
    }
}
type A = core::primitive::i32;

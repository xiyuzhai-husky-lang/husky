use husky_sema_opr::prefix::SemaPrefixOpr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirPrefixOpr {
    Minus,
    Not,
    BitNot,
    TakeRef,
    Deref,
}

impl HirPrefixOpr {
    pub fn from_sema(opr: SemaPrefixOpr) -> Self {
        match opr {
            SemaPrefixOpr::Minus => HirPrefixOpr::Minus,
            SemaPrefixOpr::Not => HirPrefixOpr::Not,
            SemaPrefixOpr::BitNot => HirPrefixOpr::BitNot,
            SemaPrefixOpr::LeashType | SemaPrefixOpr::RefType | SemaPrefixOpr::Option => {
                unreachable!()
            }
        }
    }
}

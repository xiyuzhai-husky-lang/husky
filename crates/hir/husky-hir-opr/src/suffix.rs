use husky_sema_opr::suffix::SemaSuffixOpr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirSuffixOpr {
    Incr,
    Decr,
}

impl HirSuffixOpr {
    pub fn from_sema(opr: SemaSuffixOpr) -> Self {
        match opr {
            SemaSuffixOpr::Incr => HirSuffixOpr::Incr,
            SemaSuffixOpr::Decr => HirSuffixOpr::Decr,
            SemaSuffixOpr::ComposeWithOption => unreachable!(),
            SemaSuffixOpr::ComposeWithNot => unreachable!(),
        }
    }
}

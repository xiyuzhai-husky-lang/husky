use husky_sema_opr::suffix::SemaSuffixOpr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirSuffixOpr {
    Incr,
    Decr,
    Unveil,
    Unwrap,
}

impl HirSuffixOpr {
    pub fn from_sema(opr: SemaSuffixOpr) -> Self {
        match opr {
            SemaSuffixOpr::Incr => HirSuffixOpr::Incr,
            SemaSuffixOpr::Decr => HirSuffixOpr::Decr,
            SemaSuffixOpr::Unveil => HirSuffixOpr::Unveil,
            SemaSuffixOpr::ComposeWithOption => unreachable!(),
            SemaSuffixOpr::Unwrap => HirSuffixOpr::Unwrap,
            SemaSuffixOpr::ComposeWithNot => unreachable!(),
        }
    }
}

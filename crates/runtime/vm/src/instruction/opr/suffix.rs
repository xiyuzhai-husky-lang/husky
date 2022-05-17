use entity_route::RangedEntityRoute;

use super::*;

impl From<SuffixOpr> for Opr {
    fn from(suffix: SuffixOpr) -> Self {
        Self::Suffix(suffix)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SuffixOpr {
    Incr,                                // ++
    Decr,                                // --
    MayReturn,                           // ?
    FieldAccess(RangedCustomIdentifier), // .
    WithTy(EntityRoutePtr),              // :
    AsTy(RangedEntityRoute),             // :
}

impl SuffixOpr {
    pub fn act_on_primitive(&self, opd: PrimitiveValue) -> PrimitiveValue {
        todo!()
    }

    pub fn code(self) -> String {
        match self {
            SuffixOpr::Incr => "++".into(),
            SuffixOpr::Decr => "--".into(),
            SuffixOpr::MayReturn => "?".into(),
            SuffixOpr::FieldAccess(ident) => format!(".{}", ident.ident),
            SuffixOpr::WithTy(ty) => format!(": {}", ty),
            SuffixOpr::AsTy(ty) => format!(" as {}", ty.route),
        }
    }
}

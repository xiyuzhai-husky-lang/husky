use husky_static_defn::MethodStaticDefnKind;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MethodDefnKind {
    TypeMethod { ty: Ty },
    TraitMethod { trai: Ty },
    TraitMethodImpl { trai: Ty },
}

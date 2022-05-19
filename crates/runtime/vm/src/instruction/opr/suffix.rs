use entity_route::RangedEntityRoute;
use word::RootIdentifier;

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
    // pub fn act(self, stack_value: StackValue) -> PrimitiveValue {
    //     match self {
    //         SuffixOpr::Incr => todo!(),
    //         SuffixOpr::Decr => todo!(),
    //         SuffixOpr::MayReturn => todo!(),
    //         SuffixOpr::FieldAccess(_) => todo!(),
    //         SuffixOpr::WithTy(_) => todo!(),
    //         SuffixOpr::AsTy(ty) => match ty.route {
    //             EntityRoutePtr::Root(ty_ident) => match ty_ident {
    //                 RootIdentifier::Void => todo!(),
    //                 RootIdentifier::I32 => match stack_value {
    //                     StackValue::Moved => todo!(),
    //                     StackValue::Primitive(_) => todo!(),
    //                     StackValue::Boxed(_) => todo!(),
    //                     StackValue::GlobalPure(_) => todo!(),
    //                     StackValue::GlobalRef(_) => todo!(),
    //                     StackValue::LocalRef { value, owner, gen } => todo!(),
    //                     StackValue::LocalRefMut { value, owner, gen } => todo!(),
    //                 },
    //                 RootIdentifier::F32 => todo!(),
    //                 RootIdentifier::B32 => todo!(),
    //                 RootIdentifier::B64 => todo!(),
    //                 RootIdentifier::Bool => todo!(),
    //                 _ => todo!(),
    //             },
    //             EntityRoutePtr::Custom(_) => todo!(),
    //             EntityRoutePtr::ThisType => todo!(),
    //         },
    //     }
    // }

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

use word::BuiltinIdentifier;

use crate::*;

pub(crate) fn is_copyable(db: &dyn InferContractSalsaQueryGroup, ty: EntityRoutePtr) -> bool {
    match ty {
        EntityRoutePtr::Builtin(builtin_ident) => match builtin_ident {
            BuiltinIdentifier::Void
            | BuiltinIdentifier::I32
            | BuiltinIdentifier::F32
            | BuiltinIdentifier::B32
            | BuiltinIdentifier::B64
            | BuiltinIdentifier::Bool => true,
            BuiltinIdentifier::True => todo!(),
            BuiltinIdentifier::False => todo!(),
            BuiltinIdentifier::Vec => todo!(),
            BuiltinIdentifier::Tuple => todo!(),
            BuiltinIdentifier::Debug => todo!(),
            BuiltinIdentifier::Std => todo!(),
            BuiltinIdentifier::Core => todo!(),
            BuiltinIdentifier::Fp => todo!(),
            BuiltinIdentifier::Fn => todo!(),
            BuiltinIdentifier::FnMut => todo!(),
            BuiltinIdentifier::FnOnce => todo!(),
            BuiltinIdentifier::Array => todo!(),
            BuiltinIdentifier::DatasetType => todo!(),
            BuiltinIdentifier::Type => todo!(),
            BuiltinIdentifier::Datasets => todo!(),
            BuiltinIdentifier::CloneTrait => todo!(),
            BuiltinIdentifier::CopyTrait => todo!(),
            BuiltinIdentifier::PartialEqTrait => todo!(),
            BuiltinIdentifier::EqTrait => todo!(),
        },
        EntityRoutePtr::Custom(_) => todo!(),
    }
}

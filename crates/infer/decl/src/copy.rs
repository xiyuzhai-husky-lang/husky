use print_utils::{emsg_once, p};
use word::RootIdentifier;

use crate::*;

pub(crate) fn is_copyable(db: &dyn DeclQueryGroup, ty: EntityRoutePtr) -> InferResult<bool> {
    match ty {
        EntityRoutePtr::Root(builtin_ident) => Ok(match builtin_ident {
            RootIdentifier::Void
            | RootIdentifier::I32
            | RootIdentifier::F32
            | RootIdentifier::B32
            | RootIdentifier::B64
            | RootIdentifier::Bool => true,
            RootIdentifier::Vec => false,
            RootIdentifier::True => todo!(),
            RootIdentifier::False => todo!(),
            RootIdentifier::Tuple => todo!(),
            RootIdentifier::Debug => todo!(),
            RootIdentifier::Std => todo!(),
            RootIdentifier::Core => todo!(),
            RootIdentifier::Fp => todo!(),
            RootIdentifier::Fn => todo!(),
            RootIdentifier::FnMut => todo!(),
            RootIdentifier::FnOnce => todo!(),
            RootIdentifier::Array => todo!(),
            RootIdentifier::DatasetType => todo!(),
            RootIdentifier::TypeType => false,
            RootIdentifier::Datasets => todo!(),
            RootIdentifier::CloneTrait => todo!(),
            RootIdentifier::CopyTrait => todo!(),
            RootIdentifier::PartialEqTrait => todo!(),
            RootIdentifier::EqTrait => todo!(),
            RootIdentifier::ModuleType => todo!(),
        }),
        EntityRoutePtr::Custom(_) => {
            let ty_decl = db.ty_decl(ty)?;
            Ok(ty_decl
                .trai_impl(db.entity_route_menu().copy_trait)
                .is_some())
        }
        EntityRoutePtr::ThisType => todo!(),
    }
}

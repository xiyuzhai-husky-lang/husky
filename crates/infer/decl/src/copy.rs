use print_utils::msg_once;
use word::RootIdentifier;

use crate::*;

pub(crate) fn is_copy_constructible(db: &dyn DeclQueryGroup, ty: EntityRoutePtr) -> bool {
    match ty {
        EntityRoutePtr::Root(builtin_ident) => match builtin_ident {
            RootIdentifier::Void
            | RootIdentifier::I32
            | RootIdentifier::F32
            | RootIdentifier::B32
            | RootIdentifier::B64
            | RootIdentifier::Bool => true,
            RootIdentifier::True => todo!(),
            RootIdentifier::False => todo!(),
            RootIdentifier::Vec => todo!(),
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
            RootIdentifier::Type => todo!(),
            RootIdentifier::Datasets => todo!(),
            RootIdentifier::CloneTrait => todo!(),
            RootIdentifier::CopyTrait => todo!(),
            RootIdentifier::PartialEqTrait => todo!(),
            RootIdentifier::EqTrait => todo!(),
        },
        EntityRoutePtr::Custom(_) => {
            msg_once!("handle reference to prevent looping");
            let ty_decl = db.type_decl(ty).unwrap();
            todo!()
        }
        EntityRoutePtr::ThisType => todo!(),
    }
}

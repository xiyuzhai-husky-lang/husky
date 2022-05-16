use upcast::Upcast;

use crate::*;

#[salsa::query_group(InferEntityRouteQueryGroupStorage)]
pub trait InferEntityRouteQueryGroup: DeclQueryGroup + Upcast<dyn DeclQueryGroup> {
    // fn scope_ty(&self, scope: EntityRoutePtr) -> InferResult<EntityRoutePtr>;
    // fn enum_literal_value(&self, scope: EntityRoutePtr) -> EnumLiteralValue;
    fn entity_route_sheet(&self, file: FilePtr) -> EntitySyntaxResultArc<EntityRouteSheet>;

    fn is_implicitly_castable(&self, src_ty: EntityRoutePtr, dst_ty: EntityRoutePtr) -> bool;
    fn is_explicitly_castable(&self, src_ty: EntityRoutePtr, dst_ty: EntityRoutePtr) -> bool;
}

fn is_implicitly_castable(
    db: &dyn InferEntityRouteQueryGroup,
    src_ty: EntityRoutePtr,
    dst_ty: EntityRoutePtr,
) -> bool {
    if src_ty == dst_ty {
        return true;
    }
    match dst_ty {
        EntityRoutePtr::Root(builtin_ident) => match builtin_ident {
            RootIdentifier::Void => false,
            RootIdentifier::I32 => false,
            RootIdentifier::F32 => false,
            RootIdentifier::B32 => false,
            RootIdentifier::B64 => false,
            RootIdentifier::Bool => match src_ty {
                EntityRoutePtr::Root(builtin_ident) => match builtin_ident {
                    RootIdentifier::I32
                    | RootIdentifier::F32
                    | RootIdentifier::B32
                    | RootIdentifier::B64
                    | RootIdentifier::Bool => true,
                    RootIdentifier::Void
                    | RootIdentifier::List
                    | RootIdentifier::Tuple
                    | RootIdentifier::Fp
                    | RootIdentifier::Fn
                    | RootIdentifier::FnMut
                    | RootIdentifier::FnOnce
                    | RootIdentifier::Array
                    | RootIdentifier::DatasetType
                    | RootIdentifier::Type => false,
                    _ => panic!(),
                },
                EntityRoutePtr::Custom(_) => todo!(),
                EntityRoutePtr::ThisType => todo!(),
            },
            RootIdentifier::True => todo!(),
            RootIdentifier::False => todo!(),
            RootIdentifier::List => todo!(),
            RootIdentifier::Tuple => todo!(),
            RootIdentifier::Debug => todo!(),
            RootIdentifier::Std => todo!(),
            RootIdentifier::Core => todo!(),
            RootIdentifier::Fp => todo!(),
            RootIdentifier::Fn => todo!(),
            RootIdentifier::FnMut => todo!(),
            RootIdentifier::FnOnce => todo!(),
            RootIdentifier::Array => todo!(),
            RootIdentifier::DatasetType => match src_ty {
                EntityRoutePtr::Root(RootIdentifier::DatasetType) => true,
                EntityRoutePtr::Custom(scope) => match scope.kind {
                    EntityRouteKind::Root {
                        ident: RootIdentifier::DatasetType,
                    } => true,
                    _ => false,
                },
                _ => false,
            },
            RootIdentifier::Type => todo!(),
            RootIdentifier::Datasets => panic!(),
            RootIdentifier::CloneTrait => todo!(),
            RootIdentifier::CopyTrait => todo!(),
            RootIdentifier::PartialEqTrait => todo!(),
            RootIdentifier::EqTrait => todo!(),
        },
        EntityRoutePtr::Custom(_) => {
            msg_once!("handle convertible");
            false
        }
        EntityRoutePtr::ThisType => todo!(),
    }
}

fn is_explicitly_castable(
    db: &dyn InferEntityRouteQueryGroup,
    src_ty: EntityRoutePtr,
    dst_ty: EntityRoutePtr,
) -> bool {
    todo!()
}

use entity_kind::TyKind;
use upcast::Upcast;

use crate::*;

#[salsa::query_group(InferEntityRouteQueryGroupStorage)]
pub trait InferEntityRouteQueryGroup: DeclQueryGroup + Upcast<dyn DeclQueryGroup> {
    // fn scope_ty(&self, scope: EntityRoutePtr) -> InferResult<EntityRoutePtr>;
    // fn enum_literal_value(&self, scope: EntityRoutePtr) -> EnumLiteralValue;
    fn entity_route_sheet(&self, file: FilePtr) -> EntitySyntaxResultArc<EntityRouteSheet>;

    fn is_implicitly_castable(&self, src_ty: EntityRoutePtr, dst_ty: EntityRoutePtr) -> bool;
    fn is_explicitly_castable(
        &self,
        src_ty: EntityRoutePtr,
        dst_ty: EntityRoutePtr,
    ) -> InferResult<bool>;
}

fn is_implicitly_castable(
    db: &dyn InferEntityRouteQueryGroup,
    src_ty: EntityRoutePtr,
    dst_ty: EntityRoutePtr,
) -> bool {
    // deref
    let src_ty = src_ty.deref_route();
    let dst_ty = dst_ty.deref_route();
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
                    | RootIdentifier::Vec
                    | RootIdentifier::Tuple
                    | RootIdentifier::Fp
                    | RootIdentifier::Fn
                    | RootIdentifier::FnMut
                    | RootIdentifier::FnOnce
                    | RootIdentifier::Array
                    | RootIdentifier::DatasetType
                    | RootIdentifier::TypeType => false,
                    _ => panic!(),
                },
                EntityRoutePtr::Custom(_) => todo!(),
                EntityRoutePtr::ThisType => todo!(),
            },
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
            RootIdentifier::TypeType => todo!(),
            RootIdentifier::Datasets => panic!(),
            RootIdentifier::CloneTrait => todo!(),
            RootIdentifier::CopyTrait => todo!(),
            RootIdentifier::PartialEqTrait => todo!(),
            RootIdentifier::EqTrait => todo!(),
            RootIdentifier::ModuleType => todo!(),
            RootIdentifier::Ref => todo!(),
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
) -> InferResult<bool> {
    if src_ty == dst_ty {
        return Ok(true);
    }
    match (src_ty, dst_ty) {
        (EntityRoutePtr::Root(src_ty_ident), EntityRoutePtr::Root(dst_ty_ident)) => {
            return Ok(are_different_root_tys_explicity_castable(
                src_ty_ident,
                dst_ty_ident,
            ))
        }
        _ => (),
    }
    let src_ty_decl = db.ty_decl(src_ty)?;
    let dst_ty_decl = db.ty_decl(dst_ty)?;
    match src_ty_decl.kind {
        TyKind::Enum => match dst_ty {
            EntityRoutePtr::Root(dst_ty_ident) => match dst_ty_ident {
                RootIdentifier::I32 | RootIdentifier::B32 | RootIdentifier::B64 => Ok(true),
                _ => todo!(),
            },
            EntityRoutePtr::Custom(_) => todo!(),
            EntityRoutePtr::ThisType => todo!(),
        },
        TyKind::Record => todo!(),
        TyKind::Struct => todo!(),
        TyKind::Primitive => todo!(),
        TyKind::Vec => todo!(),
        TyKind::Array => todo!(),
        TyKind::Other => todo!(),
    }
}

fn are_different_root_tys_explicity_castable(
    src_ty: RootIdentifier,
    dst_ty: RootIdentifier,
) -> bool {
    match (src_ty, dst_ty) {
        (RootIdentifier::I32, RootIdentifier::F32)
        | (RootIdentifier::I32, RootIdentifier::Bool)
        | (RootIdentifier::I32, RootIdentifier::B32)
        | (RootIdentifier::I32, RootIdentifier::B64)
        | (RootIdentifier::B32, RootIdentifier::I32)
        | (RootIdentifier::B32, RootIdentifier::B64) => true,
        (RootIdentifier::B32, _) => todo!(),
        (RootIdentifier::B64, _) => todo!(),
        (RootIdentifier::Bool, _) => todo!(),
        (RootIdentifier::F32, _) => todo!(),
        _ => false,
    }
}

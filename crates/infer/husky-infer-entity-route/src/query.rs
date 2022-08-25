use husky_entity_kind::TyKind;
use upcast::Upcast;

use crate::*;

#[salsa::query_group(InferEntityRouteQueryGroupStorage)]
pub trait InferEntityRouteQueryGroup: DeclQueryGroup + Upcast<dyn DeclQueryGroup> {
    // fn scope_ty(&self, entity_route: EntityRoutePtr) -> InferResult<EntityRoutePtr>;
    // fn enum_literal_value(&self, entity_route: EntityRoutePtr) -> EnumLiteralValue;
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
    let src_ty = src_ty.intrinsic();
    let dst_ty = dst_ty.intrinsic();
    if src_ty == dst_ty {
        return true;
    }
    match dst_ty {
        EntityRoutePtr::Root(builtin_ident) => match builtin_ident {
            RootIdentifier::Void => false,
            RootIdentifier::I32 => false,
            RootIdentifier::I64 => false,
            RootIdentifier::F32 => false,
            RootIdentifier::F64 => false,
            RootIdentifier::B32 => false,
            RootIdentifier::B64 => false,
            RootIdentifier::Bool => match src_ty {
                EntityRoutePtr::Root(builtin_ident) => match builtin_ident {
                    RootIdentifier::I32
                    | RootIdentifier::I64
                    | RootIdentifier::F32
                    | RootIdentifier::F64
                    | RootIdentifier::B32
                    | RootIdentifier::B64
                    | RootIdentifier::Bool => true,
                    RootIdentifier::Void
                    | RootIdentifier::Vec
                    | RootIdentifier::Tuple
                    | RootIdentifier::ThickFp
                    | RootIdentifier::Fn
                    | RootIdentifier::FnMut
                    | RootIdentifier::FnOnce
                    | RootIdentifier::Array
                    | RootIdentifier::DatasetType
                    | RootIdentifier::TypeType => false,
                    _ => panic!(),
                },
                EntityRoutePtr::Custom(_) => false,
            },
            RootIdentifier::Vec => false,
            RootIdentifier::Tuple => false,
            RootIdentifier::ThickFp => false,
            RootIdentifier::Fn => false,
            RootIdentifier::FnMut => false,
            RootIdentifier::FnOnce => false,
            RootIdentifier::Array => false,
            RootIdentifier::DatasetType => match src_ty {
                EntityRoutePtr::Root(RootIdentifier::DatasetType) => true,
                EntityRoutePtr::Custom(scope) => match scope.variant {
                    EntityRouteVariant::Root {
                        ident: RootIdentifier::DatasetType,
                    } => true,
                    _ => false,
                },
                _ => false,
            },
            RootIdentifier::TypeType => false,
            RootIdentifier::ModuleType => false,
            RootIdentifier::Ref => false,
            _ => panic!(),
        },
        EntityRoutePtr::Custom(_) => {
            msg_once!("handle convertible");
            false
        }
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
    Ok(match src_ty_decl.ty_kind {
        TyKind::Enum => match dst_ty {
            EntityRoutePtr::Root(dst_ty_ident) => match dst_ty_ident {
                RootIdentifier::I32 | RootIdentifier::B32 | RootIdentifier::B64 => true,
                _ => todo!(),
            },
            EntityRoutePtr::Custom(_) => todo!(),
        },
        _ => false,
    })
}

fn are_different_root_tys_explicity_castable(
    src_ty: RootIdentifier,
    dst_ty: RootIdentifier,
) -> bool {
    match (src_ty, dst_ty) {
        (RootIdentifier::I32, RootIdentifier::I64)
        | (RootIdentifier::I32, RootIdentifier::F32)
        | (RootIdentifier::I32, RootIdentifier::F64)
        | (RootIdentifier::I32, RootIdentifier::Bool)
        | (RootIdentifier::I32, RootIdentifier::B32)
        | (RootIdentifier::I32, RootIdentifier::B64)
        | (RootIdentifier::I64, RootIdentifier::B64)
        | (RootIdentifier::I64, RootIdentifier::F64)
        | (RootIdentifier::B32, RootIdentifier::I32)
        | (RootIdentifier::B32, RootIdentifier::I64)
        | (RootIdentifier::B32, RootIdentifier::B64) => true,
        (RootIdentifier::B32, _) => false,
        (RootIdentifier::B64, _) => false,
        (RootIdentifier::Bool, _) => false,
        (RootIdentifier::F32, _) => false,
        _ => false,
    }
}

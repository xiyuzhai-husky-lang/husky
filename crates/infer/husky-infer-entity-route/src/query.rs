use husky_entity_kind::TyKind;
use upcast::Upcast;

use crate::*;

#[salsa::query_group(InferEntityRouteQueryGroupStorage)]
pub trait InferEntityRouteQueryGroup: DeclQueryGroup + Upcast<dyn DeclQueryGroup> {
    // fn scope_ty(&self, entity_route: EntityRoutePtr) -> InferResult<EntityRoutePtr>;
    // fn enum_literal_value(&self, entity_route: EntityRoutePtr) -> EnumLiteralValue;
    fn entity_route_sheet(&self, file: FileItd) -> EntitySyntaxResultArc<EntityRouteSheet>;

    fn is_implicitly_castable(&self, src_ty: EntityRoutePtr, dst_ty: EntityRoutePtr) -> bool;
    fn is_explicitly_castable(
        &self,
        src_ty: EntityRoutePtr,
        dst_ty: EntityRoutePtr,
    ) -> InferResult<bool>;
}

fn is_implicitly_castable(
    _db: &dyn InferEntityRouteQueryGroup,
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
            RootBuiltinIdentifier::Void => false,
            RootBuiltinIdentifier::I32 => false,
            RootBuiltinIdentifier::I64 => false,
            RootBuiltinIdentifier::F32 => false,
            RootBuiltinIdentifier::F64 => false,
            RootBuiltinIdentifier::B32 => false,
            RootBuiltinIdentifier::B64 => false,
            RootBuiltinIdentifier::Bool => match src_ty {
                EntityRoutePtr::Root(builtin_ident) => match builtin_ident {
                    RootBuiltinIdentifier::I32
                    | RootBuiltinIdentifier::I64
                    | RootBuiltinIdentifier::F32
                    | RootBuiltinIdentifier::F64
                    | RootBuiltinIdentifier::B32
                    | RootBuiltinIdentifier::B64
                    | RootBuiltinIdentifier::Bool => true,
                    RootBuiltinIdentifier::Void
                    | RootBuiltinIdentifier::Vec
                    | RootBuiltinIdentifier::Tuple
                    | RootBuiltinIdentifier::ThickFp
                    | RootBuiltinIdentifier::Fn
                    | RootBuiltinIdentifier::FnMut
                    | RootBuiltinIdentifier::FnOnce
                    | RootBuiltinIdentifier::Array
                    | RootBuiltinIdentifier::DatasetType
                    | RootBuiltinIdentifier::TypeType => false,
                    _ => panic!(),
                },
                EntityRoutePtr::Custom(_) => false,
            },
            RootBuiltinIdentifier::Vec => false,
            RootBuiltinIdentifier::Tuple => false,
            RootBuiltinIdentifier::ThickFp => false,
            RootBuiltinIdentifier::Fn => false,
            RootBuiltinIdentifier::FnMut => false,
            RootBuiltinIdentifier::FnOnce => false,
            RootBuiltinIdentifier::Array => false,
            RootBuiltinIdentifier::DatasetType => match src_ty {
                EntityRoutePtr::Root(RootBuiltinIdentifier::DatasetType) => true,
                EntityRoutePtr::Custom(scope) => match scope.variant {
                    EntityRouteVariant::Root {
                        ident: RootBuiltinIdentifier::DatasetType,
                    } => true,
                    _ => false,
                },
                _ => false,
            },
            RootBuiltinIdentifier::TypeType => false,
            RootBuiltinIdentifier::Module => false,
            RootBuiltinIdentifier::Ref => false,
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
    Ok(match src_ty_decl.ty_kind {
        TyKind::Enum => match dst_ty {
            EntityRoutePtr::Root(dst_ty_ident) => match dst_ty_ident {
                RootBuiltinIdentifier::I32
                | RootBuiltinIdentifier::B32
                | RootBuiltinIdentifier::B64 => true,
                _ => todo!(),
            },
            EntityRoutePtr::Custom(_) => todo!(),
        },
        _ => false,
    })
}

fn are_different_root_tys_explicity_castable(
    src_ty: RootBuiltinIdentifier,
    dst_ty: RootBuiltinIdentifier,
) -> bool {
    match (src_ty, dst_ty) {
        (RootBuiltinIdentifier::I32, RootBuiltinIdentifier::I64)
        | (RootBuiltinIdentifier::I32, RootBuiltinIdentifier::F32)
        | (RootBuiltinIdentifier::I32, RootBuiltinIdentifier::F64)
        | (RootBuiltinIdentifier::I32, RootBuiltinIdentifier::Bool)
        | (RootBuiltinIdentifier::I32, RootBuiltinIdentifier::B32)
        | (RootBuiltinIdentifier::I32, RootBuiltinIdentifier::B64)
        | (RootBuiltinIdentifier::I64, RootBuiltinIdentifier::B64)
        | (RootBuiltinIdentifier::I64, RootBuiltinIdentifier::F64)
        | (RootBuiltinIdentifier::B32, RootBuiltinIdentifier::I32)
        | (RootBuiltinIdentifier::B32, RootBuiltinIdentifier::I64)
        | (RootBuiltinIdentifier::B32, RootBuiltinIdentifier::B64) => true,
        (RootBuiltinIdentifier::B32, _) => false,
        (RootBuiltinIdentifier::B64, _) => false,
        (RootBuiltinIdentifier::Bool, _) => false,
        (RootBuiltinIdentifier::F32, _) => false,
        _ => false,
    }
}

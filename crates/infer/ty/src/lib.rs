mod builder;
mod sheet;

pub use sheet::*;

use ast::*;
use check_utils::*;
use defn_head::*;
use entity_route::{GenericArgument, *};
use entity_route_query::{EntityRouteQueryGroup, ScopeResult, ScopeResultArc};
use file::FilePtr;
use fold::FoldStorage;
use infer_decl::{DeclQueryGroup, TyDecl};
use infer_error::*;
use print_utils::*;
use syntax_types::*;
use vm::EnumLiteralValue;
use word::{ContextualIdentifier, RootIdentifier};

#[salsa::query_group(InferTyQueryGroupStorage)]
pub trait InferTySalsaQueryGroup:
    EntityRouteQueryGroup + ast::AstQueryGroup + DeclQueryGroup
{
    fn scope_ty(&self, scope: EntityRoutePtr) -> InferResult<EntityRoutePtr>;
    fn enum_literal_value(&self, scope: EntityRoutePtr) -> EnumLiteralValue;
    fn ty_sheet(&self, file: FilePtr) -> ScopeResultArc<TySheet>;

    fn is_implicit_convertible(&self, src_ty: EntityRoutePtr, dst_ty: EntityRoutePtr) -> bool;
}

pub trait InferTyQueryGroup: InferTySalsaQueryGroup {
    fn expr_ty_result(&self, file: FilePtr, expr_idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        self.ty_sheet(file)?.expr_ty_result(expr_idx)
    }

    fn expr_ty_decl(&self, file: FilePtr, expr_idx: RawExprIdx) -> InferResultArc<TyDecl> {
        let ty = self.expr_ty_result(file, expr_idx)?;
        self.ty_decl(ty)
    }
}

fn scope_ty(db: &dyn InferTySalsaQueryGroup, scope: EntityRoutePtr) -> InferResult<EntityRoutePtr> {
    match scope {
        EntityRoutePtr::Root(ident) => match ident {
            RootIdentifier::Void => todo!(),
            RootIdentifier::I32 => todo!(),
            RootIdentifier::F32 => todo!(),
            RootIdentifier::B32 => todo!(),
            RootIdentifier::B64 => todo!(),
            RootIdentifier::Bool => todo!(),
            RootIdentifier::True | RootIdentifier::False => {
                Ok(EntityRoutePtr::Root(RootIdentifier::Bool))
            }
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
        EntityRoutePtr::Custom(scope) => match scope.kind {
            EntityRouteKind::Input { main } => db.global_input_ty(main),
            _ => todo!(),
        },
        EntityRoutePtr::ThisType => todo!(),
    }
}

fn enum_literal_value(db: &dyn InferTySalsaQueryGroup, scope: EntityRoutePtr) -> EnumLiteralValue {
    msg_once!("todo: enum_literal_value");
    EnumLiteralValue::interpreted(scope)
}

fn is_implicit_convertible(
    db: &dyn InferTySalsaQueryGroup,
    src_ty: EntityRoutePtr,
    dst_ty: EntityRoutePtr,
) -> bool {
    if src_ty == dst_ty {
        return true;
    }
    match dst_ty {
        EntityRoutePtr::Root(builtin_ident) => match builtin_ident {
            RootIdentifier::Void => false,
            RootIdentifier::I32 => {
                p!(src_ty, dst_ty);
                todo!()
            }
            RootIdentifier::F32 => todo!(),
            RootIdentifier::B32 => todo!(),
            RootIdentifier::B64 => todo!(),
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
                    | RootIdentifier::Type => false,
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
            RootIdentifier::Type => todo!(),
            RootIdentifier::Datasets => panic!(),
            RootIdentifier::CloneTrait => todo!(),
            RootIdentifier::CopyTrait => todo!(),
            RootIdentifier::PartialEqTrait => todo!(),
            RootIdentifier::EqTrait => todo!(),
        },
        EntityRoutePtr::Custom(_) => todo!(),
        EntityRoutePtr::ThisType => todo!(),
    }
}

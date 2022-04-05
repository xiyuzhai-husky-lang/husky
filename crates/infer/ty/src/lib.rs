mod builder;
mod sheet;

pub use sheet::*;

use ast::*;
use check_utils::*;
use decl::{DeclQueryGroup, TyDecl};
use entity_route::{GenericArgument, *};
use entity_route_query::{ScopeQueryGroup, ScopeResult, ScopeResultArc};
use file::FilePtr;
use fold::FoldStorage;
use infer_error::*;
use print_utils::*;
use syntax_types::*;
use vm::EnumLiteralValue;
use word::{BuiltinIdentifier, ContextualIdentifier};

#[salsa::query_group(InferTyQueryGroupStorage)]
pub trait InferTySalsaQueryGroup: ScopeQueryGroup + ast::AstQueryGroup + DeclQueryGroup {
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
        EntityRoutePtr::Builtin(ident) => match ident {
            BuiltinIdentifier::Void => todo!(),
            BuiltinIdentifier::I32 => todo!(),
            BuiltinIdentifier::F32 => todo!(),
            BuiltinIdentifier::B32 => todo!(),
            BuiltinIdentifier::B64 => todo!(),
            BuiltinIdentifier::Bool => todo!(),
            BuiltinIdentifier::True | BuiltinIdentifier::False => {
                Ok(EntityRoutePtr::Builtin(BuiltinIdentifier::Bool))
            }
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
        EntityRoutePtr::Custom(scope) => match scope.kind {
            ScopeKind::Contextual { main, ident } => match ident {
                ContextualIdentifier::Input => db.global_input_ty(main),
                ContextualIdentifier::ThisData => todo!(),
                ContextualIdentifier::ThisType => todo!(),
            },
            _ => todo!(),
        },
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
        EntityRoutePtr::Builtin(builtin_ident) => match builtin_ident {
            BuiltinIdentifier::Void => false,
            BuiltinIdentifier::I32 => {
                p!(src_ty, dst_ty);
                todo!()
            }
            BuiltinIdentifier::F32 => todo!(),
            BuiltinIdentifier::B32 => todo!(),
            BuiltinIdentifier::B64 => todo!(),
            BuiltinIdentifier::Bool => match src_ty {
                EntityRoutePtr::Builtin(builtin_ident) => match builtin_ident {
                    BuiltinIdentifier::I32
                    | BuiltinIdentifier::F32
                    | BuiltinIdentifier::B32
                    | BuiltinIdentifier::B64
                    | BuiltinIdentifier::Bool => true,
                    BuiltinIdentifier::Void
                    | BuiltinIdentifier::Vec
                    | BuiltinIdentifier::Tuple
                    | BuiltinIdentifier::Fp
                    | BuiltinIdentifier::Fn
                    | BuiltinIdentifier::FnMut
                    | BuiltinIdentifier::FnOnce
                    | BuiltinIdentifier::Array
                    | BuiltinIdentifier::DatasetType
                    | BuiltinIdentifier::Type => false,
                    _ => panic!(),
                },
                EntityRoutePtr::Custom(_) => todo!(),
            },
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
            BuiltinIdentifier::DatasetType => match src_ty {
                EntityRoutePtr::Builtin(BuiltinIdentifier::DatasetType) => true,
                EntityRoutePtr::Custom(scope) => match scope.kind {
                    ScopeKind::Builtin {
                        ident: BuiltinIdentifier::DatasetType,
                    } => true,
                    _ => false,
                },
                _ => false,
            },
            BuiltinIdentifier::Type => todo!(),
            BuiltinIdentifier::Datasets => panic!(),
            BuiltinIdentifier::CloneTrait => todo!(),
            BuiltinIdentifier::CopyTrait => todo!(),
            BuiltinIdentifier::PartialEqTrait => todo!(),
            BuiltinIdentifier::EqTrait => todo!(),
        },
        EntityRoutePtr::Custom(_) => todo!(),
    }
}

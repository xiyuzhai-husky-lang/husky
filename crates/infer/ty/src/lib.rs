mod builder;
mod sheet;

pub use sheet::*;

use ast::*;
use check_utils::*;
use file::FilePtr;
use fold::FoldStorage;
use infer_error::*;
use infer_signature::{InferSignatureQueryGroup, TySignature};
use print_utils::*;
use scope::{GenericArgument, *};
use scope_query::{ScopeQueryGroup, ScopeResult, ScopeResultArc};
use syntax_types::*;
use vm::EnumLiteralValue;
use word::{BuiltinIdentifier, ImplicitIdentifier};

#[salsa::query_group(InferTyQueryGroupStorage)]
pub trait InferTySalsaQueryGroup:
    ScopeQueryGroup + ast::AstQueryGroup + InferSignatureQueryGroup
{
    fn scope_ty(&self, scope: ScopePtr) -> InferResult<ScopePtr>;
    fn enum_literal_value(&self, scope: ScopePtr) -> EnumLiteralValue;
    fn ty_sheet(&self, file: FilePtr) -> ScopeResultArc<TySheet>;

    fn is_implicit_convertible(&self, src_ty: ScopePtr, dst_ty: ScopePtr) -> bool;
}

pub trait InferTyQueryGroup: InferTySalsaQueryGroup {
    fn expr_ty_result(&self, file: FilePtr, expr_idx: RawExprIdx) -> InferResult<ScopePtr> {
        self.ty_sheet(file)?.expr_ty_result(expr_idx)
    }

    fn expr_ty_signature(
        &self,
        file: FilePtr,
        expr_idx: RawExprIdx,
    ) -> InferResultArc<TySignature> {
        let ty = self.expr_ty_result(file, expr_idx)?;
        self.ty_signature(ty)
    }
}

fn scope_ty(db: &dyn InferTySalsaQueryGroup, scope: ScopePtr) -> InferResult<ScopePtr> {
    match scope {
        ScopePtr::Builtin(ident) => match ident {
            BuiltinIdentifier::Void => todo!(),
            BuiltinIdentifier::I32 => todo!(),
            BuiltinIdentifier::F32 => todo!(),
            BuiltinIdentifier::B32 => todo!(),
            BuiltinIdentifier::B64 => todo!(),
            BuiltinIdentifier::Bool => todo!(),
            BuiltinIdentifier::True | BuiltinIdentifier::False => {
                Ok(ScopePtr::Builtin(BuiltinIdentifier::Bool))
            }
            BuiltinIdentifier::Vector => todo!(),
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
        },
        ScopePtr::Custom(scope) => match scope.route {
            ScopeRoute::Implicit { main, ident } => match ident {
                ImplicitIdentifier::Input => db.package_input_ty(main),
            },
            _ => todo!(),
        },
    }
}

fn enum_literal_value(db: &dyn InferTySalsaQueryGroup, scope: ScopePtr) -> EnumLiteralValue {
    msg_once!("todo: enum_literal_value");
    EnumLiteralValue::interpreted(scope)
}

fn is_implicit_convertible(
    db: &dyn InferTySalsaQueryGroup,
    src_ty: ScopePtr,
    dst_ty: ScopePtr,
) -> bool {
    if src_ty == dst_ty {
        return true;
    }
    match dst_ty {
        ScopePtr::Builtin(builtin_ident) => match builtin_ident {
            BuiltinIdentifier::Void => todo!(),
            BuiltinIdentifier::I32 => {
                p!(src_ty, dst_ty);
                todo!()
            }
            BuiltinIdentifier::F32 => todo!(),
            BuiltinIdentifier::B32 => todo!(),
            BuiltinIdentifier::B64 => todo!(),
            BuiltinIdentifier::Bool => match src_ty {
                ScopePtr::Builtin(builtin_ident) => match builtin_ident {
                    BuiltinIdentifier::I32
                    | BuiltinIdentifier::F32
                    | BuiltinIdentifier::B32
                    | BuiltinIdentifier::B64
                    | BuiltinIdentifier::Bool => true,
                    BuiltinIdentifier::Void
                    | BuiltinIdentifier::Vector
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
                ScopePtr::Custom(_) => todo!(),
            },
            BuiltinIdentifier::True => todo!(),
            BuiltinIdentifier::False => todo!(),
            BuiltinIdentifier::Vector => todo!(),
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
                ScopePtr::Builtin(BuiltinIdentifier::DatasetType) => true,
                ScopePtr::Custom(scope) => match scope.route {
                    ScopeRoute::Builtin {
                        ident: BuiltinIdentifier::DatasetType,
                    } => true,
                    _ => false,
                },
                _ => false,
            },
            BuiltinIdentifier::Type => todo!(),
        },
        ScopePtr::Custom(_) => todo!(),
    }
}

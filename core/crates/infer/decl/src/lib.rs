mod feature;
mod function;
mod global;
mod impl_parse;
mod input;
mod member;
mod trai;
mod ty;

pub use function::*;
pub use input::*;
pub use member::*;
use print_utils::p;
pub use trai::*;
pub use ty::*;

use ast::*;
use defn_head::*;
use entity_kind::TyKind;
use entity_route::*;
use entity_syntax::*;
use feature::*;
use file::FilePtr;
use fold::FoldableStorage;
use global::*;
use infer_error::*;
use instantiate::*;
use liason::*;
use static_defn::*;
use std::sync::Arc;
use word::{CustomIdentifier, RootIdentifier};

#[salsa::query_group(DeclQueryGroupStorage)]
pub trait DeclQueryGroup: EntitySyntaxQueryGroup + ast::AstQueryGroup {
    fn function_decl(&self, call_route: EntityRoutePtr) -> InferQueryResultArc<FunctionDecl>;
    fn method_decl(&self, method_route: EntityRoutePtr) -> InferResultArc<MethodDecl>;
    fn ty_decl(&self, ty: EntityRoutePtr) -> InferQueryResultArc<TyDecl>;
    fn trait_decl(&self, trai: EntityRoutePtr) -> InferResultArc<TraitDecl>;
    fn feature_decl(&self, feature_entity: EntityRoutePtr) -> InferResultArc<FeatureDecl>;
    fn eval_input_ty(&self, main_file: FilePtr) -> InferResult<EntityRoutePtr>;
    fn global_output_ty(&self, main_file: FilePtr) -> InferResult<EntityRoutePtr>;
    // fn vec_decl(&self) -> Arc<TyDecl>;
    // fn trait_decl_menu(&self) -> Arc<TraitDeclMenu>;
    fn member_idx(&self, member_route: EntityRoutePtr) -> MemberIdx;

    fn is_copyable(&self, ty: EntityRoutePtr) -> InferResult<bool>;
    fn is_clonable(&self, ty: EntityRoutePtr) -> InferResult<bool>;
}

pub(crate) fn is_copyable(db: &dyn DeclQueryGroup, ty: EntityRoutePtr) -> InferResult<bool> {
    match ty.kind {
        EntityRouteKind::Root { ident } => Ok(match ident {
            RootIdentifier::Void
            | RootIdentifier::I32
            | RootIdentifier::F32
            | RootIdentifier::B32
            | RootIdentifier::B64
            | RootIdentifier::Bool
            | RootIdentifier::Ref => true,
            RootIdentifier::Vec => false,
            RootIdentifier::Tuple => false,
            RootIdentifier::Array => false,
            RootIdentifier::DatasetType => false,
            RootIdentifier::TypeType => false,
            RootIdentifier::TraitType => false,
            RootIdentifier::ModuleType => false,
            _ => {
                p!(ident);
                panic!()
            }
        }),
        _ => {
            let ty_decl = db.ty_decl(ty)?;
            Ok(ty_decl
                .trait_impl(db.entity_route_menu().copy_trait)
                .is_some())
        }
    }
}

pub(crate) fn is_clonable(db: &dyn DeclQueryGroup, ty: EntityRoutePtr) -> InferResult<bool> {
    let ty_decl = db.ty_decl(ty)?;
    Ok(ty_decl
        .trait_impl(db.entity_route_menu().clone_trait)
        .is_some())
}

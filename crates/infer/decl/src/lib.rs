mod call;
mod copy;
mod feature;
mod global;
mod impl_parse;
mod input;
mod member;
mod trai;
mod ty;

pub use call::*;
pub use input::*;
pub use member::*;
pub use trai::*;
pub use ty::*;

use ast::*;
use copy::*;
use defn_head::*;
use entity_kind::TyKind;
use entity_route::*;
use entity_route_query::*;
use feature::*;
use file::FilePtr;
use fold::FoldStorage;
use global::*;
use infer_error::*;
use instantiate::*;
use static_defn::*;
use std::sync::Arc;
use word::CustomIdentifier;

#[salsa::query_group(DeclQueryGroupStorage)]
pub trait DeclQueryGroup: EntityRouteQueryGroup + ast::AstQueryGroup {
    fn call_decl(&self, scope: EntityRoutePtr) -> InferQueryResultArc<CallDecl>;
    fn method_decl(&self, scope: EntityRoutePtr) -> InferResultArc<MethodDecl>;
    fn ty_decl(&self, ty: EntityRoutePtr) -> InferQueryResultArc<TyDecl>;
    fn trait_decl(&self, trai: EntityRoutePtr) -> InferResultArc<TraitDecl>;
    fn feature_decl(&self, feature_entity: EntityRoutePtr) -> InferResultArc<FeatureDecl>;
    fn global_input_ty(&self, main_file: FilePtr) -> InferResult<EntityRoutePtr>;
    fn global_output_ty(&self, main_file: FilePtr) -> InferResult<EntityRoutePtr>;
    fn vec_decl(&self) -> Arc<TyDecl>;
    fn trait_decl_menu(&self) -> Arc<TraitDeclMenu>;
    fn member_idx(&self, member_route: EntityRoutePtr) -> MemberIdx;

    fn is_copyable(&self, ty: EntityRoutePtr) -> bool;
}

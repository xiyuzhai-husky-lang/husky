mod call;
mod feature;
mod global;
mod impl_parse;
mod input;
mod member;
mod trai;
mod ty;

pub use call::*;
pub use call::*;
pub use input::*;
pub use member::*;
pub use trai::*;
pub use ty::*;

use ast::*;
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
use static_decl::*;
use std::sync::Arc;
use word::CustomIdentifier;

#[salsa::query_group(DeclQueryGroupStorage)]
pub trait DeclQueryGroup: EntityRouteQueryGroup + ast::AstQueryGroup {
    fn call_decl(&self, scope: EntityRoutePtr) -> InferResultArc<CallDecl>;
    fn type_decl(&self, scope: EntityRoutePtr) -> InferResultArc<TypeDecl>;
    fn trait_decl(&self, scope: EntityRoutePtr) -> InferResultArc<TraitDecl>;
    fn feature_decl(&self, scope: EntityRoutePtr) -> InferResultArc<FeatureSignature>;
    fn global_input_ty(&self, main_file: FilePtr) -> InferResult<EntityRoutePtr>;
    fn global_output_ty(&self, main_file: FilePtr) -> InferResult<EntityRoutePtr>;
    fn vec_decl(&self) -> Arc<TypeDecl>;
    fn trait_decl_menu(&self) -> Arc<TraitDeclMenu>;
    fn member_idx(&self, member_route: EntityRoutePtr) -> MemberIdx;
}

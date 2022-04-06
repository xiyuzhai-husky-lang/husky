use std::sync::Arc;

use crate::dependence::*;
use crate::*;
use entity_route::EntityRoutePtr;
use infer_total::InferQueryGroup;
use semantics_error::*;
use upcast::Upcast;
use vec_map::VecMap;

#[salsa::query_group(EntityQueryGroupStorage)]
pub trait EntityQueryGroup:
    InferQueryGroup + ast::AstQueryGroup + Upcast<dyn InferQueryGroup>
{
    fn main_defn(&self, main_file: file::FilePtr) -> SemanticResultArc<MainDefn>;
    fn entity_defn(&self, scope: EntityRoutePtr) -> SemanticResultArc<EntityDefn>;
    fn entity_immediate_dependees(&self, scope: EntityRoutePtr) -> SemanticResultArc<DependeeMap>;
    fn entity_dependees(&self, scope: EntityRoutePtr) -> SemanticResultArc<DependeeMap>;
    fn subentity_defns(&self, scope: EntityRoutePtr) -> SemanticResultArc<Vec<Arc<EntityDefn>>>;
    fn entity_defn_uid(&self, scope: EntityRoutePtr) -> EntityDefnUid;
    fn entity_uid(&self, scope: EntityRoutePtr) -> EntityUid;
}

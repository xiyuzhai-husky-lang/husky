use husky_infer_entity_route::*;
use husky_infer_qualified_ty::InferQualifiedTyQueryGroup;
use infer_contract::*;

pub trait InferQueryGroup:
    InferContractQueryGroup + InferEntityRouteQueryGroup + InferQualifiedTyQueryGroup
{
}

use crate::{builder::QualifiedTySheetBuilder, *};
use entity_route_query::EntityRouteResultArc;
use file::FilePtr;
use infer_contract::InferContractQueryGroup;
use infer_entity_route::InferEntityRouteQueryGroup;
use print_utils::p;

#[salsa::query_group(InferQualifiedTyQueryGroupStorage)]
pub trait InferQualifiedTyQueryGroup: InferEntityRouteQueryGroup + InferContractQueryGroup {
    fn qualified_ty_sheet(&self, file: FilePtr) -> EntityRouteResultArc<QualifiedTySheet>;
}

fn qualified_ty_sheet(
    db: &dyn InferQualifiedTyQueryGroup,
    file: FilePtr,
) -> EntityRouteResultArc<QualifiedTySheet> {
    let mut builder = QualifiedTySheetBuilder::new(db, file)?;
    builder.infer_all();
    Ok(builder.finish())
}

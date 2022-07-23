use crate::{builder::QualifiedTySheetBuilder, *};
use husky_entity_syntax::EntitySyntaxResultArc;
use husky_file::FilePtr;
use husky_infer_entity_route::InferEntityRouteQueryGroup;
use husky_print_utils::p;
use infer_contract::InferContractQueryGroup;

#[salsa::query_group(InferQualifiedTyQueryGroupStorage)]
pub trait InferQualifiedTyQueryGroup: InferEntityRouteQueryGroup + InferContractQueryGroup {
    fn qualified_ty_sheet(&self, file: FilePtr) -> EntitySyntaxResultArc<QualifiedTySheet>;
}

fn qualified_ty_sheet(
    db: &dyn InferQualifiedTyQueryGroup,
    file: FilePtr,
) -> EntitySyntaxResultArc<QualifiedTySheet> {
    let ast_text = db.ast_text(file)?;
    let mut builder = QualifiedTySheetBuilder::new(db, &ast_text.arena, file)?;
    builder.infer_all();
    Ok(builder.finish())
}

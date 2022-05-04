use crate::{builder::QualifiedTySheetBuilder, *};
use entity_route_query::EntityRouteResultArc;
use file::FilePtr;
use infer_contract::InferContractQueryGroup;
use infer_entity_route::InferEntityRouteQueryGroup;
use print_utils::p;

#[salsa::query_group(InferQualifiedTyQueryGroupStorage)]
pub trait InferQualifiedTyQueryGroup: InferEntityRouteQueryGroup + InferContractQueryGroup {
    fn qualified_ty_sheet(&self, file: FilePtr) -> EntityRouteResultArc<QualifiedTySheet>;
    fn is_qualified_ty_implicitly_convertible_to_contracted_ty(
        &self,
        qualified_ty: QualifiedTy,
        output_contract: OutputContract,
        output_ty: EntityRoutePtr,
    ) -> bool;
}

fn qualified_ty_sheet(
    db: &dyn InferQualifiedTyQueryGroup,
    file: FilePtr,
) -> EntityRouteResultArc<QualifiedTySheet> {
    let mut builder = QualifiedTySheetBuilder::new(db, file)?;
    builder.infer_all();
    Ok(builder.finish())
}

fn is_qualified_ty_implicitly_convertible_to_contracted_ty(
    db: &dyn InferQualifiedTyQueryGroup,
    qualified_ty: QualifiedTy,
    output_contract: OutputContract,
    output_ty: EntityRoutePtr,
) -> bool {
    if !db.is_implicit_convertible(qualified_ty.ty, output_ty) {
        return false;
    }
    match output_contract {
        OutputContract::Transitive => match qualified_ty.qual {
            Qualifier::PureRef | Qualifier::LocalRef => false,
            Qualifier::Transient
            | Qualifier::Copyable
            | Qualifier::CopyableMut
            | Qualifier::StackOwned
            | Qualifier::StackOwnedMut => true,
        },
        OutputContract::MemberAccess => todo!(),
    }
}

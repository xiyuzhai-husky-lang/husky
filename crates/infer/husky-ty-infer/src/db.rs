use husky_entity_path::EntityPathPtr;
use husky_term::Ty;

#[salsa::query_group(TyInferDbStorage)]
pub trait TyInferDb: TyInferQueries {
    fn entity_ty(&self, entity: EntityPathPtr) -> Ty;
}

fn entity_ty(db: &dyn TyInferDb, entity: EntityPathPtr) -> Ty {
    db.infer_entity_ty(entity)
}

pub trait TyInferQueries {
    fn infer_entity_ty(&self, entity: EntityPathPtr) -> Ty;
}

pub trait TyInferQueryImpls {}

impl<T> TyInferQueries for T
where
    T: TyInferQueryImpls,
{
    fn infer_entity_ty(&self, entity: EntityPathPtr) -> Ty {
        todo!()
    }
}

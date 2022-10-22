use crate::*;
use husky_entity_path::EntityPathPtr;
use husky_term::Ty;
use husky_word::InternWord;
use std::sync::Arc;

#[salsa::query_group(TyInferDbStorage)]
pub trait TyInferDb: TyInferQueries + InternWord {
    fn entity_ty(&self, entity: EntityPathPtr) -> Ty;
    fn decl(&self, entity: EntityPathPtr) -> Arc<Decl>;
}

fn entity_ty(db: &dyn TyInferDb, entity: EntityPathPtr) -> Ty {
    db.infer_entity_ty(entity)
}

fn decl(db: &dyn TyInferDb, entity: EntityPathPtr) -> Arc<Decl> {
    todo!()
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

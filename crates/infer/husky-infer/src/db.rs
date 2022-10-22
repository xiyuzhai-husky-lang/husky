use crate::*;
use husky_entity_path::EntityPathItd;
use husky_term::Ty;
use husky_word::InternWord;
use std::sync::Arc;

#[salsa::query_group(InferDbStorage)]
pub trait TyInferDb: TyInferQueries + InternWord {
    fn entity_ty(&self, entity: EntityPathItd) -> Ty;
    fn decl(&self, entity: EntityPathItd) -> Arc<Decl>;
}

fn entity_ty(db: &dyn TyInferDb, entity: EntityPathItd) -> Ty {
    db.infer_entity_ty(entity)
}

fn decl(db: &dyn TyInferDb, entity: EntityPathItd) -> Arc<Decl> {
    todo!()
}

pub trait TyInferQueries {
    fn infer_entity_ty(&self, entity: EntityPathItd) -> Ty;
}

pub trait TyInferQueryImpls {}

impl<T> TyInferQueries for T
where
    T: TyInferQueryImpls,
{
    fn infer_entity_ty(&self, entity: EntityPathItd) -> Ty {
        todo!()
    }
}

use crate::*;
use husky_entity_path::EntityPathItd;
use husky_term::{TermDb, Ty};
use husky_word::InternWord;
use std::sync::Arc;
use upcast::Upcast;

#[salsa::query_group(InferDbStorage)]
pub trait InferDb: TyInferQueries + TermDb + Upcast<dyn TermDb> + InternWord {
    fn entity_ty(&self, entity: EntityPathItd) -> Ty;
    fn decl(&self, entity: EntityPathItd) -> Arc<Decl>;
}

fn entity_ty(db: &dyn InferDb, entity: EntityPathItd) -> Ty {
    db.infer_entity_ty(entity)
}

fn decl(db: &dyn InferDb, entity: EntityPathItd) -> Arc<Decl> {
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

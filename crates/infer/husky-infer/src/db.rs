use crate::*;
use husky_entity_path::EntityPathItd;
use husky_term::{Decl, TermDb, Ty};
use husky_word::InternWord;
use std::sync::Arc;
use upcast::Upcast;

#[salsa::query_group(InferDbStorage)]
pub trait InferDb: TyInferQueries + TermDb + Upcast<dyn TermDb> + InternWord {
    fn entity_ty(&self, entity: EntityPathItd) -> Ty;
}

fn entity_ty(db: &dyn InferDb, entity: EntityPathItd) -> Ty {
    db.infer_entity_ty(entity)
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

use crate::*;
use husky_entity_path::EntityPathItd;
use husky_file::{FileItd, FileResultArc};
use husky_term::{Decl, TermDb, Ty};
use husky_word::InternWord;
use std::sync::Arc;
use upcast::Upcast;

#[salsa::query_group(TermInferDbStorage)]
pub trait TermInferDb: TyInferQueries + TermDb + Upcast<dyn TermDb> + InternWord {
    fn entity_ty(&self, entity: EntityPathItd) -> Ty;

    fn term_sheet(&self, file: FileItd) -> FileResultArc<TermSheet>;
}

fn entity_ty(db: &dyn TermInferDb, entity: EntityPathItd) -> Ty {
    db.infer_entity_ty(entity)
}

fn term_sheet(db: &dyn TermInferDb, file: FileItd) -> FileResultArc<TermSheet> {
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

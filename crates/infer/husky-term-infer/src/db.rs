use crate::*;
use husky_entity_path::EntityPath;
use husky_path::FileResultArc;
use husky_term::{Decl, Term, TermDb};
use husky_word::WordDb;
use salsa::DbWithJar;
use std::sync::Arc;
use upcast::Upcast;

pub trait TermInferDb:
    DbWithJar<TermInferJar> + TyInferQueries + TermDb + Upcast<dyn TermDb> + WordDb
{
    fn entity_ty(&self, entity: EntityPath) -> Term;

    fn term_sheet(&self, file: AbsolutePath) -> FileResultArc<TermSheet>;
}

impl<T> TermInferDb for T
where
    T: DbWithJar<TermInferJar> + TyInferQueries + TermDb + Upcast<dyn TermDb> + WordDb,
{
    fn entity_ty(&self, entity: EntityPath) -> Term {
        todo!()
    }

    fn term_sheet(&self, file: AbsolutePath) -> FileResultArc<TermSheet> {
        todo!()
    }
}

fn entity_ty(db: &dyn TermInferDb, entity: EntityPath) -> Term {
    db.infer_entity_ty(entity)
}

fn term_sheet(db: &dyn TermInferDb, file: AbsolutePath) -> FileResultArc<TermSheet> {
    todo!()
}

pub trait TyInferQueries {
    fn infer_entity_ty(&self, entity: EntityPath) -> Term;
}

pub trait TyInferQueryImpls {}

impl<T> TyInferQueries for T
where
    T: TyInferQueryImpls,
{
    fn infer_entity_ty(&self, entity: EntityPath) -> Term {
        todo!()
    }
}

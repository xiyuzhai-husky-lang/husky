use crate::*;

pub(crate) struct TyInferContext<'a> {
    db: &'a dyn TyInferDb,
    sheet: &'a mut TyInferSheet,
}

impl<'a> TyInferContext<'a> {
    fn infer_entity_ty(&self, entity: husky_entity_path::EntityPathPtr) -> husky_term::Ty {
        self.db.entity_ty(entity)
    }
}

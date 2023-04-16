use crate::*;

pub trait HasDeclarativeType: Copy {
    fn raw_ty(self, db: &dyn DeclarativeTypeDb) -> DeclarativeTypeResult<DeclarativeTerm>;
}

impl HasDeclarativeType for TypeItemPath {
    fn raw_ty(self, db: &dyn DeclarativeTypeDb) -> DeclarativeTypeResult<DeclarativeTerm> {
        ty_item_path_raw_ty(db, self)
    }
}

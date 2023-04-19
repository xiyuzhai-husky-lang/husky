use crate::*;

pub trait HasDeclarativeType: Copy {
    fn declarative_ty(self, db: &dyn DeclarativeTypeDb) -> DeclarativeTypeResult<DeclarativeTerm>;
}

impl HasDeclarativeType for TypeItemPath {
    fn declarative_ty(self, db: &dyn DeclarativeTypeDb) -> DeclarativeTypeResult<DeclarativeTerm> {
        ty_item_path_declarative_ty(db, self)
    }
}

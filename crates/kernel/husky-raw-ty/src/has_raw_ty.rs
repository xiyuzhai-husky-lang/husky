use crate::*;

pub trait HasRawType: Copy {
    fn raw_ty(self, db: &dyn RawTypeDb) -> RawTypeResult<RawTerm>;
}

impl HasRawType for TypeItemPath {
    fn raw_ty(self, db: &dyn RawTypeDb) -> RawTypeResult<RawTerm> {
        ty_item_path_raw_ty(db, self)
    }
}

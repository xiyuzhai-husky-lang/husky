use crate::*;
use either::*;
use path::major_item::ty::PreludeTypePath;

impl HirType {
    pub fn deleash_if_leashed(self, db: &::salsa::Db) -> HirType {
        let HirType::PathLeading(slf) = self else {
            unreachable!()
        };
        if slf.ty_path(db).refine(db) != Left(PreludeTypePath::LEASH) {
            return slf.into();
        }
        let HirTemplateArgument::Type(slf_derefed) = slf.template_arguments(db)[0] else {
            unreachable!()
        };
        slf_derefed
    }

    pub fn is_always_leashed(self, db: &::salsa::Db) -> bool {
        let HirType::PathLeading(slf) = self else {
            return false;
        };
        slf.ty_path(db).refine(db) == Left(PreludeTypePath::LEASH)
    }
}

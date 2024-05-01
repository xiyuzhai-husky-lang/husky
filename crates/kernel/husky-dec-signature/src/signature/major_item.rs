pub mod form;
pub mod trai;
pub mod ty;

use self::form::*;
use self::trai::*;
use self::ty::*;
use super::*;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MajorItemDecTemplate {
    Type(TypeDecTemplate),
    Form(MajorFormDecTemplate),
    Trait(TraitDecTemplate),
}

impl HasDecTemplate for MajorItemPath {
    type DecTemplate = MajorItemDecTemplate;

    #[inline(always)]
    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        match self {
            MajorItemPath::Type(path) => path.dec_template(db).map(Into::into),
            MajorItemPath::Form(path) => path.dec_template(db).map(Into::into),
            MajorItemPath::Trait(decl) => decl.dec_template(db).map(Into::into),
        }
    }
}

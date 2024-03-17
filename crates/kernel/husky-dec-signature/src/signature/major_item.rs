mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MajorItemDecTemplate {
    Type(TypeDecTemplate),
    Fugitive(FugitiveDecTemplate),
    Trait(TraitDecTemplate),
}

impl HasDecTemplate for MajorItemPath {
    type DecTemplate = MajorItemDecTemplate;

    #[inline(always)]
    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        match self {
            MajorItemPath::Type(path) => path.dec_template(db).map(Into::into),
            MajorItemPath::Fugitive(path) => path.dec_template(db).map(Into::into),
            MajorItemPath::Trait(decl) => decl.dec_template(db).map(Into::into),
        }
    }
}

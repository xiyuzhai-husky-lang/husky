mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum MajorItemDecTemplate {
    Type(TypeDecTemplate),
    Fugitive(FugitiveDecTemplate),
    Trait(TraitDecTemplate),
}

impl HasDecTemplate for MajorItemPath {
    type DecTemplate = MajorItemDecTemplate;

    #[inline(always)]
    fn declarative_signature_template(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self::DecTemplate> {
        match self {
            MajorItemPath::Type(path) => path.declarative_signature_template(db).map(Into::into),
            MajorItemPath::Fugitive(path) => {
                path.declarative_signature_template(db).map(Into::into)
            }
            MajorItemPath::Trait(decl) => decl.declarative_signature_template(db).map(Into::into),
        }
    }
}

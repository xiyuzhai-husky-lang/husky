mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum MajorItemDeclarativeSignatureTemplate {
    Type(TypeDeclarativeSignatureTemplate),
    Fugitive(FugitiveDeclarativeSignatureTemplate),
    Trait(TraitDeclarativeSignatureTemplate),
}

impl HasDeclarativeSignatureTemplate for MajarItemPath {
    type DeclarativeSignatureTemplate = MajorItemDeclarativeSignatureTemplate;

    #[inline(always)]
    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        match self {
            MajarItemPath::Type(path) => path.declarative_signature_template(db).map(Into::into),
            MajarItemPath::Fugitive(path) => {
                path.declarative_signature_template(db).map(Into::into)
            }
            MajarItemPath::Trait(decl) => decl.declarative_signature_template(db).map(Into::into),
        }
    }
}

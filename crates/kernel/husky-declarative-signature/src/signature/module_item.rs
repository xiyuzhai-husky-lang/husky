mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum ModuleItemDeclarativeSignatureTemplate {
    Type(TypeDeclarativeSignatureTemplate),
    Fugitive(FugitiveDeclarativeSignatureTemplate),
    Trait(TraitDeclarativeSignatureTemplate),
}

impl HasDeclarativeSignatureTemplate for ModuleItemPath {
    type DeclarativeSignatureTemplate = ModuleItemDeclarativeSignatureTemplate;

    #[inline(always)]
    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        match self {
            ModuleItemPath::Type(decl) => decl.declarative_signature_template(db).map(Into::into),
            ModuleItemPath::Fugitive(decl) => {
                decl.declarative_signature_template(db).map(Into::into)
            }
            ModuleItemPath::Trait(decl) => decl.declarative_signature_template(db).map(Into::into),
        }
    }
}

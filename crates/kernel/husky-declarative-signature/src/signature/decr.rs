mod derive_decr;

pub use self::derive_decr::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum DecrDeclarativeSignatureTemplate {
    Derive(DeriveDecrDeclarativeSignatureTemplate),
}

impl HasDeclarativeSignatureTemplate for DecrPath {
    type DeclarativeSignatureTemplate = DecrDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        decr_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
fn decr_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    path: DecrPath,
) -> DeclarativeSignatureResult<DecrDeclarativeSignatureTemplate> {
    match path.syn_decl(db)? {
        DecrSynDecl::Derive(decl) => {
            DeriveDecrDeclarativeSignatureTemplate::from_decl(decl, db).map(Into::into)
        }
    }
}

mod derive;

pub use self::derive::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum AttrDeclarativeSignatureTemplate {
    Derive(DeriveAttrDeclarativeSignatureTemplate),
}

impl HasDeclarativeSignatureTemplate for AttrItemPath {
    type DeclarativeSignatureTemplate = AttrDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        attr_declarative_signature_template(db, self)
    }
}

// #[salsa::tracked(jar = DeclarativeSignatureJar)]
fn attr_declarative_signature_template(
    db: &::salsa::Db,
    path: AttrItemPath,
) -> DeclarativeSignatureResult<AttrDeclarativeSignatureTemplate> {
    match path.syn_decl(db)? {
        AttrSynDecl::Derive(decl) => {
            DeriveAttrDeclarativeSignatureTemplate::from_decl(decl, db).map(Into::into)
        }
    }
}

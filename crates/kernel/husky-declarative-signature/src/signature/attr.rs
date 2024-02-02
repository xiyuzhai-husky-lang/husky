mod derive;

pub use self::derive::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum AttrDecTemplate {
    Derive(DeriveAttrDecTemplate),
}

impl HasDecTemplate for AttrItemPath {
    type DecTemplate = AttrDecTemplate;

    fn declarative_signature_template(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self::DecTemplate> {
        attr_declarative_signature_template(db, self)
    }
}

// #[salsa::tracked(jar = DeclarativeSignatureJar)]
fn attr_declarative_signature_template(
    db: &::salsa::Db,
    path: AttrItemPath,
) -> DeclarativeSignatureResult<AttrDecTemplate> {
    match path.syn_decl(db)? {
        AttrSynDecl::Derive(decl) => DeriveAttrDecTemplate::from_decl(decl, db).map(Into::into),
    }
}

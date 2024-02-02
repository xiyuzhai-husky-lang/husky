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

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        attr_dec_template(db, self)
    }
}

// #[salsa::tracked(jar = DecSignatureJar)]
fn attr_dec_template(db: &::salsa::Db, path: AttrItemPath) -> DecSignatureResult<AttrDecTemplate> {
    match path.syn_decl(db)? {
        AttrSynDecl::Derive(decl) => DeriveAttrDecTemplate::from_decl(decl, db).map(Into::into),
    }
}

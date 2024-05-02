pub mod derive;

use self::derive::*;
use super::*;
use husky_entity_path::path::attr::AttrItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
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

#[salsa::tracked]
fn attr_dec_template(db: &::salsa::Db, path: AttrItemPath) -> DecSignatureResult<AttrDecTemplate> {
    match path.syn_decl(db)? {
        AttrSynDecl::Backprop(_) => todo!(),
        AttrSynDecl::Derive(decl) => DeriveAttrDecTemplate::from_decl(decl, db).map(Into::into),
        AttrSynDecl::Effect(decl) => todo!(),
        AttrSynDecl::Test(decl) => todo!(),
    }
}

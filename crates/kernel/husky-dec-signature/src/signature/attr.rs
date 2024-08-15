pub mod backprop;
pub mod dep;
pub mod derive;
pub mod proj;
pub mod singleton;
pub mod task;

use self::backprop::*;
use self::dep::*;
use self::derive::*;
use self::proj::*;
use self::singleton::*;
use self::task::*;
use super::*;
use husky_entity_path::path::attr::AttrItemPath;
use husky_syn_decl::decl::attr::AttrSynDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum AttrDecTemplate {
    Backprop(BackpropAttrDecTemplate),
    Dep(DepAttrDecTemplate),
    Derive(DeriveAttrDecTemplate),
    Proj(ProjAttrDecTemplate),
    Singleton(SingletonAttrDecTemplate),
    Task(TaskAttrDecTemplate),
    Test,
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
        AttrSynDecl::Affect(decl) => todo!(),
        AttrSynDecl::Backprop(decl) => {
            BackpropAttrDecTemplate::from_decl(path, decl, db).map(Into::into)
        }
        AttrSynDecl::Dep(decl) => DepAttrDecTemplate::from_decl(path, decl, db).map(Into::into),
        AttrSynDecl::Derive(decl) => DeriveAttrDecTemplate::from_decl(decl, db).map(Into::into),
        AttrSynDecl::Proj(decl) => ProjAttrDecTemplate::from_decl(path, decl, db).map(Into::into),
        AttrSynDecl::Singleton(decl) => {
            SingletonAttrDecTemplate::from_decl(decl, db).map(Into::into)
        }
        AttrSynDecl::Task(decl) => TaskAttrDecTemplate::from_decl(decl, db).map(Into::into),
        AttrSynDecl::Test(decl) => Ok(AttrDecTemplate::Test),
    }
}

pub mod function_ritchie;
pub mod ty_alias;
pub mod val;

use self::function_ritchie::*;
use self::ty_alias::*;
use self::val::*;
use super::*;
use husky_dec_signature::signature::major_item::form::MajorFormDecTemplate;
use husky_entity_path::path::major_item::form::MajorFormPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
pub enum FormEthTemplate {
    Ritchie(MajorFunctionRitchieEthTemplate),
    TypeAlias(MajorTypeAliasEthTemplate),
    Val(MajorValEthTemplate),
}

impl FormEthTemplate {
    pub fn path(self, db: &::salsa::Db) -> MajorFormPath {
        match self {
            FormEthTemplate::Ritchie(slf) => slf.path(db),
            FormEthTemplate::TypeAlias(slf) => slf.path(db),
            FormEthTemplate::Val(slf) => slf.path(db),
        }
    }

    pub fn template_parameters(self, db: &::salsa::Db) -> &[EthTemplateParameter] {
        match self {
            FormEthTemplate::Ritchie(slf) => slf.template_parameters(db),
            FormEthTemplate::TypeAlias(slf) => slf.template_parameters(db),
            FormEthTemplate::Val(_) => &[],
        }
    }
}

impl HasEthTemplate for MajorFormPath {
    type EthTemplate = FormEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        form_eth_template(db, self)
    }
}

#[salsa::tracked]
fn form_eth_template(db: &::salsa::Db, path: MajorFormPath) -> EthSignatureResult<FormEthTemplate> {
    Ok(match path.dec_template(db)? {
        MajorFormDecTemplate::Ritchie(dec_template) => {
            MajorFunctionRitchieEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        MajorFormDecTemplate::TypeAlias(dec_template) => {
            MajorTypeAliasEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        MajorFormDecTemplate::Val(dec_template) => {
            MajorValEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        MajorFormDecTemplate::Compterm(_) => todo!(),
        MajorFormDecTemplate::Static(_) => todo!(),
    })
}

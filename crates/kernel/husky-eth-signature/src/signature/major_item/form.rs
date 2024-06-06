pub mod compterm;
pub mod function_ritchie;
pub mod static_mut;
pub mod static_var;
pub mod ty_alias;
pub mod ty_var;
pub mod val;

use self::{
    compterm::*, function_ritchie::*, static_mut::*, static_var::*, ty_alias::*, ty_var::*, val::*,
};
use super::*;
use husky_dec_signature::signature::major_item::form::MajorFormDecTemplate;
use husky_entity_path::path::major_item::form::MajorFormPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
pub enum FormEthTemplate {
    Ritchie(MajorFunctionRitchieEthTemplate),
    TypeAlias(MajorTypeAliasEthTemplate),
    TypeVar(MajorTypeVarEthTemplate),
    Val(MajorValEthTemplate),
    StaticMut(MajorStaticMutEthTemplate),
    StaticVar(MajorStaticVarEthTemplate),
    Compterm(MajorComptermEthTemplate),
}

impl FormEthTemplate {
    pub fn path(self, db: &::salsa::Db) -> MajorFormPath {
        match self {
            FormEthTemplate::Ritchie(slf) => slf.path(db),
            FormEthTemplate::TypeAlias(slf) => slf.path(db),
            FormEthTemplate::TypeVar(slf) => slf.path(db),
            FormEthTemplate::Val(slf) => slf.path(db),
            FormEthTemplate::StaticMut(slf) => slf.path(db),
            FormEthTemplate::StaticVar(slf) => slf.path(db),
            FormEthTemplate::Compterm(slf) => slf.path(db),
        }
    }

    pub fn template_parameters(self, db: &::salsa::Db) -> &[EthTemplateParameter] {
        match self {
            FormEthTemplate::Ritchie(slf) => slf.template_parameters(db),
            FormEthTemplate::TypeAlias(slf) => slf.template_parameters(db),
            FormEthTemplate::TypeVar(slf) => slf.template_parameters(db),
            FormEthTemplate::Val(_) => &[],
            FormEthTemplate::StaticMut(_) => &[],
            FormEthTemplate::StaticVar(_) => &[],
            // maybe not empty in the future
            FormEthTemplate::Compterm(_) => &[],
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
        MajorFormDecTemplate::TypeVar(dec_template) => {
            MajorTypeVarEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        MajorFormDecTemplate::Val(dec_template) => {
            MajorValEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        MajorFormDecTemplate::Compterm(dec_template) => {
            MajorComptermEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        MajorFormDecTemplate::StaticMut(dec_template) => {
            MajorStaticMutEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        MajorFormDecTemplate::StaticVar(dec_template) => {
            MajorStaticVarEthTemplate::from_dec(db, path, dec_template)?.into()
        }
    })
}

mod function_ritchie;
mod ty_alias;
mod val;

pub use self::function_ritchie::*;
pub use self::ty_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
pub enum FormEthTemplate {
    Ritchie(MajorFunctionRitchieEthTemplate),
    TypeAlias(MajorTypeAliasEthTemplate),
    Val(MajorValEthTemplate),
}

impl FormEthTemplate {
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

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        form_eth_template(db, self)
    }
}

#[salsa::tracked(jar = EthSignatureJar)]
fn form_eth_template(
    db: &::salsa::Db,
    path: MajorFormPath,
) -> EtherealSignatureResult<FormEthTemplate> {
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
        MajorFormDecTemplate::Const(_) => todo!(),
    })
}

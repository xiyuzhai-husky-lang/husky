mod r#fn;
mod gn;
mod ty_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ty_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
#[salsa::debug_with_db]
pub enum FugitiveEthTemplate {
    FunctionFn(FunctionFnEthTemplate),
    FunctionGn(GnFugitiveEthTemplate),
    TypeAlias(TypeAliasEthTemplate),
    Val(ValFugitiveEthTemplate),
}

impl FugitiveEthTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[EthTemplateParameter] {
        match self {
            FugitiveEthTemplate::FunctionFn(template) => template.template_parameters(db),
            FugitiveEthTemplate::FunctionGn(template) => template.template_parameters(db),
            FugitiveEthTemplate::TypeAlias(template) => template.template_parameters(db),
            FugitiveEthTemplate::Val(_template) => &[],
        }
    }
}

impl HasEthTemplate for FugitivePath {
    type EthTemplate = FugitiveEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        fugitive_eth_template(db, self)
    }
}

// #[salsa::tracked(jar = EtherealSignatureJar)]
fn fugitive_eth_template(
    db: &::salsa::Db,
    path: FugitivePath,
) -> EtherealSignatureResult<FugitiveEthTemplate> {
    Ok(match path.dec_template(db)? {
        FugitiveDecTemplate::Fn(dec_template) => {
            FunctionFnEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        FugitiveDecTemplate::Gn(dec_template) => {
            GnFugitiveEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        FugitiveDecTemplate::TypeAlias(dec_template) => {
            TypeAliasEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        FugitiveDecTemplate::Val(dec_template) => {
            ValFugitiveEthTemplate::from_dec(db, path, dec_template)?.into()
        }
    })
}

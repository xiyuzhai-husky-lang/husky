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
    pub fn template_parameters(self, db: &::salsa::Db) -> &[EtherealTemplateParameter] {
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

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EthTemplate> {
        fugitive_ethereal_signature_template(db, self)
    }
}

// #[salsa::tracked(jar = EtherealSignatureJar)]
fn fugitive_ethereal_signature_template(
    db: &::salsa::Db,
    path: FugitivePath,
) -> EtherealSignatureResult<FugitiveEthTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        FugitiveDecTemplate::Fn(declarative_signature_template) => {
            FunctionFnEthTemplate::from_declarative(db, path, declarative_signature_template)?
                .into()
        }
        FugitiveDecTemplate::Gn(declarative_signature_template) => {
            GnFugitiveEthTemplate::from_declarative(db, path, declarative_signature_template)?
                .into()
        }
        FugitiveDecTemplate::TypeAlias(declarative_signature_template) => {
            TypeAliasEthTemplate::from_declarative(db, path, declarative_signature_template)?.into()
        }
        FugitiveDecTemplate::Val(declarative_signature_template) => {
            ValFugitiveEthTemplate::from_declarative(db, path, declarative_signature_template)?
                .into()
        }
    })
}

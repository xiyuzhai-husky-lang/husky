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
#[salsa::debug_with_db(db = EtherealSignatureDb)]
pub enum FugitiveEtherealSignatureTemplate {
    FunctionFn(FunctionFnEtherealSignatureTemplate),
    FunctionGn(GnFugitiveEtherealSignatureTemplate),
    TypeAlias(TypeAliasEtherealSignatureTemplate),
    Val(ValFugitiveEtherealSignatureTemplate),
}

impl FugitiveEtherealSignatureTemplate {
    pub fn template_parameters(self, db: &dyn EtherealSignatureDb) -> &[EtherealTemplateParameter] {
        match self {
            FugitiveEtherealSignatureTemplate::FunctionFn(template) => {
                template.template_parameters(db)
            }
            FugitiveEtherealSignatureTemplate::FunctionGn(template) => {
                template.template_parameters(db)
            }
            FugitiveEtherealSignatureTemplate::TypeAlias(template) => {
                template.template_parameters(db)
            }
            FugitiveEtherealSignatureTemplate::Val(_template) => &[],
        }
    }
}

impl HasEtherealSignatureTemplate for FugitivePath {
    type EtherealSignatureTemplate = FugitiveEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        fugitive_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn fugitive_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    path: FugitivePath,
) -> EtherealSignatureResult<FugitiveEtherealSignatureTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        FugitiveDeclarativeSignatureTemplate::FunctionFn(declarative_signature_template) => {
            FunctionFnEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        FugitiveDeclarativeSignatureTemplate::FunctionGn(declarative_signature_template) => {
            GnFugitiveEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        FugitiveDeclarativeSignatureTemplate::TypeAlias(declarative_signature_template) => {
            TypeAliasEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        FugitiveDeclarativeSignatureTemplate::Val(declarative_signature_template) => {
            ValFugitiveEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
    })
}

mod associated_fn;
mod associated_val;
mod method_fn;
mod method_function;

pub use self::associated_fn::*;
pub use self::associated_val::*;
pub use self::method_fn::*;
pub use self::method_function::*;

use super::*;

pub trait HasTypeMethodEtherealSignatures: Copy {
    fn ty_method_ethereal_signature_templates_map<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &'a [(
            Ident,
            EtherealSignatureResult<TypeMethodEtherealSignatureTemplates>,
        )],
    >;

    fn ty_method_ethereal_signature_templates<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        ident: Ident,
    ) -> EtherealSignatureResult<Option<&'a TypeMethodEtherealSignatureTemplates>> {
        use vec_like::VecMapGetEntry;
        match self
            .ty_method_ethereal_signature_templates_map(db)?
            .get_entry(ident)
        {
            Some((_, Ok(templates))) => Ok(Some(templates)),
            Some((_, Err(e))) => Err(*e),
            None => Ok(None),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeMethodEtherealSignatureTemplates {
    MethodFn(SmallVecImpl<TypeMethodFnEtherealSignatureTemplate>),
    MethodFunction(SmallVecImpl<TypeMethodFunctionEtherealSignatureTemplate>),
}

impl HasTypeMethodEtherealSignatures for TypePath {
    fn ty_method_ethereal_signature_templates_map<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &'a [(
            Ident,
            EtherealSignatureResult<TypeMethodEtherealSignatureTemplates>,
        )],
    > {
        ty_method_ethereal_signature_templates_map(db, self)
            .as_ref()
            .map(|v| v as &[_])
            .map_err(|e| *e)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
pub(crate) fn ty_method_ethereal_signature_templates_map(
    db: &dyn EtherealSignatureDb,
    ty_path: TypePath,
) -> EtherealSignatureResult<
    IdentPairMap<EtherealSignatureResult<TypeMethodEtherealSignatureTemplates>>,
> {
    Ok(ty_path
        .ty_method_declarative_signature_templates_map(db)?
        .iter()
        .map(|(ident, result)| {
            let result = match result {
                Ok(templates) => match templates {
                    TypeMethodDeclarativeSignatureTemplates::MethodFn(templates) => templates
                        .iter()
                        .copied()
                        .map(|template| template.ethereal_signature_template(db))
                        .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
                        .map(TypeMethodEtherealSignatureTemplates::MethodFn),
                    TypeMethodDeclarativeSignatureTemplates::MethodFunction(templates) => todo!(),
                },
                Err(e) => Err(todo!()),
            };
            (*ident, result)
        })
        .collect())
}

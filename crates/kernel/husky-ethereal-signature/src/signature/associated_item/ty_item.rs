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
    Fn(SmallVecImpl<TypeMethodFnEtherealSignatureTemplate>),
    Function(SmallVecImpl<TypeMethodFunctionEtherealSignatureTemplate>),
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
    ty_path.ty_method_declarative_signature_templates_map(db);
    todo!()
}

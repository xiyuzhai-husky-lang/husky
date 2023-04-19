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
    ) -> &'a IdentPairMap<SmallVec<[TypeMethodEtherealSignatureTemplate; 2]>>;

    fn ty_method_ethereal_signature_templates<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        ident: Ident,
    ) -> Option<&'a [TypeMethodEtherealSignatureTemplate]> {
        self.ty_method_ethereal_signature_templates_map(db)
            .get_entry(ident)
            .map(|v| v.1.as_ref())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeMethodEtherealSignatureTemplate {
    Fn(TypeMethodFnEtherealSignatureTemplate),
    Function(TypeMethodFunctionEtherealSignatureTemplate),
}

impl HasTypeMethodEtherealSignatures for TypePath {
    fn ty_method_ethereal_signature_templates_map<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
    ) -> &'a IdentPairMap<SmallVec<[TypeMethodEtherealSignatureTemplate; 2]>> {
        ty_method_ethereal_signature_templates_map(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
pub(crate) fn ty_method_ethereal_signature_templates_map(
    db: &dyn EtherealSignatureDb,
    ty_path: TypePath,
) -> IdentPairMap<SmallVec<[TypeMethodEtherealSignatureTemplate; 2]>> {
    ty_path.ty_method_declarative_signature_templates_map(db);
    todo!()
}

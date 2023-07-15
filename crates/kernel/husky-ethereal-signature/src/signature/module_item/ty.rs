mod r#enum;
mod r#extern;
mod inductive;
mod props_struct;
mod record;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::record::*;
pub use self::structure::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use crate::*;
use husky_coword::Ident;
use husky_declarative_signature::{
    HasDeclarativeSignatureTemplate, TypeDeclarativeSignature, TypeDeclarativeSignatureTemplate,
};
use husky_entity_path::TypePath;
use husky_print_utils::p;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TypeEtherealSignatureTemplate {
    Enum(EnumEtherealSignatureTemplate),
    PropsStruct(PropsStructEtherealSignatureTemplate),
    UnitStruct(UnitStructEtherealSignatureTemplate),
    TupleStruct(TupleStructEtherealSignatureTemplate),
    Record(RecordEtherealSignatureTemplate),
    Inductive(InductiveEtherealSignatureTemplate),
    Structure(StructureEtherealSignatureTemplate),
    Extern(ExternEtherealSignatureTemplate),
    Union(UnionEtherealSignatureTemplate),
}

impl HasEtherealSignatureTemplate for TypePath {
    type EtherealSignatureTemplate = TypeEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        ty_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn ty_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    path: TypePath,
) -> EtherealSignatureResult<TypeEtherealSignatureTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        TypeDeclarativeSignatureTemplate::Enum(declarative_signature_template) => {
            EnumEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::PropsStruct(declarative_signature_template) => {
            PropsStructEtherealSignatureTemplate::from_declarative(
                db,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::UnitStruct(_) => todo!(),
        TypeDeclarativeSignatureTemplate::TupleStruct(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Record(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Inductive(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Structure(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Extern(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Union(_) => todo!(),
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EtherealSignatureDb)]
#[enum_class::from_variants]
pub enum RegularFieldEtherealSignature {
    PropsStruct(PropsStructFieldEtherealSignature),
}

pub trait HasRegularFieldEtherealSignature: Copy {
    fn regular_field_ethereal_signature(
        self,
        db: &dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<RegularFieldEtherealSignature>;
}

impl HasRegularFieldEtherealSignature for TypePath {
    fn regular_field_ethereal_signature(
        self,
        db: &dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<RegularFieldEtherealSignature> {
        self.ethereal_signature_template(db)?
            .regular_field_ethereal_signature(db, arguments, ident)
    }
}

impl HasRegularFieldEtherealSignature for TypeEtherealSignatureTemplate {
    fn regular_field_ethereal_signature(
        self,
        db: &dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<RegularFieldEtherealSignature> {
        match self {
            TypeEtherealSignatureTemplate::Enum(ethereal_signature_template) => Nothing,
            TypeEtherealSignatureTemplate::PropsStruct(ethereal_signature_template) => {
                ethereal_signature_template.regular_field_ethereal_signature(db, arguments, ident)
            }
            TypeEtherealSignatureTemplate::UnitStruct(_) => todo!(),
            TypeEtherealSignatureTemplate::TupleStruct(_) => todo!(),
            TypeEtherealSignatureTemplate::Record(_) => todo!(),
            TypeEtherealSignatureTemplate::Inductive(_) => todo!(),
            TypeEtherealSignatureTemplate::Structure(_) => todo!(),
            TypeEtherealSignatureTemplate::Extern(_) => todo!(),
            TypeEtherealSignatureTemplate::Union(_) => todo!(),
        }
    }
}

impl RegularFieldEtherealSignature {
    pub fn ty(self) -> EtherealTerm {
        match self {
            RegularFieldEtherealSignature::PropsStruct(signature) => signature.ty(),
        }
    }
}

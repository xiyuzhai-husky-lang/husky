mod r#enum;
mod r#extern;
mod inductive;
mod props_struct;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::structure::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use crate::*;
use husky_coword::Ident;
use husky_dec_signature::{HasDecTemplate, TypeDecTemplate};
use husky_entity_path::TypePath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
pub enum TypeEthTemplate {
    Enum(EnumEthTemplate),
    PropsStruct(PropsStructEthTemplate),
    UnitStruct(UnitStructEthTemplate),
    TupleStruct(TupleStructEthTemplate),
    Inductive(InductiveTypeEthTemplate),
    Structure(StructureTypeEthTemplate),
    Extern(ExternTypeEthTemplate),
    Union(UnionTypeEthTemplate),
}

impl TypeEthTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[EthTemplateParameter] {
        match self {
            TypeEthTemplate::Enum(template) => template.template_parameters(db),
            TypeEthTemplate::PropsStruct(template) => template.template_parameters(db),
            TypeEthTemplate::UnitStruct(template) => template.template_parameters(db),
            TypeEthTemplate::TupleStruct(template) => template.template_parameters(db),
            TypeEthTemplate::Inductive(template) => template.template_parameters(db),
            TypeEthTemplate::Structure(template) => template.template_parameters(db),
            TypeEthTemplate::Extern(template) => template.template_parameters(db),
            TypeEthTemplate::Union(template) => template.template_parameters(db),
        }
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            TypeEthTemplate::Enum(_) => None,
            TypeEthTemplate::PropsStruct(slf) => Some(slf.instance_constructor_ty(db).into()),
            TypeEthTemplate::UnitStruct(slf) => Some(slf.instance_constructor_ty(db).into()),
            TypeEthTemplate::TupleStruct(slf) => Some(slf.instance_constructor_ty(db).into()),
            TypeEthTemplate::Inductive(_) => None,
            TypeEthTemplate::Structure(_) => todo!(),
            TypeEthTemplate::Extern(_) => None,
            TypeEthTemplate::Union(_) => None,
        }
    }
}

impl HasEthTemplate for TypePath {
    type EthTemplate = TypeEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        ty_eth_template(db, self)
    }
}

#[salsa::tracked()]
fn ty_eth_template(db: &::salsa::Db, path: TypePath) -> EtherealSignatureResult<TypeEthTemplate> {
    Ok(match path.dec_template(db)? {
        TypeDecTemplate::Enum(dec_template) => {
            EnumEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TypeDecTemplate::PropsStruct(dec_template) => {
            PropsStructEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TypeDecTemplate::UnitStruct(dec_template) => {
            UnitStructEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TypeDecTemplate::TupleStruct(dec_template) => {
            TupleStructEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TypeDecTemplate::Inductive(dec_template) => {
            InductiveTypeEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TypeDecTemplate::Structure(dec_template) => {
            StructureTypeEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TypeDecTemplate::Extern(dec_template) => {
            ExternTypeEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TypeDecTemplate::Union(dec_template) => {
            UnionTypeEthTemplate::from_dec(db, path, dec_template)?.into()
        }
    })
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum PropsFieldEtherealSignature {
    PropsStruct(PropsStructFieldEtherealSignature),
}

pub trait HasPropsFieldEtherealSignature: Copy {
    fn props_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EthTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature>;
}

impl HasPropsFieldEtherealSignature for TypePath {
    fn props_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EthTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature> {
        self.eth_template(db)?
            .props_field_ethereal_signature(db, arguments, ident)
    }
}

impl HasPropsFieldEtherealSignature for TypeEthTemplate {
    fn props_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EthTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature> {
        match self {
            TypeEthTemplate::Enum(_)
            | TypeEthTemplate::Inductive(_)
            | TypeEthTemplate::UnitStruct(_)
            | TypeEthTemplate::TupleStruct(_)
            | TypeEthTemplate::Extern(_) => Nothing,
            TypeEthTemplate::PropsStruct(eth_template) => {
                eth_template.props_field_ethereal_signature(db, arguments, ident)
            }
            TypeEthTemplate::Structure(_) => todo!(),
            TypeEthTemplate::Union(_) => todo!(),
        }
    }
}

impl PropsFieldEtherealSignature {
    pub fn ty(self) -> EthTerm {
        match self {
            PropsFieldEtherealSignature::PropsStruct(signature) => signature.ty(),
        }
    }
}

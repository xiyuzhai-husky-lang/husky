use super::*;
use husky_declarative_signature::{
    PropsStructDeclarativeSignatureTemplate, PropsStructFieldDeclarativeSignatureTemplate,
};

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct PropsStructEtherealSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterEtherealSignatures,
    #[return_ref]
    pub fields: SmallVec<[RegularFieldEtherealSignatureTemplate; 4]>,
}

impl HasRegularFieldEtherealSignature for PropsStructEtherealSignatureTemplate {
    fn regular_field_ethereal_signature(
        self,
        db: &dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<RegularFieldEtherealSignature> {
        let field = self
            .fields(db)
            .iter()
            .copied()
            .find(|field| field.ident == ident)?;
        JustOk(
            field
                .instantiate(self.implicit_parameters(db), arguments)
                .into(),
        )
    }
}

impl HasEtherealSignatureTemplate for PropsStructDeclarativeSignatureTemplate {
    type EtherealSignatureTemplate = PropsStructEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        props_struct_ethereal_signature_template(db, self)
    }
}

pub(crate) fn props_struct_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    declarative_signature_template: PropsStructDeclarativeSignatureTemplate,
) -> EtherealSignatureResult<PropsStructEtherealSignatureTemplate> {
    let implicit_parameters = ImplicitParameterEtherealSignatures::from_declarative(
        db,
        declarative_signature_template.implicit_parameters(db),
    )?;
    let fields = declarative_signature_template
        .fields(db)
        .iter()
        .copied()
        .map(|declarative_signature_template| {
            RegularFieldEtherealSignatureTemplate::from_declarative(
                db,
                declarative_signature_template,
            )
        })
        .collect::<EtherealSignatureResult<_>>()?;
    Ok(PropsStructEtherealSignatureTemplate::new(
        db,
        implicit_parameters,
        fields,
    ))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EtherealSignatureDb, jar= EtherealSignatureJar)]
pub struct RegularFieldEtherealSignatureTemplate {
    ident: Ident,
    ty: EtherealTerm,
}

impl RegularFieldEtherealSignatureTemplate {
    fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature_template: PropsStructFieldDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self {
            ident: declarative_signature_template.ident(),
            ty: EtherealTerm::ty_from_declarative(db, declarative_signature_template.ty())?,
        })
    }

    fn instantiate(
        self,
        implicit_parameters: &ImplicitParameterEtherealSignatures,
        arguments: &[EtherealTerm],
    ) -> PropsStructFieldEtherealSignature {
        if implicit_parameters.data().len() != arguments.len() {
            todo!()
        }

        if implicit_parameters.data().len() == 0 {
            return PropsStructFieldEtherealSignature {
                ident: self.ident,
                ty: self.ty,
            };
        }
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EtherealSignatureDb)]
pub struct PropsStructFieldEtherealSignature {
    ident: Ident,
    ty: EtherealTerm,
}

impl PropsStructFieldEtherealSignature {
    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}

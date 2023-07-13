use super::*;
use husky_declarative_signature::{
    PropsStructDeclarativeSignatureTemplate, PropsStructFieldDeclarativeSignatureTemplate,
};

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct PropsStructEtherealSignatureTemplate {
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
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
                .instantiate(self.generic_parameters(db), arguments)
                .into(),
        )
    }
}

impl PropsStructEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature_template: PropsStructDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let generic_parameters = EtherealGenericParameters::from_declarative(
            db,
            declarative_signature_template.generic_parameters(db),
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
        Ok(Self::new(db, generic_parameters, fields))
    }
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

    // todo: move this to trait
    fn instantiate(
        self,
        generic_parameters: &EtherealGenericParameters,
        arguments: &[EtherealTerm],
    ) -> PropsStructFieldEtherealSignature {
        if generic_parameters.data().len() != arguments.len() {
            todo!()
        }

        if generic_parameters.data().len() == 0 {
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

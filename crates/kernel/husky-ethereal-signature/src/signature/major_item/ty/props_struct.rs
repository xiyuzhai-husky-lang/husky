use super::*;
use husky_declarative_signature::{
    PropsStructFieldDeclarativeSignatureTemplate, PropsStructTypeDeclarativeSignatureTemplate,
};

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct PropsStructTypeEtherealSignatureTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[PropsFieldEtherealSignatureTemplate; 4]>,
    pub instance_constructor_ritchie_ty: EtherealTermRitchie,
}

impl HasPropsFieldEtherealSignature for PropsStructTypeEtherealSignatureTemplate {
    fn regular_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature> {
        let field = self
            .fields(db)
            .iter()
            .copied()
            .find(|field| field.ident == ident)?;
        JustOk(
            field
                .instantiate(self.template_parameters(db), arguments)
                .into(),
        )
    }
}

impl PropsStructTypeEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypePath,
        tmpl: PropsStructTypeDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EtherealTemplateParameters::from_declarative(db, tmpl.template_parameters(db))?;
        let fields = tmpl
            .fields(db)
            .iter()
            .copied()
            .map(|declarative_signature_template| {
                PropsFieldEtherealSignatureTemplate::from_declarative(
                    db,
                    declarative_signature_template,
                )
            })
            .collect::<EtherealSignatureResult<_>>()?;
        let instance_constructor_ritchie_ty =
            EtherealTermRitchie::from_declarative(db, tmpl.instance_constructor_ritchie_ty(db))?;
        Ok(Self::new(
            db,
            path,
            template_parameters,
            fields,
            instance_constructor_ritchie_ty,
        ))
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EtherealTerm {
        self.instance_constructor_ritchie_ty(db).into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar= EtherealSignatureJar)]
pub struct PropsFieldEtherealSignatureTemplate {
    ident: Ident,
    ty: EtherealTerm,
}

impl PropsFieldEtherealSignatureTemplate {
    fn from_declarative(
        db: &::salsa::Db,
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
        template_parameters: &EtherealTemplateParameters,
        arguments: &[EtherealTerm],
    ) -> PropsStructFieldEtherealSignature {
        if template_parameters.data().len() != arguments.len() {
            todo!()
        }

        if template_parameters.data().len() == 0 {
            return PropsStructFieldEtherealSignature {
                ident: self.ident,
                ty: self.ty,
            };
        }
        todo!()
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
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

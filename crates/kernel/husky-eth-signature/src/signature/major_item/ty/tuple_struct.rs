use super::*;
use husky_dec_signature::{TupleStructFieldDecTemplate, TupleStructTypeDecTemplate};

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TupleStructTypeEthTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[TupleFieldEthTemplate; 2]>,
    pub instance_constructor_ritchie_ty: RitchieEthTerm,
}

impl TupleStructTypeEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypePath,
        tmpl: TupleStructTypeDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_declarative(db, tmpl.template_parameters(db))?;
        let fields = tmpl
            .fields(db)
            .iter()
            .copied()
            .map(|dec_template| TupleFieldEthTemplate::from_declarative(db, dec_template))
            .collect::<EtherealSignatureResult<_>>()?;
        let instance_constructor_ritchie_ty =
            RitchieEthTerm::from_declarative(db, tmpl.instance_constructor_ritchie_ty(db))?;
        Ok(Self::new(
            db,
            path,
            template_parameters,
            fields,
            instance_constructor_ritchie_ty,
        ))
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EthTerm {
        self.instance_constructor_ritchie_ty(db).into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct TupleFieldEthTemplate {
    ty: EthTerm,
}

impl TupleFieldEthTemplate {
    fn from_declarative(
        db: &::salsa::Db,
        dec_template: TupleStructFieldDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self {
            ty: EthTerm::ty_from_declarative(db, dec_template.ty())?,
        })
    }

    pub fn ty(&self) -> EthTerm {
        self.ty
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TupleStructFieldEtherealSignature {
    ty: EthTerm,
}

impl TupleStructFieldEtherealSignature {
    pub fn ty(&self) -> EthTerm {
        self.ty
    }
}

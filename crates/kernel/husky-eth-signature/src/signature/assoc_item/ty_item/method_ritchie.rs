use super::*;
use husky_dec_signature::signature::assoc_item::ty_item::method_ritchie::TypeMethodRitchieDecTemplate;
use husky_eth_term::term::ritchie::EthRitchieSimpleParameter;

#[salsa::interned]
pub struct TypeMethodRitchieEthTemplate {
    pub path: TypeItemPath,
    pub self_ty: EthTerm,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub self_value_parameter: EthRitchieSimpleParameter,
    #[return_ref]
    pub parenate_parameters: EtherealParenateParameters,
    pub return_ty: EthTerm,
}

impl TypeMethodRitchieEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypeItemPath,
        tmpl: TypeMethodRitchieDecTemplate,
    ) -> EthSignatureResult<Self> {
        let self_ty = EthTerm::ty_from_dec(db, tmpl.self_ty(db))?;
        let template_parameters =
            EthTemplateParameters::from_dec(db, tmpl.template_parameters(db))?;
        let self_value_parameter =
            EthRitchieSimpleParameter::from_dec(db, tmpl.self_value_parameter(db))?;
        let parenate_parameters =
            EtherealParenateParameters::from_dec(db, tmpl.parenate_parameters(db))?;
        let return_ty = EthTerm::ty_from_dec(db, tmpl.return_ty(db))?;
        Ok(Self::new(
            db,
            path,
            self_ty,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        ))
    }
}

pub struct TypeMethodRitchieEthSignature {
    path: TypeItemPath,
    self_ty: EthTerm,
    self_value_parameter: EthRitchieSimpleParameter,
    parenate_parameters: EtherealParenateParameters,
    return_ty: EthTerm,
    instantiation: EthInstantiation,
}

/// # getters
impl TypeMethodRitchieEthSignature {
    pub fn path(&self) -> TypeItemPath {
        self.path
    }

    pub fn self_ty(&self) -> EthTerm {
        self.self_ty
    }

    pub fn self_value_parameter(&self) -> EthRitchieSimpleParameter {
        self.self_value_parameter
    }

    pub fn parenate_parameters(&self) -> &EtherealParenateParameters {
        &self.parenate_parameters
    }

    pub fn return_ty(&self) -> EthTerm {
        self.return_ty
    }

    pub fn instantiation(&self) -> &EthInstantiation {
        &self.instantiation
    }
}

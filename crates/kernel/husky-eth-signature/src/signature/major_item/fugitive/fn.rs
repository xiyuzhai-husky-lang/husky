use husky_term_prelude::{RitchieKind, TypeRitchieKind};

use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct FunctionFnEthTemplate {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub ritchie_ty: EthRitchie,
}

impl FunctionFnEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: FugitivePath,
        tmpl: MajorFnDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_params =
            EthTemplateParameters::from_declarative(db, tmpl.template_parameters(db))?;
        let ritchie_params: SmallVec<[_; 4]> = tmpl
            .parenate_parameters(db)
            .iter()
            .map(|&param| EtherealRitchieParameter::from_declarative(param, db))
            .collect::<EthTermResult<SmallVec<[_; 4]>>>()?;
        let return_ty = EthTerm::ty_from_declarative(db, tmpl.return_ty(db))?;
        let ritchie_ty = EthRitchie::new(
            db,
            RitchieKind::Type(TypeRitchieKind::Fn),
            ritchie_params,
            return_ty,
        )?;
        Ok(Self::new(db, path, template_params, ritchie_ty))
    }
}

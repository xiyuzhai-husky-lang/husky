use husky_term_prelude::{RitchieKind, RitchieTypeKind};

use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct GnFugitiveEthTemplate {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub ritchie_ty: RitchieEthTerm,
}

impl GnFugitiveEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: FugitivePath,
        tmpl: MajorGnDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_declarative(db, tmpl.template_parameters(db))?;
        let ritchie_params: SmallVec<[_; 4]> = tmpl
            .parenate_parameters(db)
            .iter()
            .map(|&param| EtherealRitchieParameter::from_declarative(param, db))
            .collect::<EthTermResult<SmallVec<[_; 4]>>>()?;
        let return_ty = EthTerm::ty_from_declarative(db, tmpl.return_ty(db))?;
        let ritchie_ty = RitchieEthTerm::new(
            db,
            RitchieKind::Type(RitchieTypeKind::Gn),
            ritchie_params,
            return_ty,
        )?;
        Ok(Self::new(db, path, template_parameters, ritchie_ty))
    }
}

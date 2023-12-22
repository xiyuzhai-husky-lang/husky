use husky_term_prelude::{RitchieKind, RitchieTypeKind};

use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct GnFugitiveEtherealSignatureTemplate {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    pub ritchie_ty: EtherealTermRitchie,
}

impl GnFugitiveEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: FugitivePath,
        tmpl: GnFugitiveDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EtherealTemplateParameters::from_declarative(db, tmpl.template_parameters(db))?;
        let ritchie_params: SmallVec<[_; 4]> = tmpl
            .parenate_parameters(db)
            .iter()
            .map(|&param| EtherealRitchieParameter::from_declarative(param, db))
            .collect::<EtherealTermResult<SmallVec<[_; 4]>>>()?;
        let return_ty = EtherealTerm::ty_from_declarative(db, tmpl.return_ty(db))?;
        let ritchie_ty = EtherealTermRitchie::new(
            db,
            RitchieKind::Type(RitchieTypeKind::Gn),
            ritchie_params,
            return_ty,
        )?;
        Ok(Self::new(db, path, template_parameters, ritchie_ty))
    }
}

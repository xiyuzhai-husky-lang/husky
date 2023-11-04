use super::*;
use husky_syn_decl::FunctionFnSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct FnFugitiveHirDecl {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
}

impl FnFugitiveHirDecl {
    pub(super) fn from_syn(
        path: FugitivePath,
        syn_decl: FunctionFnSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let template_parameters =
            HirTemplateParameters::from_ethereal(syn_decl.template_parameters(db), db);
        Self::new(db, path, template_parameters)
    }
}

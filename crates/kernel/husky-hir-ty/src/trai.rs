use crate::*;

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirTrait {
    pub trai_path: TraitPath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: HirTemplateArguments,
}

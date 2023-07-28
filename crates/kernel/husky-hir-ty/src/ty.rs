use crate::*;

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirType {
    pub ty_path: TypePath,
    #[return_ref]
    pub template_arguments: HirTemplateArguments,
}

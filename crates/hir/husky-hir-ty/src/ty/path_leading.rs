use super::*;
use husky_entity_path::path::major_item::ty::TypePath;

#[salsa::interned(jar = HirTypeJar)]
pub struct HirTypePathLeading {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: HirTemplateArguments,
    pub always_copyable: bool,
}

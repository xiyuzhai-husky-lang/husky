use crate::{
    instantiation::{LinkageInstantiate, LinkageInstantiation},
    jar::LinkageJar,
    template_argument::{LinkageTemplateArgument, LinkageTemplateArguments},
};
use husky_entity_path::TraitPath;
use husky_hir_ty::trai::HirTrait;

#[salsa::interned(jar = LinkageJar, constructor = new)]
pub struct LinkageTrait {
    pub trai_path: TraitPath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: LinkageTemplateArguments,
}

impl LinkageInstantiate for HirTrait {
    type Output = LinkageTrait;

    fn linkage_instantiate(
        self,
        linkage_instantiation: &LinkageInstantiation,
        db: &salsa::Db,
    ) -> Self::Output {
        let trai_path = self.trai_path(db);
        let template_arguments = LinkageTemplateArgument::from_hir_template_arguments(
            self.template_arguments(db),
            Some(linkage_instantiation),
            db,
        );
        LinkageTrait::new(db, trai_path, template_arguments)
    }
}

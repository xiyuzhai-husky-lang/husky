use crate::{
    instantiation::{LinInstantiation, LinkageInstantiate},
    jar::LinkageJar,
    template_argument::{LinTemplateArgument, LinTemplateArguments},
};
use husky_entity_path::TraitPath;
use husky_hir_ty::trai::HirTrait;

#[salsa::interned(jar = LinkageJar, constructor = new)]
pub struct LinkageTrait {
    pub trai_path: TraitPath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: LinTemplateArguments,
}

impl LinkageInstantiate for HirTrait {
    type Output = LinkageTrait;

    fn linkage_instantiate(
        self,
        lin_instantiation: &LinInstantiation,
        db: &salsa::Db,
    ) -> Self::Output {
        let trai_path = self.trai_path(db);
        let template_arguments = LinTemplateArgument::from_hir_template_arguments(
            self.template_arguments(db),
            Some(lin_instantiation),
            db,
        );
        LinkageTrait::new(db, trai_path, template_arguments)
    }
}

use crate::{
    instantiation::{LinInstantiation, LinketInstantiate},
    jar::LinketJar,
    template_argument::{LinTemplateArgument, LinTemplateArguments},
};
use husky_entity_path::path::major_item::trai::TraitPath;
use husky_hir_ty::trai::HirTrait;

#[salsa::interned(constructor = new)]
pub struct LinketTrait {
    pub trai_path: TraitPath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: LinTemplateArguments,
}

impl LinketInstantiate for HirTrait {
    type Output = LinketTrait;

    fn linket_instantiate(self, instantiation: &LinInstantiation, db: &salsa::Db) -> Self::Output {
        let trai_path = self.trai_path(db);
        let template_arguments = LinTemplateArgument::from_hir_template_arguments(
            self.template_arguments(db),
            instantiation,
            db,
        );
        LinketTrait::new(db, trai_path, template_arguments)
    }
}

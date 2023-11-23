use super::*;
use husky_entity_path::TypePath;
use husky_hir_ty::HirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = LinkageDb)]
#[enum_class::from_variants]
pub enum LinkageType {
    PathLeading(LinkageTypePathLeading),
    Ritchie(LinkageTypeRitchie),
}

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = new)]
pub struct LinkageTypePathLeading {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: LinkageTemplateArguments,
}

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = new)]
pub struct LinkageTypeRitchie {}

impl LinkageType {
    pub(super) fn from_hir(hir_ty: HirType, db: &dyn LinkageDb) -> Self {
        match hir_ty {
            HirType::PathLeading(hir_ty) => LinkageTypePathLeading::new(
                db,
                hir_ty.ty_path(db),
                LinkageTemplateArgument::from_hir_template_arguments(
                    hir_ty.template_arguments(db),
                    db,
                ),
            )
            .into(),
            HirType::Symbol(_) => unreachable!(),
            HirType::TypeAssociatedType(_) => unreachable!(),
            HirType::TraitAssociatedType(_) => unreachable!(),
            HirType::Ritchie(_) => todo!(),
        }
    }
}

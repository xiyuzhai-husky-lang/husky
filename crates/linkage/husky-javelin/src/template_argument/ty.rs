use super::*;
use husky_entity_path::TypePath;
use husky_hir_ty::HirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum JavelinType {
    PathLeading(JavelinTypePathLeading),
    Ritchie(JavelinTypeRitchie),
}

#[salsa::interned(db = JavelinDb, jar = JavelinJar, constructor = new)]
pub struct JavelinTypePathLeading {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: JavelinTemplateArguments,
}

#[salsa::interned(db = JavelinDb, jar = JavelinJar, constructor = new)]
pub struct JavelinTypeRitchie {}

impl JavelinType {
    pub(super) fn from_hir(
        hir_ty: HirType,
        javelin_instantiation: Option<&JavelinInstantiation>,
        db: &::salsa::Db,
    ) -> Self {
        match hir_ty {
            HirType::PathLeading(hir_ty) => JavelinTypePathLeading::new(
                db,
                hir_ty.ty_path(db),
                JavelinTemplateArgument::from_hir_template_arguments(
                    hir_ty.template_arguments(db),
                    javelin_instantiation,
                    db,
                ),
            )
            .into(),
            HirType::Symbol(_) => unreachable!(),
            HirType::TypeAssociatedType(_) => unreachable!(),
            HirType::TraitAssociatedType(_) => unreachable!(),
            HirType::Ritchie(_) => JavelinTypeRitchie::new(db).into(),
        }
    }
}

use super::*;
use husky_entity_path::TypePath;
use husky_hir_ty::{
    ritchie::{HirEagerContract, HirRitchieParameter},
    HirType,
};
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum JavelinType {
    PathLeading(JavelinTypePathLeading),
    Ritchie(JavelinRitchieType),
}

#[salsa::interned(db = JavelinDb, jar = JavelinJar, constructor = new)]
pub struct JavelinTypePathLeading {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: JavelinTemplateArguments,
}

#[salsa::interned(db = JavelinDb, jar = JavelinJar, constructor = new)]
pub struct JavelinRitchieType {
    pub parameters: SmallVec<[JavelinRitchieParameter; 4]>,
    pub return_ty: JavelinType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct JavelinRitchieParameter {
    contract: HirEagerContract,
    parameter_ty: JavelinType,
}

impl JavelinRitchieParameter {
    fn from_hir(
        param: HirRitchieParameter,
        javelin_instantiation: Option<&JavelinInstantiation>,
        db: &salsa::Db,
    ) -> Self {
        match param {
            HirRitchieParameter::Ordinary(param) => Self {
                contract: param.contract(),
                parameter_ty: JavelinType::from_hir(param.ty, javelin_instantiation, db),
            },
            HirRitchieParameter::Variadic(_) => todo!(),
            HirRitchieParameter::Keyed(_) => todo!(),
        }
    }

    pub fn contract(&self) -> HirEagerContract {
        self.contract
    }

    pub fn parameter_ty(&self) -> JavelinType {
        self.parameter_ty
    }
}

impl JavelinType {
    pub(crate) fn from_hir(
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
            HirType::Ritchie(hir_ty) => JavelinRitchieType::new(
                db,
                hir_ty
                    .parameters(db)
                    .iter()
                    .map(|&param| {
                        JavelinRitchieParameter::from_hir(param, javelin_instantiation, db)
                    })
                    .collect(),
                JavelinType::from_hir(hir_ty.return_ty(db), javelin_instantiation, db),
            )
            .into(),
        }
    }
}

use super::*;
use husky_entity_path::path::major_item::ty::TypePath;
use husky_hir_ty::{
    ritchie::{HirContract, HirRitchieParameter},
    HirType,
};
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum JavelinType {
    PathLeading(JavelinTypePathLeading),
    Ritchie(JavelinRitchieType),
}

#[salsa::interned(db = JavelinDb, constructor = new)]
pub struct JavelinTypePathLeading {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: JavTemplateArguments,
}

#[salsa::interned(db = JavelinDb, constructor = new)]
pub struct JavelinRitchieType {
    pub parameters: SmallVec<[JavelinRitchieParameter; 4]>,
    pub return_ty: JavelinType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct JavelinRitchieParameter {
    contract: HirContract,
    parameter_ty: JavelinType,
}

impl JavelinRitchieParameter {
    fn from_hir(
        param: HirRitchieParameter,
        jav_instantiation: &JavInstantiation,
        db: &salsa::Db,
    ) -> Self {
        match param {
            HirRitchieParameter::Simple(param) => Self {
                contract: param.contract(),
                parameter_ty: JavelinType::from_hir(param.ty, jav_instantiation, db),
            },
            HirRitchieParameter::Variadic(_) => todo!(),
            HirRitchieParameter::Keyed(_) => todo!(),
        }
    }

    pub fn contract(&self) -> HirContract {
        self.contract
    }

    pub fn parameter_ty(&self) -> JavelinType {
        self.parameter_ty
    }
}

impl JavelinType {
    pub(crate) fn from_hir(
        hir_ty: HirType,
        jav_instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match hir_ty {
            HirType::PathLeading(hir_ty) => JavelinTypePathLeading::new(
                db,
                hir_ty.ty_path(db),
                JavTemplateArgument::from_hir_template_arguments(
                    hir_ty.template_arguments(db),
                    jav_instantiation,
                    db,
                ),
            )
            .into(),
            HirType::Variable(symbol) => jav_instantiation.resolve_ty(symbol),
            HirType::TypeAssocType(_) => todo!(),
            HirType::TraitAssocType(_) => todo!(),
            HirType::Ritchie(hir_ty) => JavelinRitchieType::new(
                db,
                hir_ty
                    .parameters(db)
                    .iter()
                    .map(|&param| JavelinRitchieParameter::from_hir(param, jav_instantiation, db))
                    .collect(),
                JavelinType::from_hir(hir_ty.return_ty(db), jav_instantiation, db),
            )
            .into(),
            HirType::TypeVar(_) => todo!(),
        }
    }
}

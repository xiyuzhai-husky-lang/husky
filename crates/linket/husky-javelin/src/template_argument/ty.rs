use super::*;
use husky_entity_path::path::major_item::ty::TypePath;
use husky_hir_ty::{
    ritchie::{HirContract, HirRitchieParameter},
    HirType,
};
use husky_vfs::toolchain::Toolchain;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum JavType {
    PathLeading(JavTypePathLeading),
    Ritchie(JavRitchieType),
}

impl JavType {
    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        match self {
            JavType::PathLeading(slf) => slf.ty_path(db).toolchain(db),
            // ad hoc
            JavType::Ritchie(slf) => slf.return_ty(db).toolchain(db),
        }
    }
}

#[salsa::interned(db = JavelinDb, constructor = new)]
pub struct JavTypePathLeading {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: JavTemplateArguments,
}

#[salsa::interned(db = JavelinDb, constructor = new)]
pub struct JavRitchieType {
    pub parameters: SmallVec<[JavRitchieParameter; 4]>,
    pub return_ty: JavType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct JavRitchieParameter {
    contract: HirContract,
    parameter_ty: JavType,
}

impl JavRitchieParameter {
    fn from_hir(
        param: HirRitchieParameter,
        jav_instantiation: &JavInstantiation,
        db: &salsa::Db,
    ) -> Self {
        match param {
            HirRitchieParameter::Simple(param) => Self {
                contract: param.contract(),
                parameter_ty: JavType::from_hir(param.ty, jav_instantiation, db),
            },
            HirRitchieParameter::Variadic(_) => todo!(),
            HirRitchieParameter::Keyed(_) => todo!(),
        }
    }

    pub fn contract(&self) -> HirContract {
        self.contract
    }

    pub fn parameter_ty(&self) -> JavType {
        self.parameter_ty
    }
}

impl JavType {
    pub(crate) fn from_hir(
        hir_ty: HirType,
        instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match hir_ty {
            HirType::PathLeading(hir_ty) => JavTypePathLeading::new(
                db,
                hir_ty.ty_path(db),
                JavTemplateArgument::from_hir_template_arguments(
                    hir_ty.template_arguments(db),
                    instantiation,
                    db,
                ),
            )
            .into(),
            HirType::Variable(variable) => instantiation.resolve_ty(variable),
            HirType::TypeAssocType(_) => todo!(),
            HirType::TraitAssocType(_) => todo!(),
            HirType::Ritchie(hir_ty) => JavRitchieType::new(
                db,
                hir_ty
                    .parameters(db)
                    .iter()
                    .map(|&param| JavRitchieParameter::from_hir(param, instantiation, db))
                    .collect(),
                JavType::from_hir(hir_ty.return_ty(db), instantiation, db),
            )
            .into(),
            HirType::TypeVar(_) => todo!(),
            HirType::Quaried => todo!(),
        }
    }
}

use super::*;
use husky_entity_path::path::major_item::ty::TypePath;
use husky_hir_ty::{
    ritchie::{HirContract, HirRitchieParameter},
    HirType,
};
use husky_javelin::template_argument::ty::{JavRitchieParameter, JavType, JavTypePathLeading};

use smallvec::SmallVec;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinType {
    PathLeading(LinTypePathLeading),
    Ritchie(LinketRitchieType),
}

impl LinketInstantiate for HirType {
    type Output = LinType;

    fn linket_instantiate(self, instantiation: &LinInstantiation, db: &salsa::Db) -> Self::Output {
        match self {
            HirType::PathLeading(slf) => LinType::PathLeading(LinTypePathLeading::new(
                db,
                slf.ty_path(db),
                slf.template_arguments(db)
                    .iter()
                    .map(|&arg| LinTemplateArgument::from_hir(arg, instantiation, db))
                    .collect(),
            )),
            HirType::Variable(slf) => match instantiation.resolve(slf.into()) {
                LinTermSymbolResolution::Explicit(arg) => match arg {
                    LinTemplateArgument::Vacant => todo!(),
                    LinTemplateArgument::Type(linket_ty) => linket_ty,
                    LinTemplateArgument::Constant(_) => todo!(),
                    LinTemplateArgument::Lifetime => todo!(),
                    LinTemplateArgument::Qual(_) => todo!(),
                },
                LinTermSymbolResolution::SelfLifetime => todo!(),
                LinTermSymbolResolution::SelfQual(_) => todo!(),
            },
            HirType::TypeAssocType(_) => todo!(),
            HirType::TraitAssocType(_) => todo!(),
            HirType::Ritchie(_) => todo!(),
            HirType::TypeVar(_) => todo!(),
        }
    }
}

#[salsa::interned(constructor = pub(crate) new)]
pub struct LinTypePathLeading {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: LinTemplateArguments,
}

impl LinTypePathLeading {
    pub(crate) fn from_path_instantiation(
        ty_path: TypePath,
        instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        LinTypePathLeading::new(
            db,
            ty_path,
            instantiation
                .symbol_resolutions()
                .iter()
                .map(|(_, res)| match *res {
                    LinTermSymbolResolution::Explicit(arg) => arg,
                    LinTermSymbolResolution::SelfLifetime
                    | LinTermSymbolResolution::SelfQual(_) => unreachable!(),
                })
                .collect(),
        )
    }
}

#[salsa::interned(constructor = new)]
pub struct LinketRitchieType {
    pub parameters: SmallVec<[LinketRitchieParameter; 4]>,
    pub return_ty: LinType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LinketRitchieParameter {
    contract: HirContract,
    parameter_ty: LinType,
}
impl LinketRitchieParameter {
    fn from_jav(
        param: JavRitchieParameter,
        lin_instantiation: &LinInstantiation,
        db: &salsa::Db,
    ) -> Self {
        Self {
            contract: param.contract(),
            parameter_ty: LinType::from_jav(param.parameter_ty(), lin_instantiation, db),
        }
    }

    fn from_hir(
        param: HirRitchieParameter,
        instantiation: &LinInstantiation,
        db: &salsa::Db,
    ) -> Self {
        match param {
            HirRitchieParameter::Simple(param) => Self {
                contract: param.contract(),
                parameter_ty: LinType::from_hir(param.ty(), instantiation, db),
            },
            HirRitchieParameter::Variadic(_) => todo!(),
            HirRitchieParameter::Keyed(_) => todo!(),
        }
    }

    pub fn contract(&self) -> HirContract {
        self.contract
    }

    pub fn parameter_ty(&self) -> LinType {
        self.parameter_ty
    }
}

impl LinType {
    pub(crate) fn from_hir(
        hir_ty: HirType,
        instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match hir_ty {
            HirType::PathLeading(hir_ty) => LinTypePathLeading::new(
                db,
                hir_ty.ty_path(db),
                LinTemplateArgument::from_hir_template_arguments(
                    hir_ty.template_arguments(db),
                    instantiation,
                    db,
                ),
            )
            .into(),
            HirType::Variable(symbol) => match instantiation.resolve(symbol.into()) {
                LinTermSymbolResolution::Explicit(arg) => match arg {
                    LinTemplateArgument::Vacant => todo!(),
                    LinTemplateArgument::Type(linket_ty) => linket_ty,
                    LinTemplateArgument::Constant(_) => todo!(),
                    LinTemplateArgument::Lifetime => todo!(),
                    LinTemplateArgument::Qual(_) => todo!(),
                },
                LinTermSymbolResolution::SelfLifetime => todo!(),
                LinTermSymbolResolution::SelfQual(_) => todo!(),
            },
            HirType::TypeAssocType(_) => unreachable!(),
            HirType::TraitAssocType(_) => unreachable!(),
            HirType::Ritchie(hir_ty) => LinketRitchieType::new(
                db,
                hir_ty
                    .parameters(db)
                    .iter()
                    .map(|&param| LinketRitchieParameter::from_hir(param, instantiation, db))
                    .collect(),
                LinType::from_hir(hir_ty.return_ty(db), instantiation, db),
            )
            .into(),
            HirType::TypeVar(_) => todo!(),
        }
    }

    pub(crate) fn from_jav(
        jav_ty: JavType,
        instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match jav_ty {
            JavType::PathLeading(javelin_ty) => {
                LinTypePathLeading::from_jav(javelin_ty, instantiation, db).into()
            }
            JavType::Ritchie(javelin_ty) => LinketRitchieType::new(
                db,
                javelin_ty
                    .parameters(db)
                    .iter()
                    .map(|&param| LinketRitchieParameter::from_jav(param, instantiation, db))
                    .collect(),
                LinType::from_jav(javelin_ty.return_ty(db), instantiation, db),
            )
            .into(),
        }
    }
}

impl LinTypePathLeading {
    fn from_jav(
        javelin_ty: JavTypePathLeading,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        LinTypePathLeading::new(
            db,
            javelin_ty.ty_path(db),
            javelin_ty
                .template_arguments(db)
                .iter()
                .map(|&arg| LinTemplateArgument::from_jav(arg, lin_instantiation, db))
                .collect(),
        )
    }
}

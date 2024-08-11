use super::*;
use either::*;
use husky_entity_path::path::major_item::ty::{PreludeTypePath, TypePath};
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
    Ritchie(LinRitchieType),
}

impl LinType {
    pub fn is_copyable(self, db: &::salsa::Db) -> bool {
        match self {
            LinType::PathLeading(slf) => slf.is_copyable(db),
            LinType::Ritchie(slf) => true,
        }
    }
}

impl LinInstantiate for HirType {
    type Output = LinType;

    fn lin_instantiate(self, instantiation: &LinInstantiation, db: &salsa::Db) -> Self::Output {
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
                LinTermVariableResolution::Explicit(arg) => match arg {
                    LinTemplateArgument::Vacant => todo!(),
                    LinTemplateArgument::Type(linket_ty) => linket_ty,
                    LinTemplateArgument::Constant(_) => todo!(),
                    LinTemplateArgument::Lifetime => todo!(),
                    LinTemplateArgument::Qual(_) => todo!(),
                },
                LinTermVariableResolution::SelfLifetime => todo!(),
                LinTermVariableResolution::SelfQual(_) => todo!(),
            },
            HirType::TypeAssocType(_) => todo!(),
            HirType::TraitAssocType(_) => todo!(),
            HirType::Ritchie(_) => todo!(),
            HirType::TypeVar(_) => todo!(),
            HirType::Quaried => todo!(),
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
                .variable_resolutions()
                .iter()
                .map(|(_, res)| match *res {
                    LinTermVariableResolution::Explicit(arg) => arg,
                    LinTermVariableResolution::SelfLifetime
                    | LinTermVariableResolution::SelfQual(_) => unreachable!(),
                })
                .collect(),
        )
    }
}

impl LinTypePathLeading {
    pub fn is_copyable(self, db: &::salsa::Db) -> bool {
        match self.ty_path(db).refine(db) {
            Left(prelude_ty_path) => match prelude_ty_path {
                PreludeTypePath::Basic(_) => true,
                PreludeTypePath::Num(_) => true,
                PreludeTypePath::Indirection(_) => true,
                PreludeTypePath::Container(_) => false,
                PreludeTypePath::Nat => unreachable!(),
                PreludeTypePath::Lifetime => unreachable!(),
                PreludeTypePath::Place => unreachable!(),
                PreludeTypePath::Module => unreachable!(),
                PreludeTypePath::Trait => unreachable!(),
                PreludeTypePath::List => unreachable!(),
                PreludeTypePath::StringLiteral => unreachable!(),
                PreludeTypePath::Str => false,
                PreludeTypePath::Option => todo!(),
                PreludeTypePath::Result => todo!(),
                PreludeTypePath::Universe => unreachable!(),
            },
            // ad hoc
            Right(_) => false,
        }
    }
}

#[salsa::interned(constructor = new)]
pub struct LinRitchieType {
    pub parameters: SmallVec<[LinRitchieParameter; 4]>,
    pub return_ty: LinType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LinRitchieParameter {
    contract: HirContract,
    parameter_ty: LinType,
}
impl LinRitchieParameter {
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
                LinTermVariableResolution::Explicit(arg) => match arg {
                    LinTemplateArgument::Vacant => todo!(),
                    LinTemplateArgument::Type(linket_ty) => linket_ty,
                    LinTemplateArgument::Constant(_) => todo!(),
                    LinTemplateArgument::Lifetime => todo!(),
                    LinTemplateArgument::Qual(_) => todo!(),
                },
                LinTermVariableResolution::SelfLifetime => todo!(),
                LinTermVariableResolution::SelfQual(_) => todo!(),
            },
            HirType::TypeAssocType(_) => unreachable!(),
            HirType::TraitAssocType(_) => unreachable!(),
            HirType::Ritchie(hir_ty) => LinRitchieType::new(
                db,
                hir_ty
                    .parameters(db)
                    .iter()
                    .map(|&param| LinRitchieParameter::from_hir(param, instantiation, db))
                    .collect(),
                LinType::from_hir(hir_ty.return_ty(db), instantiation, db),
            )
            .into(),
            HirType::TypeVar(_) => todo!(),
            HirType::Quaried => todo!(),
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
            JavType::Ritchie(javelin_ty) => LinRitchieType::new(
                db,
                javelin_ty
                    .parameters(db)
                    .iter()
                    .map(|&param| LinRitchieParameter::from_jav(param, instantiation, db))
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

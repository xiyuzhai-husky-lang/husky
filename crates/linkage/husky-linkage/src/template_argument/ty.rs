use super::*;
use husky_entity_path::path::major_item::ty::TypePath;
use husky_hir_ty::{
    ritchie::{HirContract, HirRitchieParameter},
    HirType,
};
use husky_javelin::template_argument::ty::{
    JavelinRitchieParameter, JavelinType, JavelinTypePathLeading,
};

use smallvec::SmallVec;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinType {
    PathLeading(LinTypePathLeading),
    Ritchie(LinkageRitchieType),
}

impl LinkageInstantiate for HirType {
    type Output = LinType;

    fn linkage_instantiate(
        self,
        lin_instantiation: &LinInstantiation,
        db: &salsa::Db,
    ) -> Self::Output {
        match self {
            HirType::PathLeading(slf) => LinType::PathLeading(LinTypePathLeading::new(
                db,
                slf.ty_path(db),
                slf.template_arguments(db)
                    .iter()
                    .map(|&arg| LinTemplateArgument::from_hir(arg, Some(lin_instantiation), db))
                    .collect(),
            )),
            HirType::Variable(slf) => match lin_instantiation.resolve(slf.into()) {
                LinTermSymbolResolution::Explicit(arg) => match arg {
                    LinTemplateArgument::Vacant => todo!(),
                    LinTemplateArgument::Type(linkage_ty) => linkage_ty,
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
        }
    }
}

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = pub(crate) new)]
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

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = new)]
pub struct LinkageRitchieType {
    pub parameters: SmallVec<[LinkageRitchieParameter; 4]>,
    pub return_ty: LinType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LinkageRitchieParameter {
    contract: HirContract,
    parameter_ty: LinType,
}
impl LinkageRitchieParameter {
    fn from_javelin(
        param: JavelinRitchieParameter,
        lin_instantiation: &LinInstantiation,
        db: &salsa::Db,
    ) -> Self {
        Self {
            contract: param.contract(),
            parameter_ty: LinType::from_javelin(param.parameter_ty(), lin_instantiation, db),
        }
    }

    fn from_hir(
        param: HirRitchieParameter,
        lin_instantiation: Option<&LinInstantiation>,
        db: &salsa::Db,
    ) -> Self {
        match param {
            HirRitchieParameter::Simple(param) => Self {
                contract: param.contract(),
                parameter_ty: LinType::from_hir(param.ty(), lin_instantiation, db),
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
        lin_instantiation: Option<&LinInstantiation>,
        db: &::salsa::Db,
    ) -> Self {
        match hir_ty {
            HirType::PathLeading(hir_ty) => LinTypePathLeading::new(
                db,
                hir_ty.ty_path(db),
                LinTemplateArgument::from_hir_template_arguments(
                    hir_ty.template_arguments(db),
                    lin_instantiation,
                    db,
                ),
            )
            .into(),
            HirType::Variable(symbol) => match lin_instantiation {
                Some(lin_instantiation) => match lin_instantiation.resolve(symbol.into()) {
                    LinTermSymbolResolution::Explicit(arg) => match arg {
                        LinTemplateArgument::Vacant => todo!(),
                        LinTemplateArgument::Type(linkage_ty) => linkage_ty,
                        LinTemplateArgument::Constant(_) => todo!(),
                        LinTemplateArgument::Lifetime => todo!(),
                        LinTemplateArgument::Qual(_) => todo!(),
                    },
                    LinTermSymbolResolution::SelfLifetime => todo!(),
                    LinTermSymbolResolution::SelfQual(_) => todo!(),
                },
                None => todo!(),
            },
            HirType::TypeAssocType(_) => unreachable!(),
            HirType::TraitAssocType(_) => unreachable!(),
            HirType::Ritchie(hir_ty) => LinkageRitchieType::new(
                db,
                hir_ty
                    .parameters(db)
                    .iter()
                    .map(|&param| LinkageRitchieParameter::from_hir(param, lin_instantiation, db))
                    .collect(),
                LinType::from_hir(hir_ty.return_ty(db), lin_instantiation, db),
            )
            .into(),
        }
    }

    pub(crate) fn from_javelin(
        javelin_ty: JavelinType,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match javelin_ty {
            JavelinType::PathLeading(javelin_ty) => {
                LinTypePathLeading::from_javelin(javelin_ty, lin_instantiation, db).into()
            }
            JavelinType::Ritchie(javelin_ty) => LinkageRitchieType::new(
                db,
                javelin_ty
                    .parameters(db)
                    .iter()
                    .map(|&param| {
                        LinkageRitchieParameter::from_javelin(param, lin_instantiation, db)
                    })
                    .collect(),
                LinType::from_javelin(javelin_ty.return_ty(db), lin_instantiation, db),
            )
            .into(),
        }
    }
}

impl LinTypePathLeading {
    fn from_javelin(
        javelin_ty: JavelinTypePathLeading,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        LinTypePathLeading::new(
            db,
            javelin_ty.ty_path(db),
            javelin_ty
                .template_arguments(db)
                .iter()
                .map(|&arg| LinTemplateArgument::from_javelin(arg, lin_instantiation, db))
                .collect(),
        )
    }
}

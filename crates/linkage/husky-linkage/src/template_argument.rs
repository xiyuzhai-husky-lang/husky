pub mod constant;
pub mod qual;
pub mod ty;

use self::{constant::LinConstant, ty::*};
use super::*;
use crate::{instantiation::LinInstantiation, template_argument::qual::LinQual};
use husky_hir_ty::HirTemplateArgument;
use husky_javelin::template_argument::JavTemplateArgument;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinTemplateArgument {
    Vacant,
    Type(LinType),
    Constant(LinConstant),
    Lifetime,
    Qual(LinQual),
}

impl LinTemplateArgument {
    pub(crate) fn from_hir_template_arguments(
        template_arguments: &[HirTemplateArgument],
        lin_instantiation: Option<&LinInstantiation>,
        db: &::salsa::Db,
    ) -> LinTemplateArguments {
        template_arguments
            .iter()
            .map(|&template_argument| {
                LinTemplateArgument::from_hir(template_argument, lin_instantiation, db)
            })
            .collect()
    }

    pub(crate) fn from_hir(
        arg: HirTemplateArgument,
        instantiation: Option<&LinInstantiation>,
        db: &::salsa::Db,
    ) -> Self {
        match arg {
            HirTemplateArgument::Vacant => LinTemplateArgument::Vacant,
            HirTemplateArgument::Type(hir_ty) => {
                LinTemplateArgument::Type(LinType::from_hir(hir_ty, instantiation, db))
            }
            HirTemplateArgument::Constant(hir_constant) => {
                LinTemplateArgument::Constant(LinConstant::from_hir(hir_constant, instantiation))
            }
            HirTemplateArgument::Lifetime(_) => LinTemplateArgument::Lifetime,
            HirTemplateArgument::ContractedQuary(contracted_quary) => {
                LinTemplateArgument::Qual(LinQual::from_hir(contracted_quary))
            }
        }
    }

    pub(crate) fn from_javelin(
        arg: JavTemplateArgument,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match arg {
            JavTemplateArgument::Vacant => todo!(),
            JavTemplateArgument::Type(javelin_ty) => {
                LinTemplateArgument::Type(LinType::from_javelin(javelin_ty, lin_instantiation, db))
            }
            JavTemplateArgument::Constant(constant) => {
                LinTemplateArgument::Constant(LinConstant(constant))
            }
            JavTemplateArgument::Lifetime => todo!(),
            JavTemplateArgument::Place => todo!(),
        }
    }
}

pub type LinTemplateArguments = smallvec::SmallVec<[LinTemplateArgument; 2]>;

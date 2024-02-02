pub mod constant;
pub mod place;
pub mod ty;

use self::{constant::LinConstant, ty::*};
use super::*;
use crate::{instantiation::LinkageInstantiation, template_argument::place::LinPlace};
use husky_hir_ty::HirTemplateArgument;
use husky_javelin::template_argument::JavTemplateArgument;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinTemplateArgument {
    Vacant,
    Type(LinType),
    Constant(LinConstant),
    Lifetime,
    Place(LinPlace),
}

impl LinTemplateArgument {
    pub(crate) fn from_hir_template_arguments(
        template_arguments: &[HirTemplateArgument],
        linkage_instantiation: Option<&LinkageInstantiation>,
        db: &::salsa::Db,
    ) -> LinTemplateArguments {
        template_arguments
            .iter()
            .map(|&template_argument| {
                LinTemplateArgument::from_hir(template_argument, linkage_instantiation, db)
            })
            .collect()
    }

    pub(crate) fn from_hir(
        arg: HirTemplateArgument,
        linkage_instantiation: Option<&LinkageInstantiation>,
        db: &::salsa::Db,
    ) -> Self {
        match arg {
            HirTemplateArgument::Vacant => LinTemplateArgument::Vacant,
            HirTemplateArgument::Type(hir_ty) => {
                LinTemplateArgument::Type(LinType::from_hir(hir_ty, linkage_instantiation, db))
            }
            HirTemplateArgument::Constant(hir_constant) => LinTemplateArgument::Constant(
                LinConstant::from_hir(hir_constant, linkage_instantiation),
            ),
            HirTemplateArgument::Lifetime(_) => LinTemplateArgument::Lifetime,
            HirTemplateArgument::Place(_) => LinTemplateArgument::Place(todo!()),
        }
    }

    pub(crate) fn from_javelin(
        arg: JavTemplateArgument,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match arg {
            JavTemplateArgument::Vacant => todo!(),
            JavTemplateArgument::Type(javelin_ty) => LinTemplateArgument::Type(
                LinType::from_javelin(javelin_ty, linkage_instantiation, db),
            ),
            JavTemplateArgument::Constant(constant) => {
                LinTemplateArgument::Constant(LinConstant(constant))
            }
            JavTemplateArgument::Lifetime => todo!(),
            JavTemplateArgument::Place => todo!(),
        }
    }
}

pub type LinTemplateArguments = smallvec::SmallVec<[LinTemplateArgument; 2]>;

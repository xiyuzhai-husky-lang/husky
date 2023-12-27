pub mod constant;
pub mod place;
pub mod ty;

use self::{constant::LinkageConstant, ty::*};
use super::*;
use crate::{instantiation::LinkageInstantiation, template_argument::place::LinkagePlace};
use husky_hir_ty::HirTemplateArgument;
use husky_javelin::template_argument::{constant::JavelinConstant, JavelinTemplateArgument};

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinkageTemplateArgument {
    Vacant,
    Type(LinkageType),
    Constant(LinkageConstant),
    Lifetime,
    Place(LinkagePlace),
}

impl LinkageTemplateArgument {
    pub(crate) fn from_hir_template_arguments(
        template_arguments: &[HirTemplateArgument],
        linkage_instantiation: Option<&LinkageInstantiation>,
        db: &::salsa::Db,
    ) -> LinkageTemplateArguments {
        template_arguments
            .iter()
            .map(|&template_argument| {
                LinkageTemplateArgument::from_hir(template_argument, linkage_instantiation, db)
            })
            .collect()
    }

    pub(crate) fn from_hir(
        arg: HirTemplateArgument,
        linkage_instantiation: Option<&LinkageInstantiation>,
        db: &::salsa::Db,
    ) -> Self {
        match arg {
            HirTemplateArgument::Vacant => LinkageTemplateArgument::Vacant,
            HirTemplateArgument::Type(hir_ty) => LinkageTemplateArgument::Type(
                LinkageType::from_hir(hir_ty, linkage_instantiation, db),
            ),
            HirTemplateArgument::Constant(hir_constant) => LinkageTemplateArgument::Constant(
                LinkageConstant::from_hir(hir_constant, linkage_instantiation),
            ),
            HirTemplateArgument::Lifetime(_) => LinkageTemplateArgument::Lifetime,
            HirTemplateArgument::Place(_) => LinkageTemplateArgument::Place(todo!()),
        }
    }

    pub(crate) fn from_javelin(
        arg: JavelinTemplateArgument,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match arg {
            JavelinTemplateArgument::Vacant => todo!(),
            JavelinTemplateArgument::Type(javelin_ty) => LinkageTemplateArgument::Type(
                LinkageType::from_javelin(javelin_ty, linkage_instantiation, db),
            ),
            JavelinTemplateArgument::Constant(constant) => {
                LinkageTemplateArgument::Constant(LinkageConstant(constant))
            }
            JavelinTemplateArgument::Lifetime => todo!(),
            JavelinTemplateArgument::Place => todo!(),
        }
    }
}

pub type LinkageTemplateArguments = smallvec::SmallVec<[LinkageTemplateArgument; 2]>;

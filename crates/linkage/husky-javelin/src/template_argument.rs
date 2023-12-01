pub mod constant;
pub mod ty;

use self::constant::*;
use self::ty::*;
use super::*;
use crate::instantiation::JavelinInstantiation;
use husky_hir_ty::HirTemplateArgument;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum JavelinTemplateArgument {
    Vacant,
    Type(JavelinType),
    Constant(JavelinConstant),
    Lifetime,
    Place,
}

impl JavelinTemplateArgument {
    pub(crate) fn from_hir_template_arguments(
        template_arguments: &[HirTemplateArgument],
        javelin_instantiation: Option<&JavelinInstantiation>,
        db: &::salsa::Db,
    ) -> JavelinTemplateArguments {
        template_arguments
            .iter()
            .map(|&template_argument| {
                JavelinTemplateArgument::from_hir(template_argument, javelin_instantiation, db)
            })
            .collect()
    }

    pub(crate) fn from_hir(
        template_argument: HirTemplateArgument,
        javelin_instantiation: Option<&JavelinInstantiation>,
        db: &::salsa::Db,
    ) -> Self {
        match template_argument {
            HirTemplateArgument::Vacant => JavelinTemplateArgument::Vacant,
            HirTemplateArgument::Type(hir_ty) => JavelinTemplateArgument::Type(
                JavelinType::from_hir(hir_ty, javelin_instantiation, db),
            ),
            HirTemplateArgument::Constant(hir_constant) => JavelinTemplateArgument::Constant(
                JavelinConstant::from_hir(hir_constant, javelin_instantiation),
            ),
            HirTemplateArgument::Lifetime(_) => JavelinTemplateArgument::Lifetime,
            HirTemplateArgument::Place(_) => JavelinTemplateArgument::Place,
        }
    }
}

pub type JavelinTemplateArguments = smallvec::SmallVec<[JavelinTemplateArgument; 2]>;

pub mod constant;
pub mod ty;

use self::constant::*;
use self::ty::*;
use super::*;
use husky_hir_ty::HirTemplateArgument;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinkageTemplateArgument {
    Vacant,
    Type(LinkageType),
    Constant(LinkageConstant),
    Lifetime,
    Place,
}

impl LinkageTemplateArgument {
    pub(crate) fn from_hir_template_arguments(
        template_arguments: &[HirTemplateArgument],
        db: &dyn LinkageDb,
    ) -> LinkageTemplateArguments {
        template_arguments
            .iter()
            .map(|&template_argument| LinkageTemplateArgument::from_hir(template_argument, db))
            .collect()
    }

    pub(crate) fn from_hir(template_argument: HirTemplateArgument, db: &dyn LinkageDb) -> Self {
        match template_argument {
            HirTemplateArgument::Vacant => LinkageTemplateArgument::Vacant,
            HirTemplateArgument::Type(hir_ty) => {
                LinkageTemplateArgument::Type(LinkageType::from_hir(hir_ty, db))
            }
            HirTemplateArgument::Constant(hir_constant) => {
                LinkageTemplateArgument::Constant(LinkageConstant::from_hir(hir_constant))
            }
            HirTemplateArgument::Lifetime(_) => LinkageTemplateArgument::Lifetime,
            HirTemplateArgument::Place(_) => LinkageTemplateArgument::Place,
        }
    }
}

pub type LinkageTemplateArguments = smallvec::SmallVec<[LinkageTemplateArgument; 2]>;

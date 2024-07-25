use crate::*;
use husky_ki_repr::repr::KiCachingClass;

#[derive(Default)]
pub struct DevRuntimeCommonConfig {}

pub struct DevRuntimeConfig<Devsoul: IsDevsoul> {
    common: DevRuntimeCommonConfig,
    specifc: Devsoul::RuntimeSpecificConfig,
}

impl<Devsoul: IsDevsoul> Default for DevRuntimeConfig<Devsoul> {
    fn default() -> Self {
        Self {
            common: Default::default(),
            specifc: Default::default(),
        }
    }
}

impl<Devsoul: IsDevsoul> DevRuntimeConfig<Devsoul> {
    pub(crate) fn needs_caching(&self, caching_class: KiCachingClass) -> bool {
        // ad hoc
        match caching_class {
            KiCachingClass::StaticVar => false,
            // this is by design, vals are cached through a different route.
            // the val linket will calculate the pedestal and ask to cache in a val specific place
            // probably needs more testing on this
            KiCachingClass::Val => false,
            // this should be carefully designed
            // although variables should be cached due to laziness,
            // we should think of some cache invalidation strategy to keep things tight.
            KiCachingClass::Variable => true,
            KiCachingClass::Expr => false,
            KiCachingClass::Stmt => false,
            KiCachingClass::Condition => false,
            KiCachingClass::RuntimeConstant => false,
        }
    }
}

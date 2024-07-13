use crate::*;

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

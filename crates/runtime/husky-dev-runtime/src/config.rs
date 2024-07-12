use crate::*;

#[derive(Default)]
pub struct DevRuntimeCommonConfig {}

pub struct DevRuntimeConfig<Devend: IsDevend> {
    common: DevRuntimeCommonConfig,
    specifc: Devend::RuntimeSpecificConfig,
}

impl<Devend: IsDevend> Default for DevRuntimeConfig<Devend> {
    fn default() -> Self {
        Self {
            common: Default::default(),
            specifc: Default::default(),
        }
    }
}

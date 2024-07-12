use crate::*;

#[derive(Default)]
pub struct DevRuntimeCommonConfig {}

pub struct DevRuntimeConfig<DevAscension: IsDevAscension> {
    common: DevRuntimeCommonConfig,
    specifc: DevAscension::RuntimeSpecificConfig,
}

impl<DevAscension: IsDevAscension> Default for DevRuntimeConfig<DevAscension> {
    fn default() -> Self {
        Self {
            common: Default::default(),
            specifc: Default::default(),
        }
    }
}

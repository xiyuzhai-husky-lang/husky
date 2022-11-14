use crate::*;

#[derive(Subcommand)]
pub(crate) enum HuskyDemoTarget {
    /// demonstrate example for mnist classification
    Mnist,
    /// demonstrate example for imagenet classification
    Imagenet,
    /// demonstrate example for a simple clock
    SimpleClock,
}

pub(crate) fn demo(target: HuskyDemoTarget) {
    match target {
        HuskyDemoTarget::Mnist => todo!(),
        HuskyDemoTarget::Imagenet => todo!(),
        HuskyDemoTarget::SimpleClock => todo!(),
    }
}

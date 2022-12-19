use crate::{Toolchain, ToolchainChannel, ToolchainDate, ToolchainJar};
use husky_platform::Platform;
use salsa::{storage::HasJar, DbWithJar};

pub trait ToolchainDb: DbWithJar<ToolchainJar> {
    fn toolchain(&self) -> Toolchain;
    fn toolchain_jar(&self) -> &ToolchainJar;
}

impl<T> ToolchainDb for T
where
    T: DbWithJar<ToolchainJar>,
{
    fn toolchain(&self) -> Toolchain {
        todo!()
        // *self.toolchain_jar().toolchain_cell().get_or_init(|| {
        //     // ad hoc
        //     Toolchain::new(
        //         self,
        //         ToolchainChannel::new_ad_hoc(),
        //         ToolchainDate::new_ad_hoc(),
        //         Platform::new_ad_hoc(),
        //     )
        // })
    }

    fn toolchain_jar(&self) -> &ToolchainJar {
        &<Self as HasJar<ToolchainJar>>::jar(self).0
    }
}

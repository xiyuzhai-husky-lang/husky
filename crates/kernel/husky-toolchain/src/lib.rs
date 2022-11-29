mod date;
mod db;
mod jar;

pub use db::*;
pub use jar::*;

use date::*;
use husky_platform::Platform;

#[salsa::input(jar = ToolchainJar)]
pub struct Toolchain {
    channel: ToolchainChannel,
    date: ToolchainDate,
    platform: Platform,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ToolchainChannel {
    Nightly,
    Stable,
}

impl ToolchainChannel {
    pub fn new_ad_hoc() -> Self {
        ToolchainChannel::Nightly
    }
}

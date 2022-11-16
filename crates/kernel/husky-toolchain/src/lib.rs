mod date;

use date::*;
use husky_platform::Platform;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Toolchain {
    channel: Channel,
    date: Date,
    platform: Platform,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Channel {
    Nightly,
    Stable,
}

impl Toolchain {
    pub fn new_ad_hoc() -> Self {
        Self {
            channel: Channel::Stable,
            date: Date::new_ad_hoc(),
            platform: Platform::new_ad_hoc(),
        }
    }
}

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

pub struct Toolchain {
    channel: Channel,
}

pub enum Channel {
    Nightly,
    Stable,
}

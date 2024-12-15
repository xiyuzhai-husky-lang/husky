pub trait Attach: Copy + Send + Sync {
    fn attach<R>(self, f: impl FnOnce() -> R) -> R;
}

impl Attach for () {
    fn attach<R>(self, f: impl FnOnce() -> R) -> R {
        f()
    }
}

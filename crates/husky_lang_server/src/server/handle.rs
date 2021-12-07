// Enforces drop order
pub(crate) struct Handle<H, C> {
    pub(crate) handle: H,
    pub(crate) _receiver: C,
}

impl<H, C> Handle<H, C> {}

pub trait WithFmtContext {
    fn with_fmt_context(
        &self,
        f: impl FnOnce() -> ::std::fmt::Result,
        db: &crate::Db,
    ) -> ::std::fmt::Result;
}

impl<T> WithFmtContext for &T
where
    T: WithFmtContext,
{
    fn with_fmt_context(
        &self,
        f: impl FnOnce() -> std::fmt::Result,
        db: &crate::Db,
    ) -> std::fmt::Result {
        <T as WithFmtContext>::with_fmt_context(self, f, db)
    }
}

pub struct WithFmtContextTest<T>(pub T);

impl<T> WithFmtContextTest<T>
where
    T: WithFmtContext,
{
    pub fn with_fmt_context(
        &self,
        f: impl FnOnce() -> ::std::fmt::Result,
        db: &crate::Db,
    ) -> ::std::fmt::Result {
        self.0.with_fmt_context(f, db)
    }
}

// fallback
impl<T> WithFmtContext for WithFmtContextTest<T> {
    fn with_fmt_context(
        &self,
        f: impl FnOnce() -> ::std::fmt::Result,
        db: &crate::Db,
    ) -> ::std::fmt::Result {
        f()
    }
}

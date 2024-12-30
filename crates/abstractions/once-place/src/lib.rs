#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OncePlace<T>(Option<T>);

impl<T> Default for OncePlace<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> OncePlace<T> {
    pub fn set(&mut self, value: T) -> Self {
        debug_assert!(self.0.is_none());
        Self(Some(value))
    }
}

impl<T> std::ops::Deref for OncePlace<T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("OncePlace is not set")
    }
}

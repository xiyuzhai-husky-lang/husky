#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OncePlace<T>(Option<T>);

impl<T> Default for OncePlace<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> OncePlace<T> {
    pub fn get(&self) -> Option<&T> {
        self.0.as_ref()
    }
}

impl<T> OncePlace<T> {
    pub fn set(&mut self, value: T) {
        debug_assert!(self.0.is_none());
        self.0 = Some(value);
    }
}

impl<T> std::ops::Deref for OncePlace<T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("OncePlace is not set")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let place: OncePlace<i32> = OncePlace::default();
        assert!(place.0.is_none());
    }

    #[test]
    fn test_set_and_deref() {
        let mut place: OncePlace<i32> = OncePlace::default();
        place.set(42);
        assert_eq!(*place, 42);
    }

    #[test]
    #[should_panic(expected = "OncePlace is not set")]
    fn test_deref_unset() {
        let place: OncePlace<i32> = OncePlace::default();
        let _value = *place; // Should panic
    }

    #[test]
    fn test_clone() {
        let mut place: OncePlace<i32> = OncePlace::default();
        place.set(42);
        let cloned = place.clone();
        assert_eq!(*cloned, 42);
    }
}

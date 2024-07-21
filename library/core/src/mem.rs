#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Leash<T>(pub &'static T)
where
    T: 'static;

impl<T> std::hash::Hash for Leash<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const T).hash(state);
    }
}

impl<T> Leash<T>
where
    T: 'static,
{
    pub fn new(t: &'static T) -> Self {
        Self(t)
    }
}

impl<T> Leash<T>
where
    T: 'static,
{
    pub fn deleash(self) -> &'static T {
        self.0
    }
}

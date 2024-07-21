#[derive(Debug, PartialEq, Eq)]
pub struct Leash<T>(pub &'static T)
where
    T: ?Sized + 'static;

impl<T> Clone for Leash<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for Leash<T> {}

impl<T> std::hash::Hash for Leash<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const T).hash(state);
    }
}

impl<T> Leash<T>
where
    T: ?Sized + 'static,
{
    pub fn new(t: &'static T) -> Self {
        Self(t)
    }
}

impl<T> Leash<T>
where
    T: ?Sized + 'static,
{
    pub fn deleash(self) -> &'static T {
        self.0
    }
}

// ad hoc, should use Leash<[T]>
impl<T> Leash<Vec<T>>
where
    T: 'static,
{
    pub fn collect_leashes(self) -> Vec<Leash<T>> {
        self.0.iter().map(Leash).collect()
    }
}

impl<T> Leash<[T]>
where
    T: 'static,
{
    pub fn collect_leashes(self) -> Vec<Leash<T>> {
        self.0.iter().map(Leash).collect()
    }
}

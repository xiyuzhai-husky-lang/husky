#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time(usize);

#[derive(Debug)]
pub struct TimeDb<T>(Vec<(Time, T)>);

impl<T> TimeDb<T> {
    pub(crate) fn new_uninitialized() -> Self {
        Self(Default::default())
    }
}

impl<T> TimeDb<T> {
    pub fn now(&self) -> Option<&T> {
        self.0.last().map(|(_, t)| t)
    }
}

#[derive(Debug, Default)]
pub struct Timer {
    current: usize,
}

impl Timer {
    pub fn pass(&mut self) {
        self.current += 1
    }

    pub(crate) fn set<T>(&self, db: &mut TimeDb<T>, t: T) {
        db.0.push((self.current(), t))
    }

    pub(crate) fn new_db<T>(&self) -> TimeDb<T>
    where
        T: Default,
    {
        TimeDb(vec![(self.current(), Default::default())])
    }

    fn current(&self) -> Time {
        Time(self.current)
    }
}

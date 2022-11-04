#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time(usize);

#[derive(Debug)]
pub struct TimeDb<T>(Vec<(Time, T)>);

impl<T> TimeDb<T> {
    pub fn now(&self) -> &T {
        &self.0.last().unwrap().1
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

    pub(crate) fn update<T, E>(
        &self,
        db: &mut TimeDb<T>,
        f: impl Fn(&T) -> Result<T, E>,
    ) -> Result<(), E> {
        let t = f(db.now())?;
        Ok(db.0.push((self.current(), t)))
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

use bimap::BiMap;
use std::sync::{Mutex, RwLock};
// todo: this needs some heavy improvement
use dashmap::DashMap;

pub struct BiDashMap<L, R>
where
    L: Eq + std::hash::Hash,
    R: Eq + std::hash::Hash,
{
    inner: Mutex<BiMap<L, R>>,
}

impl<L, R> BiDashMap<L, R>
where
    L: Eq + std::hash::Hash,
    R: Eq + std::hash::Hash,
{
    pub fn get_right_or_insert_with_result<E>(
        &self,
        left: L,
        gen_right: impl FnOnce() -> Result<R, E>,
    ) -> Result<R, E>
    where
        R: Clone,
    {
        let mut inner = self.inner.lock().unwrap();
        if let Some(right) = inner.get_by_left(&left) {
            Ok(right.clone())
        } else {
            let right = gen_right()?;
            inner.insert(left, right.clone());
            Ok(right)
        }
    }
}

impl<L, R> Default for BiDashMap<L, R>
where
    L: Eq + std::hash::Hash,
    R: Eq + std::hash::Hash,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

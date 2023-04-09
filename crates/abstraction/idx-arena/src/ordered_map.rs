use crate::*;
use husky_check_utils::should;
use std::marker::PhantomData;

/// expect entries to be inserted in order
#[derive(Clone)]
pub struct ArenaOrderedMap<T, V> {
    data: Vec<V>,
    phantom: PhantomData<T>,
}

impl<T, V> Default for ArenaOrderedMap<T, V> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            phantom: Default::default(),
        }
    }
}

impl<T, V> std::ops::Deref for ArenaOrderedMap<T, V> {
    type Target = [V];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, V> ArenaOrderedMap<T, V> {
    pub fn new(arena: &Arena<T>, f: impl Fn(&T) -> V) -> Self {
        Self {
            data: arena.iter().map(f).collect(),
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn insert_next(&mut self, idx: ArenaIdx<T>, v: V) {
        debug_assert_eq!(self.data.len(), idx.raw);
        self.data.push(v)
    }

    #[inline(always)]
    pub unsafe fn insert_next_unchecked(&mut self, v: V) {
        self.data.push(v)
    }
}

impl<T, V> std::ops::Index<ArenaIdx<T>> for ArenaOrderedMap<T, V> {
    type Output = V;

    fn index(&self, index: ArenaIdx<T>) -> &Self::Output {
        &self.data[index.raw]
    }
}

impl<Db: ?Sized, T, V> salsa::DebugWithDb<Db> for ArenaOrderedMap<T, V>
where
    T: salsa::DebugWithDb<Db>,
    V: salsa::DebugWithDb<Db>,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let elements = self.data.iter().map(|v| v.debug_with(db, level.next()));
        f.debug_list().entries(elements).finish()
    }
}

impl<T, V> PartialEq for ArenaOrderedMap<T, V>
where
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T, V> Eq for ArenaOrderedMap<T, V> where V: Eq {}

impl<T, V> std::fmt::Debug for ArenaOrderedMap<T, V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArenaMap")
            .field("data", &self.data)
            .finish()
    }
}

use crate::*;

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
    pub fn new_with(arena: &Arena<T>, f: impl Fn(&T) -> V) -> Self {
        Self {
            data: arena.iter().map(f).collect(),
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn insert_next(&mut self, idx: ArenaIdx<T>, v: V) {
        debug_assert_eq!(self.data.len(), idx.index());
        self.data.push(v)
    }

    #[inline(always)]
    pub unsafe fn insert_next_unchecked(&mut self, v: V) {
        self.data.push(v)
    }

    pub fn insert_next_batch(
        &mut self,
        items: impl IntoIterator<Item = ArenaIdx<T>>,
        comments: impl IntoIterator<Item = V>,
    ) {
        for (idx, comment) in items.into_iter().zip(comments) {
            self.insert_next(idx, comment);
        }
    }
}

impl<T, V> std::ops::Index<ArenaIdx<T>> for ArenaOrderedMap<T, V> {
    type Output = V;

    fn index(&self, index: ArenaIdx<T>) -> &Self::Output {
        &self.data[index.index()]
    }
}

impl<T, V> std::ops::IndexMut<ArenaIdx<T>> for ArenaOrderedMap<T, V> {
    fn index_mut(&mut self, index: ArenaIdx<T>) -> &mut Self::Output {
        &mut self.data[index.index()]
    }
}

impl<T, V> salsa::DebugWithDb for ArenaOrderedMap<T, V>
where
    T: salsa::DebugWithDb,
    V: salsa::DebugWithDb,
{
    fn debug_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        let elements = self.data.iter().map(|v| v.debug_with(db));
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

pub trait MapCollect<T, S, V>
where
    V: FromIterator<S>,
{
    fn map(&self, f: impl FnMut(&T) -> S) -> V;
    // fn map_into<S>(self, f: impl FnMut(T) -> S) -> Vec<S>;
}

impl<T, S, V> MapCollect<T, S, V> for Vec<T>
where
    V: FromIterator<S>,
{
    fn map(&self, f: impl FnMut(&T) -> S) -> V {
        self.iter().map(f).collect()
    }
    // fn map_into<S>(self, f: impl FnMut(T) -> S) -> Vec<S> {
    //     self.into_iter().map(f).collect()
    // }
}

impl<T, S, V> MapCollect<T, S, V> for [T]
where
    V: FromIterator<S>,
{
    fn map(&self, f: impl FnMut(&T) -> S) -> V {
        self.iter().map(f).collect()
    }
    // fn map_into<S>(self, f: impl FnMut(T) -> S) -> Vec<S> {
    //     self.into_iter().map(f).collect()
    // }
}

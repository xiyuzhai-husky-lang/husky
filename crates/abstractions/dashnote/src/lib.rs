use dashmap::DashMap;

pub struct DashNote<K, V>
where
    K: std::hash::Hash + Eq + Copy,
{
    map: DashMap<K, V>,
}

impl<K, V> Default for DashNote<K, V>
where
    K: std::hash::Hash + Eq + Copy,
{
    fn default() -> Self {
        Self {
            map: DashMap::default(),
        }
    }
}

impl<K, V> DashNote<K, V>
where
    K: std::hash::Hash + Eq + Copy,
{
    pub fn get_or_calc(&self, key: K, f: impl FnOnce() -> V) -> V
    where
        V: Clone,
    {
        self.map.entry(key).or_insert_with(f).value().clone()
    }
}

pub struct DashRefNote<K, V> {
    map: DashMap<K, Box<V>>,
}

impl<K, V> Default for DashRefNote<K, V>
where
    K: std::hash::Hash + Eq + Copy,
{
    fn default() -> Self {
        Self {
            map: DashMap::default(),
        }
    }
}

impl<K, V> DashRefNote<K, V>
where
    K: std::hash::Hash + Eq + Copy,
{
    pub fn get_or_calc(&self, key: K, f: impl FnOnce() -> V) -> &V {
        use husky_wild_utils::arb_ref;

        let entry = self.map.entry(key).or_insert_with(|| Box::new(f()));
        let v: &V = &**entry.value();
        unsafe { arb_ref(v) }
    }
}

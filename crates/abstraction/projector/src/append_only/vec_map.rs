use vec_like::{AppendOnlyVecMap, VecMapEntry};

pub struct AppendOnlyVecMapProjector<K, V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: VecMapEntry<K> + Projector,
{
    data: AppendOnlyVecMap<K, V>,
    old_len: usize,
}

impl<K, V> Projector for AppendOnlyVecMapProjector<K, V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: VecMapEntry<K>,
{
}

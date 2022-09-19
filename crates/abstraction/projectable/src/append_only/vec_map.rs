use vec_like::{AppendOnlyVecMap, VecMapEntry};

pub struct ProjAppendOnlyVecMap<K, V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: VecMapEntry<K> + Projector,
{
    data: AppendOnlyVecMap<K, V>,
    old_len: usize,
}

impl<K, V> Projectable for AppendOnlyVecMapProjector<K, V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: VecMapEntry<K>,
{
}

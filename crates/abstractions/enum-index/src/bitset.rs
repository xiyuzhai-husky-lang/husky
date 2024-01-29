use crate::IsEnumIndex;

/// (I::N + 7) / 8 is always equal to ((I::N as f32)/8.0).ceil()
#[derive(Clone, Copy)]
pub struct EnumBitSet<I: IsEnumIndex>
where
    [(); (I::N + 7) / 8]:,
{
    data: [u8; (I::N + 7) / 8],
}

impl<I: IsEnumIndex> std::fmt::Debug for EnumBitSet<I>
where
    [(); (I::N + 7) / 8]:,
    I: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_vec().fmt(f)
    }
}

impl<I: IsEnumIndex> Default for EnumBitSet<I>
where
    [u8; (I::N + 7) / 8]: Default,
{
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<I: IsEnumIndex> EnumBitSet<I>
where
    [(); (I::N + 7) / 8]:,
{
    pub fn new(elems: impl IntoIterator<Item = I>) -> Self
    where
        [u8; (I::N + 7) / 8]: Default,
    {
        let mut slf = Self::default();
        for elem in elems {
            slf.insert(elem)
        }
        slf
    }

    pub fn insert(&mut self, elem: I) {
        let index = elem.index();
        let i = index / 8;
        let j = index % 8;
        self.data[i] |= 1 << j;
    }

    pub fn remove(&mut self, elem: I) {
        let index = elem.index();
        let i = index / 8;
        let j = index % 8;
        self.data[i] &= !(1u8 << j);
    }

    pub fn toggle(&mut self, elem: I) {
        let index = elem.index();
        let i = index / 8;
        let j = index % 8;
        self.data[i] ^= 1u8 << j;
    }

    pub fn contains(self, elem: I) -> bool {
        let index = elem.index();
        let i = index / 8;
        let j = index % 8;
        ((self.data[i] >> j) & 1) != 0
    }

    pub fn iter(self) -> impl Iterator<Item = I> {
        (0..I::N).into_iter().filter_map(move |index| {
            let elem = I::from_index(index);
            self.contains(elem).then_some(elem)
        })
    }

    pub fn to_vec(&self) -> Vec<I> {
        self.iter().collect::<Vec<_>>()
    }
}

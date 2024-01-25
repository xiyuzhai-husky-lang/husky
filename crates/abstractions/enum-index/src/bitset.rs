use crate::IsEnumIndex;

/// (I::N + 7) / 8 is always equal to ((I::N as f32)/8.0).ceil()
#[derive(Debug, Clone, Copy)]
pub struct EnumBitSet<I: IsEnumIndex>
where
    [(); (I::N + 7) / 8]:,
{
    data: [u8; (I::N + 7) / 8],
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
    pub fn new() -> Self
    where
        [u8; (I::N + 7) / 8]: Default,
    {
        Self::default()
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

    pub fn to_vec(self) -> Vec<I> {
        (0..I::N)
            .into_iter()
            .filter_map(|index| {
                let elem = I::from_index(index);
                self.contains(elem).then_some(elem)
            })
            .collect()
    }
}

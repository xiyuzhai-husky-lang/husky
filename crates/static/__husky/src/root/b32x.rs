pub trait __B32X {
    fn ctz(self) -> i32;
    fn clz(self) -> i32;
    fn last_bits(self, n: i32) -> u32;
}

impl __B32X for u32 {
    fn ctz(self) -> i32 {
        self.trailing_zeros() as i32
    }

    fn clz(self) -> i32 {
        self.leading_zeros() as i32
    }
    #[inline(always)]
    fn last_bits(self, n: i32) -> u32 {
        self & !(u32::MAX << n)
    }
}

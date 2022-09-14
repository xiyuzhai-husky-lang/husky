pub trait __B32X {
    fn co(self) -> i32;
    fn ctz(self) -> i32;
    fn clz(self) -> i32;
    fn last_bits(self, n: i32) -> u32;
    fn span(self) -> i32;
}

impl __B32X for u32 {
    fn co(self) -> i32 {
        self.count_ones() as i32
    }

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

    #[inline(always)]
    fn span(self) -> i32 {
        if self == 0 {
            return 0;
        }
        32 - (self.clz() + self.ctz())
    }
}

#[test]
fn test_span() {
    assert_eq!(1.span(), 1);
    assert_eq!(2.span(), 1);
    assert_eq!(3.span(), 2);
    assert_eq!(6.span(), 2);
    assert_eq!(0.span(), 0)
}

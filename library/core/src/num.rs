pub trait __U32X: Copy {
    fn span(self) -> i32;

    /// count leading zeros
    fn clz(self) -> i32;

    /// count trailing zeros
    fn ctz(self) -> i32;

    /// count ones
    fn co(self) -> i32;

    fn right_mass(self) -> i32;

    fn last_bits(self, k: i32) -> u32;
}

impl __U32X for u32 {
    fn span(self) -> i32 {
        if self == 0 {
            return 0;
        }
        32 - (self.clz() + self.ctz())
    }

    fn clz(self) -> i32 {
        self.leading_zeros() as i32
    }

    /// count trailing zeros
    fn ctz(self) -> i32 {
        self.trailing_zeros() as i32
    }

    /// count ones
    fn co(self) -> i32 {
        todo!()
    }

    fn right_mass(self) -> i32 {
        todo!()
    }

    // make except the last `k` bits
    fn last_bits(self, k: i32) -> u32 {
        self & !(u32::MAX << k)
    }
}

pub trait __F32X: Copy {
    fn sgnx(self) -> i32;
}

impl __F32X for f32 {
    fn sgnx(self) -> i32 {
        if self > 0.0 {
            1
        } else if self < 0.0 {
            -1
        } else {
            0
        }
    }
}

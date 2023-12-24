pub trait __U32X: Copy {
    fn span(self) -> i32;

    /// count trailing zeros
    fn ctz(self) -> i32;

    /// count ones
    fn co(self) -> i32;

    fn right_mass(self) -> i32;

    fn last_bits(self, k: i32) -> u32;
}

impl __U32X for u32 {
    fn span(self) -> i32 {
        todo!()
    }

    /// count trailing zeros
    fn ctz(self) -> i32 {
        self.trailing_zeros().try_into().unwrap()
    }

    /// count ones
    fn co(self) -> i32 {
        todo!()
    }

    fn right_mass(self) -> i32 {
        todo!()
    }

    fn last_bits(self, k: i32) -> u32 {
        todo!()
    }
}

pub trait __F32X: Copy {
    fn sgnx(self) -> i32;
}

impl __F32X for f32 {
    fn sgnx(self) -> i32 {
        todo!()
    }
}

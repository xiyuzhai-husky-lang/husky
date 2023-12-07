pub trait __U32X: Copy {
    fn span(self) -> i32;

    /// count trailing zeros
    fn ctz(self) -> i32;

    /// count ones
    fn co(self) -> i32;
}

impl __U32X for u32 {
    fn span(self) -> i32 {
        todo!()
    }

    /// count trailing zeros
    fn ctz(self) -> i32 {
        todo!()
    }

    /// count ones
    fn co(self) -> i32 {
        todo!()
    }
}

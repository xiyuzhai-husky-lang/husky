pub trait __F32X {
    fn sgnx(self) -> i32;
}

impl __F32X for f32 {
    fn sgnx(self) -> i32 {
        {
            if self > 0. {
                1
            } else if self == 0. {
                0
            } else {
                -1
            }
        }
    }
}

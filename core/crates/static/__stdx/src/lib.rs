pub trait __F32X: Copy {
    fn this(self) -> f32;
    fn sgnx(self) -> i32 {
        let this = self.this();
        if this > 0. {
            1
        } else if this == 0. {
            0
        } else {
            -1
        }
    }
}

impl __F32X for f32 {
    fn this(self) -> f32 {
        self
    }
}

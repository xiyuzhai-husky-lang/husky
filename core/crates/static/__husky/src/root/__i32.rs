pub trait __I32X {
    fn sgn(self) -> i32;
}

impl __I32X for i32 {
    fn sgn(self) -> i32 {
        if self > 0 {
            1
        } else if self == 0 {
            0
        } else {
            -1
        }
    }
}

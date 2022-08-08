use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VMStackIdx(u8);

impl Add<u8> for VMStackIdx {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        VMStackIdx(self.0 + rhs)
    }
}

impl Add<usize> for VMStackIdx {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        let rhs: u8 = rhs.try_into().unwrap();
        VMStackIdx(self.0 + rhs)
    }
}

impl std::fmt::Debug for VMStackIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("StackIdx({})", self.0))
    }
}

impl VMStackIdx {
    pub fn this() -> VMStackIdx {
        Self(0)
    }

    pub fn new(raw: usize) -> VMStackIdx {
        let raw: u8 = raw.try_into().unwrap();
        VMStackIdx(raw)
    }

    pub fn raw(&self) -> usize {
        self.0 as usize
    }
}

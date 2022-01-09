#[derive(Debug, Copy, Clone)]
pub enum Literal {
    I32Literal(i32),
    F32Literal(f32),
}

impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Self::I32Literal(i)
    }
}

impl From<f32> for Literal {
    fn from(f: f32) -> Self {
        Self::F32Literal(f)
    }
}

impl From<&i32> for Literal {
    fn from(i: &i32) -> Self {
        Self::I32Literal(*i)
    }
}

impl From<&f32> for Literal {
    fn from(f: &f32) -> Self {
        Self::F32Literal(*f)
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::I32Literal(l0), Self::I32Literal(r0)) => l0 == r0,
            (Self::F32Literal(l0), Self::F32Literal(r0)) => l0.to_bits() == r0.to_bits(),
            _ => false,
        }
    }
}

impl Eq for Literal {}

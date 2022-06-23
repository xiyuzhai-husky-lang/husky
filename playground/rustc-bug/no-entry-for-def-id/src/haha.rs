pub struct A {
    pub items: &'static [&'static A],
}

pub static A1: &A = &A { items: &[] };
pub static A2: A = A { items: &[A1] };

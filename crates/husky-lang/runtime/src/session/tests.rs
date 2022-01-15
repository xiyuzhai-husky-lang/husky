use common::*;

#[test]
fn memb_access() {
    let a: A = A { x: 0 };
    let ra: *const dyn Printable = &a;
    let rx: *const dyn Printable = &a.x;
    p!(std::mem::size_of::<*const dyn Printable>());
    unsafe {
        p!((*rx).print());
    }
}

pub trait Printable {
    fn print(&self) -> String;
}

impl Printable for i32 {
    fn print(&self) -> String {
        "i32".into()
    }
}

struct A {
    x: i32,
}

impl Printable for A {
    fn print(&self) -> String {
        "A".into()
    }
}

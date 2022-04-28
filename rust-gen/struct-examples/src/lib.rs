pub struct A {
    pub(crate) x: i32,
}

impl A {
    pub(crate) fn get_x(&self) -> i32 {
        self.x
    }
}

pub(crate) fn f1() -> crate::A {
    crate::A::__call__(1i32)
}

pub(crate) fn f2() -> crate::A {
    let mut a = crate::A::__call__(2i32);
    a.x = 1i32;
    a
}

pub(crate) fn f3() -> void {
    let a = crate::A::__call__(2i32);
    assert!(a.get_x() == 2i32);
}

pub(crate) fn g1() -> i32 {
    let a = crate::A::__call__(2i32);

    a.x
}

pub mod __init__ {
    pub fn link_entity_with_compiled(compile_time: &mut compile_time_db::HuskyLangCompileTime) {
        todo!()
    }
}

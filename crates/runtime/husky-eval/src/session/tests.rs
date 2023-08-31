// #[cfg(test)]
// use husky_print_utils::test_print;

// #[test]
// fn field_access() {
//     let a: A = A { x: 0 };
//     let rx: *const dyn Printable = &a.x;
//     test_print!(std::mem::size_of::<*const dyn Printable>());
//     unsafe {
//         test_print!((*rx).print());
//     }
// }

// pub trait Printable {
//     fn print(&self) -> String;
// }

// impl Printable for i32 {
//     fn print(&self) -> String {
//         "i32".into()
//     }
// }

// pub struct A {
//     pub x: i32,
// }

// impl Printable for A {
//     fn print(&self) -> String {
//         "A".into()
//     }
// }

// pub struct A<T> {
//     value: T,
// }
// impl<T> A<T>
// where
//     T: HasName,
// {
//     fn hello() -> &'static B {
//         static b: B = B { name: T::name() };
//         &b
//     }
// }
// pub trait HasName {
//     fn name() -> &'static str;
// }

// pub struct B {
//     name: &'static str,
// }

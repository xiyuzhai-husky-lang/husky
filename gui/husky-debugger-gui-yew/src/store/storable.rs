use super::*;

pub trait Storable: Clone + 'static + std::fmt::Display + std::fmt::Debug {}

impl Storable for i32 {}

impl Storable for f32 {}
impl Storable for bool {}

impl<T> Storable for Rc<T> where T: 'static + std::fmt::Display + std::fmt::Debug {}

use super::*;

pub trait __RegularIncubatorDyn: std::fmt::Debug + UnwindSafe + RefUnwindSafe + 'static {
    unsafe fn incubate_box_dyn(&mut self) -> Box<dyn __RegularStaticDyn>;
}

impl<T> __RegularIncubatorDyn for T
where
    T: __RegularIncubator,
{
    unsafe fn incubate_box_dyn(&mut self) -> Box<dyn __RegularStaticDyn> {
        todo!()
    }
}

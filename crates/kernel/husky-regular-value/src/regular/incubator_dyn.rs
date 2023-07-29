use super::*;

pub trait __RegularIncubatorDyn: std::fmt::Debug + UnwindSafe + RefUnwindSafe + 'static {
    fn incubate_box(&mut self) -> Box<dyn __RegularStaticDyn>;
    fn incubate_sized_ref(&self) -> *const dyn __RegularStaticDyn;
    fn incubate_sized_mut(&mut self) -> *mut dyn __RegularStaticDyn;
}

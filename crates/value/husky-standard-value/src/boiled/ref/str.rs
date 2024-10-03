use super::*;

impl Boiled for &str {
    type Thawed = (); // ad hoc

    unsafe fn from_thawed(thawed: Self::Thawed) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    unsafe fn into_thawed(self) -> Self::Thawed
    where
        Self: Sized,
    {
        todo!()
    }
}

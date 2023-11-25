pub use enum_class_macros::*;

/// always be zero
/// but occupied a nonempy space
#[derive(Default, Clone, Copy)]
pub struct Room32(u32);

impl std::fmt::Debug for Room32 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Room32")
    }
}

impl PartialEq for Room32 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl std::hash::Hash for Room32 {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {}
}

impl Eq for Room32 {}

mod leash;
pub mod r#mut;
mod option;
mod primitive;
pub mod r#ref;
mod ritchie;
mod str;
mod tuple;
pub mod value;
mod vec;

use husky_decl_macro_utils::{
    for_all_non_unit_tuple_tys, for_all_primitive_tys, for_all_ritchie_tys,
};
use slush::SlushValue;
use smallvec::SmallVec;

use super::*;
use crate::thawed::{Thawed, ThawedDyn};

pub trait Frozen:
    std::fmt::Debug + Clone + RefUnwindSafe + UnwindSafe + Send + Sync + 'static
{
    type Thawed: Thawed;
    type Slush: std::any::Any;

    /// this function gives back the value snapshoted,
    /// together with a stand so that the value is valid if the stand is not dropped.
    ///
    /// Returns None if Slush is trivial to save a call of `Box::new`.
    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed);
}

pub trait FrozenDyn: std::fmt::Debug + Send + Sync {
    /// returns a owned type and the stand it needed
    fn thaw_dyn(&self) -> (Option<SlushValue>, Box<dyn ThawedDyn>);
    fn thaw_ref_dyn(self: Arc<Self>) -> (SlushValue, *const dyn ThawedDyn);
    fn thaw_mut_dyn(&self) -> (SlushValue, *mut dyn ThawedDyn);
}

impl<T> FrozenDyn for T
where
    T: Frozen,
{
    fn thaw_dyn(&self) -> (Option<SlushValue>, Box<dyn ThawedDyn>) {
        let (stand, thawed_self) = self.thaw();
        (
            stand.map(|stand| SlushValue::Box(Box::<T::Slush>::new(stand))),
            Box::<T::Thawed>::new(thawed_self),
        )
    }

    fn thaw_ref_dyn(self: Arc<Self>) -> (SlushValue, *const dyn ThawedDyn) {
        todo!()
        // let slf: *const <Self as Frozen>::Thawed =
        //     unsafe { std::mem::transmute(&*self as *const Self) };
        // (SlushValue::Arc(self), slf)
    }

    fn thaw_mut_dyn(&self) -> (SlushValue, *mut dyn ThawedDyn) {
        todo!()
        // let mut slf = self.clone();
        // let slf_mut: *mut <Self as Frozen>::Thawed =
        //     unsafe { std::mem::transmute(&mut slf as *mut Self) };
        // (SlushValue::Box(Box::new(slf)), slf_mut)
    }
}

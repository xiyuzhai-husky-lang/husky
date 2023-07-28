use husky_feature_protocol::FeatureId;
use interner::Internable;

use crate::*;

pub trait InternFeature {
    fn feature_interner(&self) -> &FeatureInterner;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Val {
    id: FeatureId,
}

impl Val {
    pub unsafe fn from_declarative(raw: usize) -> Self {
        Self {
            id: FeatureId::new(raw),
        }
    }

    pub fn id(self) -> FeatureId {
        self.id
    }
}

impl std::ops::Deref for Val {
    type Target = Feature;

    fn deref(&self) -> &Self::Target {
        unreachable!()
    }
}

impl std::borrow::Borrow<Feature> for Val {
    fn borrow(&self) -> &Feature {
        unreachable!()
    }
}

// impl Interned for FeaturePtr {
//     type Ref = Feature;

//     type Owned = Feature;

//     fn new_interned(id: usize, feature: &'static Self::Ref) -> Self {
//         Self {
//             id: FeatureId::new(id),
//         }
//     }

//     fn new_itr() -> interner::Interner<Self> {
//         FeatureInterner::new(&[])
//     }

//     fn opt_atom_itd(t: &Self::Ref) -> Option<Self> {
//         // can be improved here
//         None
//     }
// }

pub type FeatureInterner = interner::Interner<Feature>;

impl Internable for Feature {
    type Ref<'a> = &'a Feature;

    type Interned = Val;

    fn new_itr() -> interner::Interner<Self> {
        todo!()
    }

    fn try_direct(&self) -> Option<Self::Interned> {
        todo!()
    }

    fn itd_to_borrowed(_itd: Self::Interned) -> Self::Ref {
        todo!()
    }

    fn as_ref<'a>(&'a self) -> Self::Ref<'a> {
        todo!()
    }

    fn new_itd(&'static self, _id: usize) -> Self::Interned {
        todo!()
    }

    fn try_direct_from_ref<'a>(_r: Self::Ref<'a>) -> Option<Self::Interned> {
        todo!()
    }

    unsafe fn cast_to_leash<'a>(_r: Self::Ref<'a>) -> Self::Ref {
        todo!()
    }
}

use std::{borrow::Borrow, fmt::Debug, hash::Hash, marker::PhantomData, ops::Deref};

use optional::{Noned, OptEq};

use crate::Interner;

impl<T, Q> Noned for DefaultItd<T, Q>
where
    T: 'static + ?Sized,
    Q: Sized,
{
    fn is_none(&self) -> bool {
        self.target.is_none()
    }

    fn get_none() -> Self {
        Self {
            target: None,
            phantom: PhantomData,
        }
    }
}

pub struct DefaultItd<T, Q>
where
    T: 'static + ?Sized,
    Q: Sized,
{
    target: Option<&'static T>,
    phantom: PhantomData<Q>,
}

impl<T, Q> DefaultItd<T, Q>
where
    T: 'static + ?Sized,
    Q: Sized,
{
    // pub unsafe fn from_raw(raw: *const c_void) -> DefaultInternedPtr<T, Q> {
    //     let raw = raw as *const T;
    //     let target: Option<&'static T> = Some(&*raw);
    //     Self {
    //         target,
    //         phantom: PhantomData,
    //     }
    // }
    // pub unsafe fn to_raw(self) -> *const c_void {
    //     self.target() as *const T as *const c_void
    // }

    pub fn borrow_static(self) -> &'static T {
        self.target.unwrap()
    }
}

impl<T: 'static + ?Sized, Q> PartialEq for DefaultItd<T, Q> {
    fn eq(&self, other: &Self) -> bool {
        (self.borrow_static() as *const T) == (other.borrow_static() as *const T)
    }
}

impl<T: 'static + ?Sized, Q> Eq for DefaultItd<T, Q> {}

impl<T: 'static + ?Sized, Q> OptEq for DefaultItd<T, Q> {
    fn opt_eq(&self, other: &Self) -> bool {
        match (self.target, other.target) {
            (None, None) => true,
            (Some(self_target), Some(other_target)) => {
                (self_target as *const T) == (other_target as *const T)
            }
            _ => false,
        }
    }
}

impl<T: 'static + ?Sized, Q> Hash for DefaultItd<T, Q> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.target.map(|t| t as *const T)).hash(state);
    }
}

impl<T: 'static + ?Sized, Q> Clone for DefaultItd<T, Q> {
    fn clone(&self) -> Self {
        Self {
            target: self.target,
            phantom: PhantomData,
        }
    }
}

impl<T: 'static + ?Sized, Q> Copy for DefaultItd<T, Q> {}

unsafe impl<T: 'static + ?Sized, Q> std::marker::Sync for DefaultItd<T, Q> {}
unsafe impl<T: 'static + ?Sized, Q> std::marker::Send for DefaultItd<T, Q> {}

impl<T: 'static + ?Sized, Q> std::fmt::Debug for DefaultItd<T, Q>
where
    T: 'static + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.borrow_static()))
    }
}

impl<T: 'static + ?Sized, Q> Deref for DefaultItd<T, Q> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.borrow_static()
    }
}

impl<T: 'static + ?Sized, Q> Borrow<T> for DefaultItd<T, Q> {
    fn borrow(&self) -> &T {
        self.borrow_static()
    }
}

// impl<T, Q> Interned for DefaultItd<T >
// where
//     T: 'static + Debug + Hash + Eq + ?Sized,
//     Q: 'static + Hash + Eq + Send + Sync + Debug + Borrow<T>,
// {
//     type Ref<'a> = &'a T;
//     type Raw = *const T;

//     type Owned = Q;

//     fn new_interned(_: usize, target: Self::Ref<'static>) -> Self {
//         Self {
//             target: Some(target),
//             phantom: PhantomData,
//         }
//     }

//     fn new_itr() -> Interner<Self> {
//         Interner::new_empty()
//     }

//     fn opt_atom_itd<'a>(t: Self::Ref<'a>) -> Option<Self> {
//         None
//     }

//     fn raw(self) -> Self::Raw {
//         self.target.unwrap()
//     }
// }

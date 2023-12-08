use crate::builder::{RustKeyword, RustPunctuation, TranspileToRustWith};
use husky_hir_eager_expr::HirEagerExprRegion;
use smallvec::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RustBinding {
    Deref,
    DerefCustomed,
    Reref,
    RerefMut,
    SelfValue,
    WrapInSome,
}

#[derive(Default)]
pub(crate) struct RustBindings {
    /// the order is the same as how it's written
    bindings: SmallVec<[RustBinding; 3]>,
}

impl std::ops::Deref for RustBindings {
    type Target = [RustBinding];

    fn deref(&self) -> &Self::Target {
        &self.bindings
    }
}

impl From<RustBinding> for RustBindings {
    fn from(binding: RustBinding) -> Self {
        Self {
            bindings: smallvec![binding],
        }
    }
}

impl RustBindings {
    pub(crate) fn is_non_trivial(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        if self.len() > 1 {
            return true;
        }
        self.bindings[0] != RustBinding::SelfValue
    }

    pub(crate) fn push(&mut self, binding: RustBinding) {
        match self.bindings.last() {
            Some(last_binding) => match (last_binding, binding) {
                // any binding except `DerefCustomed` can be merged into self value
                // (*a).<field_name> -> (*a).<field_name>
                // (&a).<field_name> -> (*a).<field_name>
                // (&mut a).<field_name> -> (*a).<field_name>
                //
                // in Rust, if type `A` doesn't implement Clone, for a value `a` of type `A`
                // `a.clone()` actually clones a reference to `a`, but in husky, no.
                (RustBinding::SelfValue, binding) if binding != RustBinding::DerefCustomed => (),
                // the following is automatically coercible, so we can cancel the last binding out
                // *&a -> a
                // *&mut a -> a
                // &*a -> a
                // &mut *a -> a
                (RustBinding::Deref, RustBinding::Reref | RustBinding::RerefMut)
                | (RustBinding::Reref | RustBinding::RerefMut, RustBinding::Deref) => {
                    self.bindings.pop();
                }
                (RustBinding::DerefCustomed, RustBinding::Reref | RustBinding::RerefMut) => {
                    unreachable!()
                }
                _ => self.bindings.push(binding),
            },
            None => self.bindings.push(binding),
        }
    }
}

#[test]
fn rust_bindings_works() {
    {
        // &*a -> a
        let mut bindings: RustBindings = RustBinding::Deref.into();
        bindings.push(RustBinding::Reref);
        assert!(bindings.is_empty())
    }
    {
        // &mut *a -> a
        let mut bindings: RustBindings = RustBinding::Deref.into();
        bindings.push(RustBinding::RerefMut);
        assert!(bindings.is_empty())
    }
    {
        // **a -> **a
        let mut bindings: RustBindings = RustBinding::Deref.into();
        bindings.push(RustBinding::Deref);
        assert_eq!(bindings.len(), 2)
    }
    {
        // &mut **a -> *a
        let mut bindings: RustBindings = RustBinding::Deref.into();
        bindings.push(RustBinding::Deref);
        bindings.push(RustBinding::RerefMut);
        assert_eq!(bindings.len(), 1)
    }
    {
        // (*a).<field_name> -> a.<field_name>
        let mut bindings: RustBindings = RustBinding::SelfValue.into();
        bindings.push(RustBinding::Deref);
        assert_eq!(bindings.len(), 1)
    }
    {
        // (&mut *a).<field_name> -> a.<field_name>
        let mut bindings: RustBindings = RustBinding::SelfValue.into();
        bindings.push(RustBinding::Deref);
        bindings.push(RustBinding::RerefMut);
        assert_eq!(bindings.len(), 1)
    }
}

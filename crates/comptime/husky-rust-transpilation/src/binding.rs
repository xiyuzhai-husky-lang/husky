use husky_hir_eager_expr::{
    coercion::{DedirectionHirEagerCoercion, HirEagerCoercion, RedirectionHirEagerCoercion},
    HirEagerExprEntry,
};
use husky_hir_ty::{indirections::HirIndirection, quary::HirContractedQuary, ritchie::HirContract};
use smallvec::*;

use crate::{
    expr::{role::HirEagerExprRole, RustPrecedence, RustPrecedenceRange},
    RustKeyword, RustPunctuation, RustTranspilationBuilder,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RustBinding {
    Deref,
    DerefCustomed,
    Deleash,
    Reref,
    RerefMut,
    Releash,
    SelfValue,
    WrapInSome,
}

impl RustBinding {
    pub(crate) fn precedence_range(self) -> RustPrecedenceRange {
        match self {
            RustBinding::Deref
            | RustBinding::DerefCustomed
            | RustBinding::Reref
            | RustBinding::RerefMut
            | RustBinding::Releash => RustPrecedenceRange::Geq(RustPrecedence::Prefix),
            RustBinding::Deleash | RustBinding::SelfValue => {
                RustPrecedenceRange::Geq(RustPrecedence::Suffix)
            }
            RustBinding::WrapInSome => RustPrecedenceRange::ANY,
        }
    }
}

#[derive(Debug)]
pub(crate) struct RustBindings {
    /// the order is from the innermost to the outermost
    bindings: SmallVec<[RustBinding; 4]>,
}

impl RustBindings {
    pub(crate) fn new(expr_entry: &HirEagerExprEntry, role: HirEagerExprRole) -> Self {
        let mut slf = RustBindings {
            bindings: smallvec![],
        };
        // the order is important!!!
        slf.add_coercion(expr_entry.coercion());
        slf.add_contracted_quary(
            expr_entry.contracted_quary(),
            expr_entry.is_always_copyable_before_coercion(),
            expr_entry.coercion(),
        );
        slf.add_role(role);
        slf
    }
}

impl<'a, 'b, E> RustTranspilationBuilder<'a, 'b, E> {
    pub(super) fn transpile_bindings(
        &mut self,
        rust_bindings: RustBindings,
        f: impl Fn(&mut RustTranspilationBuilder<'a, 'b, E>),
    ) {
        self.transpile_bindings_aux(
            rust_bindings.from_outermost_to_innermost_with_next_inner(),
            f,
        )
    }
    fn transpile_bindings_aux(
        &mut self,
        mut from_outermost_to_innermost: impl Iterator<Item = (RustBinding, Option<RustBinding>)>,
        f: impl Fn(&mut RustTranspilationBuilder<'a, 'b, E>),
    ) {
        match from_outermost_to_innermost.next() {
            Some((binding, next_inner)) => {
                match binding {
                    RustBinding::Deref | RustBinding::DerefCustomed => {
                        self.punctuation(RustPunctuation::DerefStar)
                    }
                    RustBinding::Deleash => {
                        if let Some(next_inner_binding) = next_inner {
                            match next_inner_binding {
                                RustBinding::Deref => todo!(),
                                RustBinding::DerefCustomed => todo!(),
                                RustBinding::Deleash => todo!(),
                                RustBinding::Reref => todo!(),
                                RustBinding::RerefMut => todo!(),
                                RustBinding::Releash => todo!(),
                                RustBinding::SelfValue => (),
                                RustBinding::WrapInSome => todo!(),
                            }
                        }
                    }
                    RustBinding::Reref => self.punctuation(RustPunctuation::Ambersand),
                    RustBinding::RerefMut => {
                        self.punctuation(RustPunctuation::Ambersand);
                        self.keyword(RustKeyword::Mut)
                    }
                    RustBinding::Releash => self.releash_left(),
                    RustBinding::SelfValue => (),
                    RustBinding::WrapInSome => self.wrap_in_some_left(),
                }
                self.transpile_bindings_aux(from_outermost_to_innermost, f);
                match binding {
                    RustBinding::Deref | RustBinding::DerefCustomed => (),
                    RustBinding::Deleash => {
                        if let Some(next_inner_binding) = next_inner {
                            match next_inner_binding {
                                RustBinding::Deref => todo!(),
                                RustBinding::DerefCustomed => todo!(),
                                RustBinding::Deleash => todo!(),
                                RustBinding::Reref => todo!(),
                                RustBinding::RerefMut => todo!(),
                                RustBinding::Releash => todo!(),
                                RustBinding::SelfValue => (),
                                RustBinding::WrapInSome => todo!(),
                            }
                        }
                        self.deleash()
                    }
                    RustBinding::Reref => (),
                    RustBinding::RerefMut => (),
                    RustBinding::Releash => self.releash_right(),
                    RustBinding::SelfValue => (),
                    RustBinding::WrapInSome => self.wrap_in_some_right(),
                }
            }
            None => f(self),
        }
    }
}

impl RustBindings {
    pub(crate) fn innermost(&self) -> Option<RustBinding> {
        self.bindings.first().copied()
    }

    pub(crate) fn outermost(&self) -> Option<RustBinding> {
        self.bindings.last().copied()
    }

    fn from_innermost_to_outermost_with_next_inner<'a>(
        &'a self,
    ) -> impl Iterator<Item = (RustBinding, Option<RustBinding>)> + 'a {
        let len = self.len();
        (0..len)
            .into_iter()
            .map(|i| (self.bindings[i], self.bindings.get(i - 1).copied()))
    }

    fn from_outermost_to_innermost_with_next_inner<'a>(
        &'a self,
    ) -> impl Iterator<Item = (RustBinding, Option<RustBinding>)> + 'a {
        let len = self.len();
        (0..len).into_iter().map(move |i| {
            (
                self.bindings[len - 1 - i],
                if len >= i + 2 {
                    Some(self.bindings[len - 1 - i - 1])
                } else {
                    None
                },
            )
        })
    }

    pub(crate) fn len(&self) -> usize {
        self.bindings.len()
    }
}

impl RustBindings {
    pub(crate) fn add_coercion(&mut self, coercion: Option<HirEagerCoercion>) {
        let Some(coercion) = coercion else { return };
        match coercion {
            HirEagerCoercion::Trivial(_) => (),
            HirEagerCoercion::Never => (),
            HirEagerCoercion::WrapInSome => self.add_outer_binding(RustBinding::WrapInSome),
            HirEagerCoercion::Redirection(redirection) => match redirection {
                RedirectionHirEagerCoercion::Releash => {
                    self.add_outer_binding(RustBinding::Releash)
                }
                RedirectionHirEagerCoercion::Reref => self.add_outer_binding(RustBinding::Reref),
                RedirectionHirEagerCoercion::RerefMut => todo!(),
            },
            HirEagerCoercion::Dedirection(dedirection) => match dedirection {
                DedirectionHirEagerCoercion::Deleash => todo!(),
                DedirectionHirEagerCoercion::Deref { lifetime } => {
                    self.add_outer_binding(RustBinding::Deref)
                }
                DedirectionHirEagerCoercion::DerefMut => todo!(),
            },
        }
    }

    pub(crate) fn add_contracted_quary(
        &mut self,
        contracted_quary: HirContractedQuary,
        is_always_copyable_before_coercion: bool,
        coercion: Option<HirEagerCoercion>,
    ) {
        if let Some(contract) = contracted_quary.contract() {
            match contract {
                HirContract::Pure => {
                    let is_always_copyable: bool = match coercion {
                        Some(coercion) => match coercion {
                            HirEagerCoercion::Trivial(_) => is_always_copyable_before_coercion,
                            HirEagerCoercion::Never => true,
                            HirEagerCoercion::WrapInSome => is_always_copyable_before_coercion,
                            HirEagerCoercion::Redirection(redirection_coercion) => {
                                match redirection_coercion {
                                    RedirectionHirEagerCoercion::Releash => true,
                                    RedirectionHirEagerCoercion::Reref => true,
                                    RedirectionHirEagerCoercion::RerefMut => false,
                                }
                            }
                            HirEagerCoercion::Dedirection(_) => todo!(),
                        },
                        None => is_always_copyable_before_coercion,
                    };
                    if !is_always_copyable {
                        self.add_outer_binding(RustBinding::Reref)
                    }
                }
                HirContract::Move => (),
                HirContract::Borrow => self.add_outer_binding(RustBinding::Reref),
                HirContract::BorrowMut => self.add_outer_binding(RustBinding::RerefMut),
                HirContract::Compterm => todo!(),
                HirContract::Leash => todo!(),
                HirContract::At => todo!(),
            }
        }
    }

    pub(crate) fn add_role(&mut self, role: HirEagerExprRole) {
        match role {
            HirEagerExprRole::SimpleSelfArgument => self.add_outer_binding(RustBinding::SelfValue),
            HirEagerExprRole::SelfArgumentWithIndirection { indirections } => {
                // the order matters!!!
                // indirection order is from innermost to outermost
                // so no need to rev
                for indirection in indirections.iter() {
                    match indirection {
                        HirIndirection::Place(_) => todo!(),
                        HirIndirection::Deleash => self.add_outer_binding(RustBinding::Deleash),
                    }
                }
                self.add_outer_binding(RustBinding::SelfValue);
            }
            HirEagerExprRole::MemoizedFieldSelfArgument { indirections } => {
                // the order matters!!!
                // indirection order is from innermost to outermost
                // so no need to rev
                for indirection in indirections.iter() {
                    match indirection {
                        HirIndirection::Place(_) => todo!(),
                        HirIndirection::Deleash => self.add_outer_binding(RustBinding::Deleash),
                    }
                }
            }
            HirEagerExprRole::Subexpr { .. } => (),
            HirEagerExprRole::RegularCallItem => (),
            HirEagerExprRole::Root => (),
            HirEagerExprRole::LetInitialValue => (),
        }
    }

    fn add_outer_binding(&mut self, binding: RustBinding) {
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

// #[test]
// fn rust_bindings_works() {
//     {
//         // &*a -> a
//         let mut bindings: RustBindings = RustBinding::Deref.into();
//         bindings.add(RustBinding::Reref);
//         assert!(bindings.is_empty())
//     }
//     {
//         // &mut *a -> a
//         let mut bindings: RustBindings = RustBinding::Deref.into();
//         bindings.add(RustBinding::RerefMut);
//         assert!(bindings.is_empty())
//     }
//     {
//         // **a -> **a
//         let mut bindings: RustBindings = RustBinding::Deref.into();
//         bindings.add(RustBinding::Deref);
//         assert_eq!(bindings.len(), 2)
//     }
//     {
//         // &mut **a -> *a
//         let mut bindings: RustBindings = RustBinding::Deref.into();
//         bindings.add(RustBinding::Deref);
//         bindings.add(RustBinding::RerefMut);
//         assert_eq!(bindings.len(), 1)
//     }
//     {
//         // (*a).<field_name> -> a.<field_name>
//         let mut bindings: RustBindings = RustBinding::SelfValue.into();
//         bindings.add(RustBinding::Deref);
//         assert_eq!(bindings.len(), 1)
//     }
//     {
//         // (&mut *a).<field_name> -> a.<field_name>
//         let mut bindings: RustBindings = RustBinding::SelfValue.into();
//         bindings.add(RustBinding::Deref);
//         bindings.add(RustBinding::RerefMut);
//         assert_eq!(bindings.len(), 1)
//     }
// }

use husky_hir_eager_expr::{
    coercion::{DedirectionHirEagerCoercion, HirEagerCoercion, RedirectionHirEagerCoercion},
    HirEagerExprData, HirEagerExprEntry,
};
use husky_hir_ty::{
    indirections::HirIndirection,
    quary::{HirContractedQuary, HirQuary},
    ritchie::HirContract,
    HirType,
};
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
    DerefMut,
}

impl RustBinding {
    pub(crate) fn precedence_range(self) -> RustPrecedenceRange {
        match self {
            RustBinding::Deref
            | RustBinding::DerefMut
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

#[derive(Debug, Clone)]
pub(crate) struct RustBindings {
    /// the order is from the innermost to the outermost
    bindings: SmallVec<[RustBinding; 4]>,
}

impl RustBindings {
    pub(crate) fn new(
        expr_entry: &HirEagerExprEntry,
        role: HirEagerExprRole,
        db: &::salsa::Db,
    ) -> Self {
        use husky_print_utils::p;
        let mut slf = Self::init(expr_entry);
        // the order is important!!!
        slf.add_coercion(expr_entry.coercion());
        slf.add_contracted_quary_after_coercion(
            expr_entry.contracted_quary_after_coercion(),
            expr_entry.is_base_ty_always_copyable(),
            expr_entry.coercion(),
        );
        slf.add_role(
            role,
            expr_entry.base_ty().is_always_leashed(db),
            expr_entry.contracted_quary_after_coercion(),
            expr_entry.is_base_ty_always_copyable(),
            db,
        );
        slf
    }

    fn init(expr_entry: &HirEagerExprEntry) -> RustBindings {
        match expr_entry.data() {
            HirEagerExprData::RuntimeVariable(_) => {
                let variable_bindings = match expr_entry.contracted_quary().quary() {
                    HirQuary::Compterm => todo!(),
                    HirQuary::StackPure { place } => {
                        if expr_entry.is_base_ty_always_copyable() {
                            smallvec![]
                        } else {
                            smallvec![RustBinding::Deref]
                        }
                    }
                    HirQuary::ImmutableOnStack { place } => smallvec![],
                    HirQuary::MutableOnStack { place } => smallvec![],
                    HirQuary::Transient => unreachable!(),
                    HirQuary::Ref { guard } => smallvec![RustBinding::Deref],
                    HirQuary::RefMut { place, lifetime } => {
                        match expr_entry.contracted_quary().contract() {
                            Some(contract) => match contract {
                                HirContract::Pure => {
                                    if expr_entry.is_base_ty_always_copyable() {
                                        smallvec![]
                                    } else {
                                        smallvec![RustBinding::Deref]
                                    }
                                }
                                HirContract::Move => smallvec![],
                                HirContract::Borrow => smallvec![RustBinding::Deref],
                                HirContract::BorrowMut => smallvec![RustBinding::DerefMut],
                                HirContract::Compterm => todo!(),
                                HirContract::Leash => todo!(),
                                HirContract::At => todo!(),
                            },
                            None => todo!(),
                        }
                    }
                    HirQuary::Leashed { place_idx } => smallvec![RustBinding::Deref],
                    HirQuary::Todo => todo!(),
                    HirQuary::Variable(_) => todo!(),
                };
                RustBindings {
                    bindings: variable_bindings,
                }
            }
            _ => RustBindings {
                bindings: smallvec![],
            },
        }
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
                    RustBinding::Deref | RustBinding::DerefMut | RustBinding::DerefCustomed => {
                        self.punctuation(RustPunctuation::DerefStar)
                    }
                    RustBinding::Deleash => {
                        if let Some(next_inner) = next_inner {
                            match next_inner {
                                RustBinding::Deref => todo!(),
                                RustBinding::DerefMut => todo!(),
                                RustBinding::DerefCustomed => todo!(),
                                RustBinding::Deleash => todo!(),
                                RustBinding::Reref => todo!(),
                                RustBinding::RerefMut => todo!(),
                                RustBinding::Releash => unreachable!("should be absorbed"),
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
                    RustBinding::SelfValue => {
                        if let Some(next_inner) = next_inner {
                            match next_inner {
                                RustBinding::Deref
                                | RustBinding::DerefMut
                                | RustBinding::DerefCustomed
                                | RustBinding::Reref
                                | RustBinding::RerefMut => {
                                    unreachable!("should be absorbed into self value")
                                }
                                RustBinding::Deleash => (),
                                RustBinding::Releash => (),
                                RustBinding::SelfValue => {
                                    unreachable!("self value binding couldn't appear twice")
                                }
                                RustBinding::WrapInSome => (),
                            }
                        }
                    }
                    RustBinding::WrapInSome => self.wrap_in_some_left(),
                }
                self.transpile_bindings_aux(from_outermost_to_innermost, f);
                match binding {
                    RustBinding::Deref | RustBinding::DerefMut | RustBinding::DerefCustomed => (),
                    RustBinding::Deleash => {
                        if let Some(next_inner_binding) = next_inner {
                            match next_inner_binding {
                                RustBinding::Deref => todo!(),
                                RustBinding::DerefMut => todo!(),
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
                    RustBinding::SelfValue => {
                        if let Some(next_inner) = next_inner {
                            match next_inner {
                                RustBinding::Deref
                                | RustBinding::DerefMut
                                | RustBinding::DerefCustomed
                                | RustBinding::Reref
                                | RustBinding::RerefMut => {
                                    unreachable!("should be absorbed into self value")
                                }
                                RustBinding::Deleash => (),
                                RustBinding::Releash => (),
                                RustBinding::SelfValue => {
                                    unreachable!("self value binding couldn't appear twice")
                                }
                                RustBinding::WrapInSome => (),
                            }
                        }
                    }
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
                DedirectionHirEagerCoercion::Deleash => {
                    self.add_outer_binding(RustBinding::Deleash)
                }
                DedirectionHirEagerCoercion::Deref { lifetime } => {
                    self.add_outer_binding(RustBinding::Deref)
                }
                DedirectionHirEagerCoercion::DerefMut => todo!(),
            },
        }
    }

    pub(crate) fn add_contracted_quary_after_coercion(
        &mut self,
        contracted_quary_after_coercion: HirContractedQuary,
        always_copyable: bool,
        coercion: Option<HirEagerCoercion>,
    ) {
        if let Some(contract) = contracted_quary_after_coercion.contract() {
            match contract {
                HirContract::Pure => {
                    let is_always_copyable_after_coercion: bool = match coercion {
                        Some(coercion) => match coercion {
                            HirEagerCoercion::Trivial(_) => always_copyable,
                            HirEagerCoercion::Never => true,
                            HirEagerCoercion::WrapInSome => always_copyable,
                            HirEagerCoercion::Redirection(redirection_coercion) => {
                                match redirection_coercion {
                                    RedirectionHirEagerCoercion::Releash => true,
                                    RedirectionHirEagerCoercion::Reref => true,
                                    RedirectionHirEagerCoercion::RerefMut => false,
                                }
                            }
                            // ad hoc
                            HirEagerCoercion::Dedirection(_) => false,
                        },
                        None => always_copyable,
                    };
                    if !is_always_copyable_after_coercion {
                        self.add_outer_binding(RustBinding::Reref)
                    }
                }
                HirContract::Move => (),
                HirContract::Borrow => self.add_outer_binding(RustBinding::Reref),
                HirContract::BorrowMut => self.add_outer_binding(RustBinding::RerefMut),
                HirContract::Compterm => todo!(),
                // ad hoc
                HirContract::Leash => match contracted_quary_after_coercion.quary() {
                    HirQuary::Compterm => todo!(),
                    HirQuary::StackPure { place } => (),
                    HirQuary::ImmutableOnStack { .. } => (),
                    HirQuary::MutableOnStack { place } => (),
                    HirQuary::Transient => todo!(),
                    HirQuary::Ref { guard } => todo!(),
                    HirQuary::RefMut { place, lifetime } => unreachable!(),
                    HirQuary::Leashed { place_idx } => self.add_outer_binding(RustBinding::Releash),
                    HirQuary::Todo => todo!(),
                    HirQuary::Variable(_) => todo!(),
                },
                HirContract::At => todo!(),
            }
        }
    }

    pub(crate) fn add_role(
        &mut self,
        role: HirEagerExprRole,
        is_base_ty_always_leashed: bool,
        contracted_quary_after_coercion: HirContractedQuary,
        always_copyable: bool,
        db: &::salsa::Db,
    ) {
        match role {
            HirEagerExprRole::SimpleSelfArgument => {
                if is_base_ty_always_leashed {
                    self.add_outer_binding(RustBinding::Deleash);
                }
                self.add_outer_binding(RustBinding::SelfValue);
            }
            HirEagerExprRole::AssignSelfArgument => self.add_outer_binding(RustBinding::DerefMut),
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
                // ad hoc
                self.add_outer_binding(RustBinding::Releash)
            }
            HirEagerExprRole::Subexpr { .. } => (),
            HirEagerExprRole::Root => (),
            HirEagerExprRole::PatternOpd { contract } => {
                if contracted_quary_after_coercion.contract().is_none() {
                    match contracted_quary_after_coercion.quary() {
                        HirQuary::Compterm => (),
                        HirQuary::StackPure { place } => (),
                        HirQuary::ImmutableOnStack { place } => {
                            if !always_copyable {
                                self.add_outer_binding(RustBinding::Reref)
                            }
                        }
                        HirQuary::MutableOnStack { place } => match contracted_quary_after_coercion
                            .contract()
                        {
                            Some(contract) => match contract {
                                HirContract::Pure => {
                                    if !always_copyable {
                                        self.add_outer_binding(RustBinding::Reref)
                                    }
                                }
                                HirContract::Move => (),
                                HirContract::Borrow => self.add_outer_binding(RustBinding::Reref),
                                HirContract::BorrowMut => {
                                    self.add_outer_binding(RustBinding::RerefMut)
                                }
                                HirContract::Compterm => todo!(),
                                HirContract::Leash => todo!(),
                                HirContract::At => todo!(),
                            },
                            None => {
                                if !always_copyable {
                                    self.add_outer_binding(RustBinding::Reref)
                                }
                            }
                        },
                        HirQuary::Transient => match contract {
                            HirContract::Pure => (),
                            HirContract::Borrow => {
                                if !always_copyable {
                                    self.add_outer_binding(RustBinding::Reref)
                                }
                            }
                            HirContract::Move => (),
                            HirContract::BorrowMut => todo!(),
                            HirContract::Compterm => todo!(),
                            HirContract::Leash => todo!(),
                            HirContract::At => todo!(),
                        },
                        HirQuary::Ref { guard } => todo!(),
                        HirQuary::RefMut { place, lifetime } => match contract {
                            HirContract::Pure => todo!(),
                            HirContract::Move => todo!(),
                            HirContract::Borrow => todo!(),
                            HirContract::BorrowMut => todo!(),
                            HirContract::Compterm => todo!(),
                            HirContract::Leash => todo!(),
                            HirContract::At => todo!(),
                        },
                        HirQuary::Leashed { place_idx } => {
                            if !always_copyable {
                                self.add_outer_binding(RustBinding::Reref)
                            }
                        }
                        HirQuary::Todo => todo!(),
                        HirQuary::Variable(_) => todo!(),
                    }
                }
            }
            HirEagerExprRole::RegularCallItem { contract } => {
                if contracted_quary_after_coercion.contract().is_none() {
                    match contracted_quary_after_coercion.quary() {
                        HirQuary::Compterm => (),
                        HirQuary::StackPure { place } => {
                            if !always_copyable {
                                self.add_outer_binding(RustBinding::Reref)
                            }
                        }
                        HirQuary::ImmutableOnStack { place } => {
                            if !always_copyable {
                                self.add_outer_binding(RustBinding::Reref)
                            }
                        }
                        HirQuary::MutableOnStack { place } => match contracted_quary_after_coercion
                            .contract()
                        {
                            Some(contract) => match contract {
                                HirContract::Pure => {
                                    if !always_copyable {
                                        self.add_outer_binding(RustBinding::Reref)
                                    }
                                }
                                HirContract::Move => (),
                                HirContract::Borrow => self.add_outer_binding(RustBinding::Reref),
                                HirContract::BorrowMut => {
                                    self.add_outer_binding(RustBinding::RerefMut)
                                }
                                HirContract::Compterm => todo!(),
                                HirContract::Leash => todo!(),
                                HirContract::At => todo!(),
                            },
                            None => {
                                if !always_copyable {
                                    self.add_outer_binding(RustBinding::Reref)
                                }
                            }
                        },
                        HirQuary::Transient => match contract {
                            HirContract::Pure | HirContract::Borrow => {
                                if !always_copyable {
                                    self.add_outer_binding(RustBinding::Reref)
                                }
                            }
                            HirContract::Move => (),
                            HirContract::BorrowMut => todo!(),
                            HirContract::Compterm => todo!(),
                            HirContract::Leash => todo!(),
                            HirContract::At => todo!(),
                        },
                        HirQuary::Ref { guard } => todo!(),
                        HirQuary::RefMut { place, lifetime } => match contract {
                            HirContract::Pure => todo!(),
                            HirContract::Move => todo!(),
                            HirContract::Borrow => todo!(),
                            HirContract::BorrowMut => todo!(),
                            HirContract::Compterm => todo!(),
                            HirContract::Leash => todo!(),
                            HirContract::At => todo!(),
                        },
                        HirQuary::Leashed { place_idx } => {
                            if !always_copyable {
                                self.add_outer_binding(RustBinding::Reref)
                            }
                        }
                        HirQuary::Todo => todo!(),
                        HirQuary::Variable(_) => todo!(),
                    }
                }
            }
        }
    }

    fn add_outer_binding(&mut self, binding: RustBinding) {
        if binding == RustBinding::SelfValue {
            while let Some(last_binding) = self.bindings.last() {
                match last_binding {
                    RustBinding::Deref
                    | RustBinding::DerefCustomed
                    | RustBinding::DerefMut
                    | RustBinding::Reref
                    | RustBinding::RerefMut => {
                        self.bindings.pop();
                    }
                    RustBinding::Deleash | RustBinding::Releash | RustBinding::WrapInSome => break,
                    RustBinding::SelfValue => unreachable!(),
                }
            }
            self.bindings.push(RustBinding::SelfValue)
        } else {
            match self.bindings.last() {
                Some(last_binding) => match (last_binding, binding) {
                    // any binding except `DerefCustomed` can be merged into self value
                    // (*a).<field_name> -> (*a).<field_name>
                    // (&a).<field_name> -> (*a).<field_name>
                    // (&mut a).<field_name> -> (*a).<field_name>
                    //
                    // in Rust, if type `A` doesn't implement Clone, for a value `a` of type `A`
                    // `a.clone()` actually clones a reference to `a`, but in husky, no.
                    (RustBinding::SelfValue, binding) if binding != RustBinding::DerefCustomed => {
                        ()
                    }
                    // the followings are automatically coercible, so we can cancel the last binding out
                    // *&a -> a
                    // *&mut a -> a
                    // &*a -> a
                    // &mut *a -> a
                    // *~a -> a
                    // ~*a -> a
                    (
                        RustBinding::Deref | RustBinding::DerefMut,
                        RustBinding::Reref | RustBinding::RerefMut,
                    )
                    | (
                        RustBinding::Reref | RustBinding::RerefMut,
                        RustBinding::Deref | RustBinding::DerefMut,
                    )
                    | (RustBinding::Deleash, RustBinding::Releash)
                    | (RustBinding::Releash, RustBinding::Deleash) => {
                        self.bindings.pop();
                    }
                    (RustBinding::DerefCustomed, RustBinding::Reref | RustBinding::RerefMut) => {
                        unreachable!()
                    }
                    // (RustBinding::Reref, RustBinding:)
                    _ => self.bindings.push(binding),
                },
                None => self.bindings.push(binding),
            }
        }
    }
}

#[test]
fn rust_bindings_works() {
    #[cfg(test)]
    impl From<RustBinding> for RustBindings {
        fn from(binding: RustBinding) -> Self {
            Self {
                bindings: smallvec![binding],
            }
        }
    }

    #[cfg(test)]
    impl RustBindings {
        fn is_empty(&self) -> bool {
            self.bindings.is_empty()
        }
    }

    {
        // &*a -> a
        let mut bindings: RustBindings = RustBinding::Deref.into();
        bindings.add_outer_binding(RustBinding::Reref);
        assert!(bindings.is_empty())
    }
    {
        // &mut *a -> a
        let mut bindings: RustBindings = RustBinding::Deref.into();
        bindings.add_outer_binding(RustBinding::RerefMut);
        assert!(bindings.is_empty())
    }
    {
        // **a -> **a
        let mut bindings: RustBindings = RustBinding::Deref.into();
        bindings.add_outer_binding(RustBinding::Deref);
        assert_eq!(bindings.len(), 2)
    }
    {
        // &mut **a -> *a
        let mut bindings: RustBindings = RustBinding::Deref.into();
        bindings.add_outer_binding(RustBinding::Deref);
        bindings.add_outer_binding(RustBinding::RerefMut);
        assert_eq!(bindings.len(), 1)
    }
    {
        // (*a).<field_name> -> a.<field_name>
        let mut bindings: RustBindings = RustBinding::SelfValue.into();
        bindings.add_outer_binding(RustBinding::Deref);
        assert_eq!(bindings.len(), 1)
    }
    {
        // (&mut *a).<field_name> -> a.<field_name>
        let mut bindings: RustBindings = RustBinding::SelfValue.into();
        bindings.add_outer_binding(RustBinding::Deref);
        bindings.add_outer_binding(RustBinding::RerefMut);
        assert_eq!(bindings.len(), 1)
    }
    {
        // (*a).<field_name> -> a.<field_name>
        let mut bindings: RustBindings = RustBinding::Deleash.into();
        bindings.add_outer_binding(RustBinding::SelfValue);
        assert_eq!(bindings.len(), 2)
    }
    {
        // (~a).<field_name> -> a.<field_name>
        let mut bindings: RustBindings = RustBinding::Deleash.into();
        bindings.add_outer_binding(RustBinding::SelfValue);
        assert_eq!(bindings.len(), 2)
    }
}

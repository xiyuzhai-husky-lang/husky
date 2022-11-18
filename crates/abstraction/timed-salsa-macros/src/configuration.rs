pub(crate) struct Configuration {
    pub(crate) jar_ty: syn::Type,
    pub(crate) salsa_struct_ty: syn::Type,
    pub(crate) key_ty: syn::Type,
    pub(crate) value_ty: syn::Type,
    pub(crate) cycle_strategy: CycleRecoveryStrategy,
    pub(crate) backdate_fn: syn::ImplItemMethod,
    pub(crate) execute_fn: syn::ImplItemMethod,
    pub(crate) recover_fn: syn::ImplItemMethod,
}

impl Configuration {
    pub(crate) fn to_impl(&self, self_ty: &syn::Type) -> syn::ItemImpl {
        let Configuration {
            jar_ty,
            salsa_struct_ty,
            key_ty,
            value_ty,
            cycle_strategy,
            backdate_fn,
            execute_fn,
            recover_fn,
        } = self;
        parse_quote! {
            impl timed_salsa::function::Configuration for #self_ty {
                type Jar = #jar_ty;
                type SalsaStruct = #salsa_struct_ty;
                type Key = #key_ty;
                type Value = #value_ty;
                const CYCLE_STRATEGY: timed_salsa::cycle::CycleRecoveryStrategy = #cycle_strategy;
                #backdate_fn
                #execute_fn
                #recover_fn
            }
        }
    }
}

pub(crate) enum CycleRecoveryStrategy {
    Panic,
    Fallback,
}

impl quote::ToTokens for CycleRecoveryStrategy {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            CycleRecoveryStrategy::Panic => {
                tokens.extend(quote! {timed_salsa::cycle::CycleRecoveryStrategy::Panic})
            }
            CycleRecoveryStrategy::Fallback => {
                tokens.extend(quote! {timed_salsa::cycle::CycleRecoveryStrategy::Fallback})
            }
        }
    }
}

/// Returns an appropriate definition for `should_backdate_value` depending on
/// whether this value is memoized or not.
pub(crate) fn should_backdate_value_fn(should_backdate: bool) -> syn::ImplItemMethod {
    if should_backdate {
        parse_quote! {
            fn should_backdate_value(v1: &Self::Value, v2: &Self::Value) -> bool {
                timed_salsa::function::should_backdate_value(v1, v2)
            }
        }
    } else {
        parse_quote! {
            fn should_backdate_value(_v1: &Self::Value, _v2: &Self::Value) -> bool {
                false
            }
        }
    }
}

/// Returns an appropriate definition for `recover_from_cycle` for cases where
/// the cycle recovery is panic.
pub(crate) fn panic_cycle_recovery_fn() -> syn::ImplItemMethod {
    parse_quote! {
        fn recover_from_cycle(
            _db: &timed_salsa::function::DynDb<Self>,
            _cycle: &timed_salsa::Cycle,
            _key: Self::Key,
        ) -> Self::Value {
            panic!()
        }
    }
}

pub(crate) fn value_ty(sig: &syn::Signature) -> syn::Type {
    match &sig.output {
        syn::ReturnType::Default => parse_quote!(()),
        syn::ReturnType::Type(_, ty) => syn::Type::clone(ty),
    }
}

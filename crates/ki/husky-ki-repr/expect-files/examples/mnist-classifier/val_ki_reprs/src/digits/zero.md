```rust
[
    (
        FormPath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
        KiRepr {
            ki_domain_repr: Omni,
            opn: KiOpn::Linkage(
                Linkage {
                    data: LinkageData::MajorVal {
                        path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        instantiation: LinInstantiation {
                            symbol_resolutions: [],
                            separator: None,
                        },
                    },
                },
            ),
            arguments: [],
            source: KiReprSource::Val(
                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
            ),
            caching_class: Val,
        },
    ),
    (
        FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
        KiRepr {
            ki_domain_repr: Omni,
            opn: KiOpn::ValLazilyDefined(
                FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
            ),
            arguments: [],
            source: KiReprSource::Val(
                FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
            ),
            caching_class: Val,
        },
    ),
]
```
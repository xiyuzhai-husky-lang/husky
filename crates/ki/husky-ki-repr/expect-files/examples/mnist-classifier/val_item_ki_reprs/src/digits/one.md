```rust
[
    (
        FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
        KiRepr {
            ki_domain_repr: Omni,
            opn: KiOpn::Linkage(
                Linkage {
                    data: LinkageData::MajorVal {
                        path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        instantiation: LinInstantiation {
                            symbol_resolutions: [],
                            separator: None,
                        },
                    },
                },
            ),
            arguments: [],
            source: KiReprSource::ValItem(
                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
            ),
            caching_class: Val,
        },
    ),
    (
        FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
        KiRepr {
            ki_domain_repr: Omni,
            opn: KiOpn::ValItemLazilyDefined(
                FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
            ),
            arguments: [],
            source: KiReprSource::ValItem(
                FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
            ),
            caching_class: Val,
        },
    ),
]
```
[
    (
        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
        ValRepr {
            val_domain_repr: Omni,
            opn: ValOpn::Linkage(
                Linkage {
                    data: LinkageData::ValItem {
                        path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        instantiation: LinkageInstantiation {
                            symbol_resolutions: [],
                            separator: None,
                        },
                    },
                },
            ),
            arguments: [],
            caching_class: ValItem,
        },
    ),
    (
        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
        ValRepr {
            val_domain_repr: Omni,
            opn: ValOpn::ValItemLazilyDefined(
                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
            ),
            arguments: [],
            caching_class: ValItem,
        },
    ),
]
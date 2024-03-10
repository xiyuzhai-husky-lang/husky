[
    (
        Linkage {
            data: LinkageData::EnumU8ToJsonValue {
                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
            },
        },
        None,
    ),
    (
        Linkage {
            data: LinkageData::MajorVal {
                path: FugitivePath(`mnist::input`, `Val`),
                instantiation: LinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linkage {
            data: LinkageData::AssocRitchie {
                path: AssocItemPath::TypeItem(
                    TypeItemPath(`(mnist::BinaryImage28(0)::new_zeros`, `AssocRitchie(
                        Fn,
                    )`),
                ),
                instantiation: LinInstantiation {
                    symbol_resolutions: [],
                    separator: Some(
                        0,
                    ),
                },
            },
        },
        None,
    ),
    (
        Linkage {
            data: LinkageData::AssocRitchie {
                path: AssocItemPath::TypeItem(
                    TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssocRitchie(
                        Fn,
                    )`),
                ),
                instantiation: LinInstantiation {
                    symbol_resolutions: [],
                    separator: Some(
                        0,
                    ),
                },
            },
        },
        None,
    ),
]
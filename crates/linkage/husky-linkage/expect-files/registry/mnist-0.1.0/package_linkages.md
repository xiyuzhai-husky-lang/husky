[
    Linkage {
        data: LinkageData::EnumU8ToJsonValue {
            ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist::input`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::AssocFn {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`(mnist::BinaryImage28(0)::new_zeros`, `AssocFunctionFn`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::AssocFn {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssocFunctionFn`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
]
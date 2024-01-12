[
    Linkage {
        data: LinkageData::EnumU8ToJsonValue {
            ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
        },
    },
    Linkage {
        data: LinkageData::ValItem {
            path: FugitivePath(`mnist::input`, `Val`),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::AssociatedFunctionFn {
            path: AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist::BinaryImage28(0)::new_zeros`, `AssociatedFunctionFn`),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::AssociatedFunctionFn {
            path: AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssociatedFunctionFn`),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
]
Ok(
    [
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::slice::Slice`, `Extern`),
            ),
        ),
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::slice::CyclicSlice`, `Extern`),
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath {
                    module_path: `core::slice`,
                    ty_path: TypePath(`core::slice::Slice`, `Extern`),
                    disambiguator: 0,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::Slice`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `len`,
                    item_kind: MethodFn,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::Slice`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `swap`,
                    item_kind: MethodFn,
                },
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath {
                    module_path: `core::slice`,
                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                    disambiguator: 0,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `ilen`,
                    item_kind: MethodFn,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `start`,
                    item_kind: MethodFn,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `end`,
                    item_kind: MethodFn,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `first`,
                    item_kind: MethodFn,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `last`,
                    item_kind: MethodFn,
                },
            ),
        ),
    ],
)
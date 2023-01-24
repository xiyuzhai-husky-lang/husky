[
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`core::num::i8`, `Alien`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`core::num::i16`, `Alien`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`core::num::i32`, `Alien`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`core::num::i64`, `Alien`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`core::num::f8`, `Alien`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`core::num::f16`, `Alien`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`core::num::f32`, `Alien`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`core::num::f64`, `Alien`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `core::num`,
                        impl_block_kind: Type {
                            ty: TypePath(`core::num::i8`, `Alien`),
                        },
                    },
                    ast_idx: 18,
                    body: ArenaIdxRange(
                        0..1,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 7,
                            },
                        ),
                    },
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 4,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 7,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`core::num::i8`, `Alien`),
                            ident: `abs`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: Type {
                                ty: TypePath(`core::num::i8`, `Alien`),
                            },
                        },
                        ast_idx: 18,
                        body: ArenaIdxRange(
                            0..1,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 7,
                                },
                            ),
                        },
                    },
                    ast_idx: 0,
                    ident: `abs`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: Public,
                    is_generic: false,
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `core::num`,
                        impl_block_kind: Type {
                            ty: TypePath(`core::num::i16`, `Alien`),
                        },
                    },
                    ast_idx: 23,
                    body: ArenaIdxRange(
                        2..3,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 8,
                            },
                        ),
                    },
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 4,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 8,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`core::num::i16`, `Alien`),
                            ident: `abs`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: Type {
                                ty: TypePath(`core::num::i16`, `Alien`),
                            },
                        },
                        ast_idx: 23,
                        body: ArenaIdxRange(
                            2..3,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 8,
                                },
                            ),
                        },
                    },
                    ast_idx: 2,
                    ident: `abs`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: Public,
                    is_generic: false,
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `core::num`,
                        impl_block_kind: Type {
                            ty: TypePath(`core::num::i32`, `Alien`),
                        },
                    },
                    ast_idx: 27,
                    body: ArenaIdxRange(
                        4..5,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 9,
                            },
                        ),
                    },
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 4,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 9,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`core::num::i32`, `Alien`),
                            ident: `abs`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: Type {
                                ty: TypePath(`core::num::i32`, `Alien`),
                            },
                        },
                        ast_idx: 27,
                        body: ArenaIdxRange(
                            4..5,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 9,
                                },
                            ),
                        },
                    },
                    ast_idx: 4,
                    ident: `abs`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: Public,
                    is_generic: false,
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `core::num`,
                        impl_block_kind: Type {
                            ty: TypePath(`core::num::i64`, `Alien`),
                        },
                    },
                    ast_idx: 31,
                    body: ArenaIdxRange(
                        6..7,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 10,
                            },
                        ),
                    },
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 4,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 10,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`core::num::i64`, `Alien`),
                            ident: `abs`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: Type {
                                ty: TypePath(`core::num::i64`, `Alien`),
                            },
                        },
                        ast_idx: 31,
                        body: ArenaIdxRange(
                            6..7,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 10,
                                },
                            ),
                        },
                    },
                    ast_idx: 6,
                    ident: `abs`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: Public,
                    is_generic: false,
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `core::num`,
                        impl_block_kind: Type {
                            ty: TypePath(`core::num::f8`, `Alien`),
                        },
                    },
                    ast_idx: 35,
                    body: ArenaIdxRange(
                        8..9,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 11,
                            },
                        ),
                    },
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 4,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 11,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`core::num::f8`, `Alien`),
                            ident: `abs`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: Type {
                                ty: TypePath(`core::num::f8`, `Alien`),
                            },
                        },
                        ast_idx: 35,
                        body: ArenaIdxRange(
                            8..9,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 11,
                                },
                            ),
                        },
                    },
                    ast_idx: 8,
                    ident: `abs`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: Public,
                    is_generic: false,
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `core::num`,
                        impl_block_kind: Type {
                            ty: TypePath(`core::num::f16`, `Alien`),
                        },
                    },
                    ast_idx: 39,
                    body: ArenaIdxRange(
                        10..11,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 12,
                            },
                        ),
                    },
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 4,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`core::num::f16`, `Alien`),
                            ident: `abs`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: Type {
                                ty: TypePath(`core::num::f16`, `Alien`),
                            },
                        },
                        ast_idx: 39,
                        body: ArenaIdxRange(
                            10..11,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 12,
                                },
                            ),
                        },
                    },
                    ast_idx: 10,
                    ident: `abs`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: Public,
                    is_generic: false,
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `core::num`,
                        impl_block_kind: Type {
                            ty: TypePath(`core::num::f32`, `Alien`),
                        },
                    },
                    ast_idx: 43,
                    body: ArenaIdxRange(
                        12..13,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 13,
                            },
                        ),
                    },
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 4,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 13,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`core::num::f32`, `Alien`),
                            ident: `abs`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: Type {
                                ty: TypePath(`core::num::f32`, `Alien`),
                            },
                        },
                        ast_idx: 43,
                        body: ArenaIdxRange(
                            12..13,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 13,
                                },
                            ),
                        },
                    },
                    ast_idx: 12,
                    ident: `abs`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: Public,
                    is_generic: false,
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `core::num`,
                        impl_block_kind: Type {
                            ty: TypePath(`core::num::f64`, `Alien`),
                        },
                    },
                    ast_idx: 47,
                    body: ArenaIdxRange(
                        14..15,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 14,
                            },
                        ),
                    },
                },
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 4,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`core::num::f64`, `Alien`),
                            ident: `abs`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: Type {
                                ty: TypePath(`core::num::f64`, `Alien`),
                            },
                        },
                        ast_idx: 47,
                        body: ArenaIdxRange(
                            14..15,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 14,
                                },
                            ),
                        },
                    },
                    ast_idx: 14,
                    ident: `abs`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: Public,
                    is_generic: false,
                },
            ),
        ),
    },
]
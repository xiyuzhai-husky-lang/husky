[
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationArgumentTypeNotInferred,
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_block_kind: Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    },
                    ast_idx: 125,
                    body: ArenaIdxRange(
                        67..79,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 27,
                            },
                        ),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 22,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 27,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 154,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ident: `raw_contours`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                        },
                        ast_idx: 125,
                        body: ArenaIdxRange(
                            67..79,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 27,
                                },
                            ),
                        },
                    },
                    ast_idx: 67,
                    ident: `raw_contours`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 22,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 27,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 155,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ident: `eff_holes`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                        },
                        ast_idx: 125,
                        body: ArenaIdxRange(
                            67..79,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 27,
                                },
                            ),
                        },
                    },
                    ast_idx: 68,
                    ident: `eff_holes`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 22,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 27,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 159,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ident: `max_hole_ilen`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                        },
                        ast_idx: 125,
                        body: ArenaIdxRange(
                            67..79,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 27,
                                },
                            ),
                        },
                    },
                    ast_idx: 69,
                    ident: `max_hole_ilen`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 22,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 27,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 164,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ident: `max_row_span`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                        },
                        ast_idx: 125,
                        body: ArenaIdxRange(
                            67..79,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 27,
                                },
                            ),
                        },
                    },
                    ast_idx: 70,
                    ident: `max_row_span`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 22,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 27,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 168,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ident: `row_span_sum`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                        },
                        ast_idx: 125,
                        body: ArenaIdxRange(
                            67..79,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 27,
                                },
                            ),
                        },
                    },
                    ast_idx: 71,
                    ident: `row_span_sum`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 22,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 27,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 169,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ident: `distribution`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                        },
                        ast_idx: 125,
                        body: ArenaIdxRange(
                            67..79,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 27,
                                },
                            ),
                        },
                    },
                    ast_idx: 72,
                    ident: `distribution`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 22,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 27,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 144,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ident: `upper_mass`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                        },
                        ast_idx: 125,
                        body: ArenaIdxRange(
                            67..79,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 27,
                                },
                            ),
                        },
                    },
                    ast_idx: 73,
                    ident: `upper_mass`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 22,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 27,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 145,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ident: `lower_mass`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                        },
                        ast_idx: 125,
                        body: ArenaIdxRange(
                            67..79,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 27,
                                },
                            ),
                        },
                    },
                    ast_idx: 74,
                    ident: `lower_mass`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 22,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 27,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 175,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ident: `top_k_row_span_sum`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                        },
                        ast_idx: 125,
                        body: ArenaIdxRange(
                            67..79,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 27,
                                },
                            ),
                        },
                    },
                    ast_idx: 75,
                    ident: `top_k_row_span_sum`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 22,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 27,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 178,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ident: `top_k_row_right_mass_sum`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                        },
                        ast_idx: 125,
                        body: ArenaIdxRange(
                            67..79,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 27,
                                },
                            ),
                        },
                    },
                    ast_idx: 76,
                    ident: `top_k_row_right_mass_sum`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
]
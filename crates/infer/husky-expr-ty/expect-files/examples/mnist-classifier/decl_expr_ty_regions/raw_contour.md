[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Err(
                    Derived(
                        ResolvedTypeUninitialized,
                    ),
                ),
            },
            TypeInfo {
                ty_result: Err(
                    Derived(
                        PrefixOperandTypeNotInferred,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Err(
                    Derived(
                        ResolvedTypeUninitialized,
                    ),
                ),
            },
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::ImplBlock(
                ImplBlockId {
                    module_path: `mnist_classifier::raw_contour`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::raw_contour`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    },
                    ident: `line_segment_sketch`,
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::raw_contour`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    },
                    ident: `bounding_box`,
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Err(
                    Derived(
                        ResolvedTypeUninitialized,
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::raw_contour`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    },
                    ident: `relative_bounding_box`,
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::raw_contour`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    },
                    ident: `contour_len`,
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::raw_contour`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    },
                    ident: `displacement`,
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
]
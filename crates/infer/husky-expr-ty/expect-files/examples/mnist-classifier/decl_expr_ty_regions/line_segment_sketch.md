[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            EqsExactly(
                                ExpectEqsExactlyResolvedOk {
                                    destination: Resolved(
                                        ReducedTerm(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        PrefixOperandTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                            expectation: EqsExactly {
                                destination: Resolved(
                                    ReducedTerm(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    EqsExactly(
                                        ExpectEqsExactlyResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            InsSort(
                                ExpectInsSortResolvedOk {
                                    destination: Resolved(
                                        ReducedTerm(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Vector2d`),
            ),
            LocalTerm::Resolved(
                Term(`f32`),
            ),
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 2,
                            expectee: Resolved(
                                ReducedTerm(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            InsSort(
                                ExpectInsSortResolvedOk {
                                    destination: Resolved(
                                        ReducedTerm(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Vector2d`),
            ),
            LocalTerm::Resolved(
                Term(`f32`),
            ),
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 2,
                            expectee: Resolved(
                                ReducedTerm(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            InsSort(
                                ExpectInsSortResolvedOk {
                                    destination: Resolved(
                                        ReducedTerm(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Ref TermLiteral::EvalLifetime RawContour`),
            ),
            LocalTerm::Resolved(
                Term(`i32`),
            ),
            LocalTerm::Resolved(
                Term(`f32`),
            ),
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 4,
                            expectee: Resolved(
                                ReducedTerm(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            InsSort(
                                ExpectInsSortResolvedOk {
                                    destination: Resolved(
                                        ReducedTerm(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Ref TermLiteral::EvalLifetime RawContour`),
            ),
            LocalTerm::Resolved(
                Term(`i32`),
            ),
            LocalTerm::Resolved(
                Term(`i32`),
            ),
            LocalTerm::Resolved(
                Term(`f32`),
            ),
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 5,
                            expectee: Resolved(
                                ReducedTerm(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Ref TermLiteral::EvalLifetime RawContour`),
            ),
            LocalTerm::Resolved(
                Term(`f32`),
            ),
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::ImplBlock(
                ImplBlockId {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        },
                    },
                    ident: `new`,
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
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Ref TermLiteral::EvalLifetime RawContour`),
            ),
            LocalTerm::Resolved(
                Term(`i32`),
            ),
            LocalTerm::Resolved(
                Term(`i32`),
            ),
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: Some(
            Term(`LineSegmentStroke`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        },
                    },
                    ident: `displacement`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            InsSort(
                                ExpectInsSortResolvedOk {
                                    destination: Resolved(
                                        ReducedTerm(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: Some(
            Term(`LineSegmentStroke`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::ImplBlock(
                ImplBlockId {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            InsSort(
                                ExpectInsSortResolvedOk {
                                    destination: Resolved(
                                        ReducedTerm(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        },
                    },
                    ident: `concave_components`,
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
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: Some(
            Term(`LineSegmentSketch`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        },
                    },
                    ident: `bounding_box`,
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
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: Some(
            Term(`LineSegmentSketch`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        },
                    },
                    ident: `new`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            InsSort(
                                ExpectInsSortResolvedOk {
                                    destination: Resolved(
                                        ReducedTerm(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Ref TermLiteral::EvalLifetime RawContour`),
            ),
            LocalTerm::Resolved(
                Term(`f32`),
            ),
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 3,
                            expectee: Resolved(
                                ReducedTerm(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: Some(
            Term(`LineSegmentSketch`),
        ),
    },
]
[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`core::num::i8`, `Alien`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                TypePath(`core::num::i16`, `Alien`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                TypePath(`core::num::i32`, `Alien`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                TypePath(`core::num::i64`, `Alien`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                TypePath(`core::num::f8`, `Alien`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                TypePath(`core::num::f16`, `Alien`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                TypePath(`core::num::f32`, `Alien`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                TypePath(`core::num::f64`, `Alien`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                    module_path: `core::num`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`core::num::i8`, `Alien`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                        module_path: `core::num`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`core::num::i8`, `Alien`),
                        },
                    },
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                    module_path: `core::num`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`core::num::i16`, `Alien`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                        module_path: `core::num`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`core::num::i16`, `Alien`),
                        },
                    },
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                    module_path: `core::num`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`core::num::i32`, `Alien`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                        module_path: `core::num`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`core::num::i32`, `Alien`),
                        },
                    },
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                    module_path: `core::num`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`core::num::i64`, `Alien`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                        module_path: `core::num`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`core::num::i64`, `Alien`),
                        },
                    },
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                    module_path: `core::num`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`core::num::f8`, `Alien`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                        module_path: `core::num`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`core::num::f8`, `Alien`),
                        },
                    },
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                    module_path: `core::num`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`core::num::f16`, `Alien`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                        module_path: `core::num`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`core::num::f16`, `Alien`),
                        },
                    },
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                    module_path: `core::num`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`core::num::f32`, `Alien`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                        module_path: `core::num`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`core::num::f32`, `Alien`),
                        },
                    },
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                    module_path: `core::num`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`core::num::f64`, `Alien`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
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
                        module_path: `core::num`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`core::num::f64`, `Alien`),
                        },
                    },
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
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
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
]
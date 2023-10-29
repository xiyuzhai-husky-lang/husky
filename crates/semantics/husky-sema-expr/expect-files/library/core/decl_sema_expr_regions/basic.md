[
    SemaExprRegion {
        [salsa id]: 1,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::basic::bool`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: RegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::bool`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [],
                },
            ),
            sema_expr_roots: [],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: None,
        },
    },
    SemaExprRegion {
        [salsa id]: 2,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::basic::never`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: RegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::never`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [],
                },
            ),
            sema_expr_roots: [],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: None,
        },
    },
    SemaExprRegion {
        [salsa id]: 3,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::basic::unit`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: RegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::unit`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [],
                },
            ),
            sema_expr_roots: [],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: None,
        },
    },
    SemaExprRegion {
        [salsa id]: 4,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::basic::Trait`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: RegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::Trait`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [],
                },
            ),
            sema_expr_roots: [],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: None,
        },
    },
    SemaExprRegion {
        [salsa id]: 5,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::basic::Module`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: RegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::Module`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            Id {
                                value: 6,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [],
                },
            ),
            sema_expr_roots: [],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: None,
        },
    },
    SemaExprRegion {
        [salsa id]: 6,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::basic::Lifetime`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: RegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::Lifetime`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            Id {
                                value: 7,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [],
                },
            ),
            sema_expr_roots: [],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: None,
        },
    },
    SemaExprRegion {
        [salsa id]: 7,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::basic::Place`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: RegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::Place`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [],
                },
            ),
            sema_expr_roots: [],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: None,
        },
    },
]
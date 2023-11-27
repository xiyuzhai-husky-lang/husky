[
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::Add`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Add`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 57,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::AddAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::AddAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 58,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::BitAnd`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitAnd`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 59,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::BitAndAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitAndAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 60,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::BitOr`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitOr`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 61,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::BitOrAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitOrAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 62,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::BitXor`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitXor`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 63,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::BitXorAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitXorAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 64,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::Div`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Div`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 65,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::DivAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::DivAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 66,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::IntIndex`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::IntIndex`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 67,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::Mul`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Mul`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 68,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::MulAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::MulAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 69,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::Neg`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Neg`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 70,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::Not`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Not`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 71,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::Sub`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Sub`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Rhs`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 72,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::Unveil`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Unveil`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `T`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 73,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Symbol(
                    EtherealTermSymbol(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Type(
                                    TypeSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `R`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    8,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `C`,
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                2..3,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 74,
                                },
                            ),
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
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
            self_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 31,
                        },
                    ),
                ),
            ),
        },
    },
]
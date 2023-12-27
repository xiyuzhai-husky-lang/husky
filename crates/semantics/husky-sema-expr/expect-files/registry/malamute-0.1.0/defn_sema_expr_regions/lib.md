[
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TraitForTypeItem(
                                    TraitForTypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitForTypeItemPath(
                                                ItemPathId {
                                                    data: ItemPathData::AssociatedItem(
                                                        AssociatedItemPathData::TraitForTypeItem(
                                                            TraitForTypeItemPathData {
                                                                impl_block: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::default::Default`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `default`,
                                                                item_kind: AssociatedFunctionFn,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: Some(
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: SynNodeRegionPath::Decl(
                                            ItemSynNodePath::ImplBlock(
                                                ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                    TraitForTypeImplBlockSynNodePath(
                                                        ItemSynNodePathId {
                                                            data: ItemSynNodePathData::ImplBlock(
                                                                ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                    TraitForTypeImplBlockSynNodePathData {
                                                                        path: TraitForTypeImplBlock {
                                                                            data: TraitForTypeImplBlockPathData {
                                                                                module_path: `malamute`,
                                                                                trai_path: TraitPath(`core::default::Default`),
                                                                                ty_sketch: TypeSketch::Path(
                                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                                ),
                                                                                disambiguator: 0,
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        syn_expr_arena: Arena {
                                            data: [
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Trait(
                                                                TraitPath(`core::default::Default`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 3,
                                                    argument_expr_idx: 4,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        18,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 5,
                                                    argument_expr_idx: 6,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `Default`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                14,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::default::Default`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                            ],
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
                                                                syn_attrs: [
                                                                    Phantom(
                                                                        PoundRegionalToken(
                                                                            RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        PhantomRegionalToken {
                                                                            token_idx: RegionalTokenIdx(
                                                                                4,
                                                                            ),
                                                                        },
                                                                    ),
                                                                ],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `Label`,
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
                                                            13,
                                                        ),
                                                        access_end: None,
                                                        data: CurrentSynSymbolData::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [
                                                                    Phantom(
                                                                        PoundRegionalToken(
                                                                            RegionalTokenIdx(
                                                                                7,
                                                                            ),
                                                                        ),
                                                                        PhantomRegionalToken {
                                                                            token_idx: RegionalTokenIdx(
                                                                                8,
                                                                            ),
                                                                        },
                                                                    ),
                                                                ],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `label`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        10,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 1,
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
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::ConstantImplicitParameterType,
                                                syn_expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Trait,
                                                syn_expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::SelfType,
                                                syn_expr_idx: 7,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [],
                                    },
                                },
                            ),
                            path: SynNodeRegionPath::Decl(
                                ItemSynNodePath::AssociatedItem(
                                    AssociatedItemSynNodePath::TraitForTypeItem(
                                        TraitForTypeItemSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::AssociatedItem(
                                                    AssociatedItemSynNodePathData::TraitForTypeItem(
                                                        TraitForTypeItemSynNodePathData {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitForTypeItemPath(
                                                                    ItemPathId {
                                                                        data: ItemPathData::AssociatedItem(
                                                                            AssociatedItemPathData::TraitForTypeItem(
                                                                                TraitForTypeItemPathData {
                                                                                    impl_block: TraitForTypeImplBlock {
                                                                                        data: TraitForTypeImplBlockPathData {
                                                                                            module_path: `malamute`,
                                                                                            trai_path: TraitPath(`core::default::Default`),
                                                                                            ty_sketch: TypeSketch::Path(
                                                                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                                                                            ),
                                                                                            disambiguator: 0,
                                                                                        },
                                                                                    },
                                                                                    ident: `default`,
                                                                                    item_kind: AssociatedFunctionFn,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
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
                            syn_expr_arena: Arena {
                                data: [
                                    SynExprData::SelfType(
                                        RegionalTokenIdx(
                                            7,
                                        ),
                                    ),
                                ],
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
                                    data: [
                                        InheritedSynSymbol {
                                            parent_symbol_idx: Current(
                                                1,
                                            ),
                                            modifier: Const,
                                            kind: InheritedSynSymbolKind::TemplateParameter(
                                                InheritedTemplateParameterSynSymbol::Type {
                                                    ident: `Label`,
                                                },
                                            ),
                                        },
                                        InheritedSynSymbol {
                                            parent_symbol_idx: Current(
                                                2,
                                            ),
                                            modifier: Const,
                                            kind: InheritedSynSymbolKind::TemplateParameter(
                                                InheritedTemplateParameterSynSymbol::Constant {
                                                    ident: `label`,
                                                },
                                            ),
                                        },
                                    ],
                                },
                                current_syn_symbol_arena: Arena {
                                    data: [],
                                },
                                allow_self_type: True,
                                allow_self_value: True,
                                pattern_ty_constraints: [],
                            },
                            syn_pattern_expr_roots: [],
                            syn_expr_roots: [
                                SynExprRoot {
                                    kind: SynExprRootKind::ReturnType,
                                    syn_expr_idx: 1,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [],
                        },
                    },
                ),
                path: SynNodeRegionPath::Defn(
                    ItemSynNodePath::AssociatedItem(
                        AssociatedItemSynNodePath::TraitForTypeItem(
                            TraitForTypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TraitForTypeItem(
                                            TraitForTypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath(
                                                        ItemPathId {
                                                            data: ItemPathData::AssociatedItem(
                                                                AssociatedItemPathData::TraitForTypeItem(
                                                                    TraitForTypeItemPathData {
                                                                        impl_block: TraitForTypeImplBlock {
                                                                            data: TraitForTypeImplBlockPathData {
                                                                                module_path: `malamute`,
                                                                                trai_path: TraitPath(`core::default::Default`),
                                                                                ty_sketch: TypeSketch::Path(
                                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                                ),
                                                                                disambiguator: 0,
                                                                            },
                                                                        },
                                                                        ident: `default`,
                                                                        item_kind: AssociatedFunctionFn,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
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
                syn_expr_arena: Arena {
                    data: [
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ident: `No`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAll`,
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 1,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    2,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `No`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ident: `No`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 1,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
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
                        data: [
                            InheritedSynSymbol {
                                parent_symbol_idx: Current(
                                    1,
                                ),
                                modifier: Const,
                                kind: InheritedSynSymbolKind::TemplateParameter(
                                    InheritedTemplateParameterSynSymbol::Type {
                                        ident: `Label`,
                                    },
                                ),
                            },
                            InheritedSynSymbol {
                                parent_symbol_idx: Current(
                                    2,
                                ),
                                modifier: Const,
                                kind: InheritedSynSymbolKind::TemplateParameter(
                                    InheritedTemplateParameterSynSymbol::Constant {
                                        ident: `label`,
                                    },
                                ),
                            },
                        ],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 1,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::BlockExpr,
                        syn_expr_idx: 2,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                AssociatedItem(
                    TraitForTypeItem(
                        TraitForTypeItemSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 461,
                                },
                            ),
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 469,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    2,
                    (
                        SemaExprIdx(
                            2,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
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
                                    place: Some(
                                        Const,
                                    ),
                                    base: Ethereal(
                                        Symbol(
                                            EtherealTermSymbol(
                                                Id {
                                                    value: 7,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 7,
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
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
                        entries: [
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        1,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        1,
                                    ),
                                    hole_kind: Any,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: TypeOntology {
                                    path: TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 467,
                                            },
                                        ),
                                    ),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 467,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 2,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 87,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expectation(
                                            1,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expectation(
                                            1,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 87,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 87,
                        },
                    ),
                ),
            ),
            self_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 87,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TraitForTypeItem(
                                    TraitForTypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitForTypeItemPath(
                                                ItemPathId {
                                                    data: ItemPathData::AssociatedItem(
                                                        AssociatedItemPathData::TraitForTypeItem(
                                                            TraitForTypeItemPathData {
                                                                impl_block: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`malamute::Class`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `unveil`,
                                                                item_kind: AssociatedFunctionFn,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: Some(
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: SynNodeRegionPath::Decl(
                                            ItemSynNodePath::ImplBlock(
                                                ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                    TraitForTypeImplBlockSynNodePath(
                                                        ItemSynNodePathId {
                                                            data: ItemSynNodePathData::ImplBlock(
                                                                ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                    TraitForTypeImplBlockSynNodePathData {
                                                                        path: TraitForTypeImplBlock {
                                                                            data: TraitForTypeImplBlockPathData {
                                                                                module_path: `malamute`,
                                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                                ty_sketch: TypeSketch::Path(
                                                                                    TypePath(`malamute::Class`, `Enum`),
                                                                                ),
                                                                                disambiguator: 0,
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        syn_expr_arena: Arena {
                                            data: [
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    3,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 3,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Trait(
                                                                TraitPath(`core::ops::Unveil`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 4,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 2,
                                                    argument_expr_idx: 3,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        18,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    3,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 4,
                                                    argument_expr_idx: 5,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    8,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 6,
                                                    argument_expr_idx: 7,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 5,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`malamute::Class`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    3,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 9,
                                                    argument_expr_idx: 10,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `core`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::Module(
                                                        `core`,
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Subitem {
                                                    parent: 1,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `ops`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                14,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        PrincipalEntityPath::Module(
                                                            `core::ops`,
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Subitem {
                                                    parent: 2,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            15,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Unveil`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Trait(
                                                                TraitPath(`core::ops::Unveil`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `Class`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                21,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::Class`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                            ],
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
                                                            4,
                                                        ),
                                                        access_end: None,
                                                        data: CurrentSynSymbolData::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `Label`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                        access_end: None,
                                                        data: CurrentSynSymbolData::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [
                                                                    Runtime(
                                                                        PoundRegionalToken(
                                                                            RegionalTokenIdx(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        RuntimeRegionalToken {
                                                                            token_idx: RegionalTokenIdx(
                                                                                6,
                                                                            ),
                                                                        },
                                                                    ),
                                                                ],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `label`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        8,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 1,
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
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::ConstantImplicitParameterType,
                                                syn_expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Trait,
                                                syn_expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::SelfType,
                                                syn_expr_idx: 11,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [],
                                    },
                                },
                            ),
                            path: SynNodeRegionPath::Decl(
                                ItemSynNodePath::AssociatedItem(
                                    AssociatedItemSynNodePath::TraitForTypeItem(
                                        TraitForTypeItemSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::AssociatedItem(
                                                    AssociatedItemSynNodePathData::TraitForTypeItem(
                                                        TraitForTypeItemSynNodePathData {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitForTypeItemPath(
                                                                    ItemPathId {
                                                                        data: ItemPathData::AssociatedItem(
                                                                            AssociatedItemPathData::TraitForTypeItem(
                                                                                TraitForTypeItemPathData {
                                                                                    impl_block: TraitForTypeImplBlock {
                                                                                        data: TraitForTypeImplBlockPathData {
                                                                                            module_path: `malamute`,
                                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                                            ty_sketch: TypeSketch::Path(
                                                                                                TypePath(`malamute::Class`, `Enum`),
                                                                                            ),
                                                                                            disambiguator: 0,
                                                                                        },
                                                                                    },
                                                                                    ident: `unveil`,
                                                                                    item_kind: AssociatedFunctionFn,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
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
                            syn_expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::InheritedSynSymbol {
                                        ident: `Label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                        inherited_syn_symbol_idx: 1,
                                        inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                            InheritedTemplateParameterSynSymbol::Type {
                                                ident: `Label`,
                                            },
                                        ),
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 1,
                                        argument_expr_idx: 2,
                                    },
                                    SynExprData::InheritedSynSymbol {
                                        ident: `label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                        inherited_syn_symbol_idx: 2,
                                        inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                            InheritedTemplateParameterSynSymbol::Constant {
                                                ident: `label`,
                                            },
                                        ),
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 3,
                                        argument_expr_idx: 4,
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 4,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::ops::ControlFlow`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::SelfType(
                                        RegionalTokenIdx(
                                            17,
                                        ),
                                    ),
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 6,
                                        argument_expr_idx: 7,
                                    },
                                    SynExprData::FunctionApplicationOrCall {
                                        function: 8,
                                        template_arguments: None,
                                        lpar_regional_token_idx: RegionalTokenIdx(
                                            18,
                                        ),
                                        items: [],
                                        rpar_regional_token_idx: RegionalTokenIdx(
                                            19,
                                        ),
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `OneVsAll`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `core`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::Module(
                                            `core`,
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Subitem {
                                        parent: 2,
                                        colon_colon_token: ColonColonRegionalToken(
                                            RegionalTokenIdx(
                                                13,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `ops`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                            },
                                        ),
                                        path: Ok(
                                            PrincipalEntityPath::Module(
                                                `core::ops`,
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Subitem {
                                        parent: 3,
                                        colon_colon_token: ColonColonRegionalToken(
                                            RegionalTokenIdx(
                                                15,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `ControlFlow`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                            },
                                        ),
                                        path: Ok(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::ops::ControlFlow`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: SynPatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [
                                        SynPatternExprData::Ident {
                                            symbol_modifier_tokens: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `one_vs_all`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                        },
                                    ],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [
                                        Pure,
                                    ],
                                },
                                pattern_symbol_arena: Arena {
                                    data: [
                                        SynPatternSymbol::Atom(
                                            1,
                                        ),
                                    ],
                                },
                                pattern_symbol_maps: [
                                    [
                                        (
                                            `one_vs_all`,
                                            1,
                                        ),
                                    ],
                                ],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [
                                        Pure,
                                    ],
                                },
                            },
                            symbol_region: SynSymbolRegionData {
                                inherited_syn_symbol_arena: Arena {
                                    data: [
                                        InheritedSynSymbol {
                                            parent_symbol_idx: Current(
                                                1,
                                            ),
                                            modifier: Const,
                                            kind: InheritedSynSymbolKind::TemplateParameter(
                                                InheritedTemplateParameterSynSymbol::Type {
                                                    ident: `Label`,
                                                },
                                            ),
                                        },
                                        InheritedSynSymbol {
                                            parent_symbol_idx: Current(
                                                2,
                                            ),
                                            modifier: Const,
                                            kind: InheritedSynSymbolKind::TemplateParameter(
                                                InheritedTemplateParameterSynSymbol::Constant {
                                                    ident: `label`,
                                                },
                                            ),
                                        },
                                    ],
                                },
                                current_syn_symbol_arena: Arena {
                                    data: [
                                        CurrentSynSymbol {
                                            modifier: Pure,
                                            access_start: RegionalTokenIdx(
                                                6,
                                            ),
                                            access_end: None,
                                            data: CurrentSynSymbolData::ParenateRegularParameter {
                                                ident: `one_vs_all`,
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                    ],
                                },
                                allow_self_type: True,
                                allow_self_value: True,
                                pattern_ty_constraints: [
                                    (
                                        OrdinaryParenateParameter {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 1,
                                            },
                                            ty_expr_idx: 5,
                                        },
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                ],
                            },
                            syn_pattern_expr_roots: [
                                SynPatternExprRoot {
                                    kind: SynPatternExprRootKind::Parenate,
                                    syn_pattern_expr_idx: 1,
                                },
                            ],
                            syn_expr_roots: [
                                SynExprRoot {
                                    kind: SynExprRootKind::ExplicitParameterType,
                                    syn_expr_idx: 5,
                                },
                                SynExprRoot {
                                    kind: SynExprRootKind::ReturnType,
                                    syn_expr_idx: 9,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [
                                (
                                    1,
                                    1,
                                ),
                            ],
                        },
                    },
                ),
                path: SynNodeRegionPath::Defn(
                    ItemSynNodePath::AssociatedItem(
                        AssociatedItemSynNodePath::TraitForTypeItem(
                            TraitForTypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TraitForTypeItem(
                                            TraitForTypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath(
                                                        ItemPathId {
                                                            data: ItemPathData::AssociatedItem(
                                                                AssociatedItemPathData::TraitForTypeItem(
                                                                    TraitForTypeItemPathData {
                                                                        impl_block: TraitForTypeImplBlock {
                                                                            data: TraitForTypeImplBlockPathData {
                                                                                module_path: `malamute`,
                                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                                ty_sketch: TypeSketch::Path(
                                                                                    TypePath(`malamute::Class`, `Enum`),
                                                                                ),
                                                                                disambiguator: 0,
                                                                            },
                                                                        },
                                                                        ident: `unveil`,
                                                                        item_kind: AssociatedFunctionFn,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
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
                syn_expr_arena: Arena {
                    data: [
                        SynExprData::InheritedSynSymbol {
                            ident: `one_vs_all`,
                            regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            inherited_syn_symbol_idx: 3,
                            inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                ident: `one_vs_all`,
                            },
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 6,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    ident: `Break`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 8,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                                                    ident: `Known`,
                                                    index: U8(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynExprData::InheritedSynSymbol {
                            ident: `label`,
                            regional_token_idx: RegionalTokenIdx(
                                21,
                            ),
                            inherited_syn_symbol_idx: 2,
                            inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                InheritedTemplateParameterSynSymbol::Constant {
                                    ident: `label`,
                                },
                            ),
                        },
                        SynExprData::FunctionApplicationOrCall {
                            function: 3,
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                20,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 4,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                22,
                            ),
                        },
                        SynExprData::FunctionApplicationOrCall {
                            function: 2,
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                16,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 5,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                23,
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 14,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    ident: `Continue`,
                                                    index: U8(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Unit {
                            lpar_regional_token_idx: RegionalTokenIdx(
                                37,
                            ),
                            rpar_regional_token_idx: RegionalTokenIdx(
                                38,
                            ),
                        },
                        SynExprData::FunctionApplicationOrCall {
                            function: 7,
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                36,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 8,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                39,
                            ),
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                3..4,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAll`,
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 1,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    6,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Yes`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ident: `Yes`,
                                                    index: U8(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `core`,
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::Module(
                                `core`,
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 3,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    10,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ops`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::Module(
                                    `core::ops`,
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 4,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    12,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ControlFlow`,
                                    regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::ops::ControlFlow`, `Enum`),
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 5,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    14,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Break`,
                                    regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    ident: `Break`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `Class`,
                                    regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::Class`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 7,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    18,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Known`,
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                                                    ident: `Known`,
                                                    index: U8(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAll`,
                                    regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 9,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    26,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `No`,
                                    regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ident: `No`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `core`,
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::Module(
                                `core`,
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 11,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    30,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ops`,
                                    regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::Module(
                                    `core::ops`,
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 12,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    32,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ControlFlow`,
                                    regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::ops::ControlFlow`, `Enum`),
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 13,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    34,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Continue`,
                                    regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    ident: `Continue`,
                                                    index: U8(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 6,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                        SynStmtData::Eval {
                            expr_idx: 9,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                        SynStmtData::Match {
                            match_token: MatchRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    1,
                                ),
                            },
                            match_expr: Ok(
                                1,
                            ),
                            eol_with_token: Ok(
                                EolWithRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            ),
                            case_branches: [
                                SynCaseBranch {
                                    vertical_token: VerticalRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    case_pattern_syn_obelisk: Ok(
                                        CasePatternSyndicate {
                                            syn_pattern_root: CaseSynPatternExprRoot {
                                                syn_pattern_expr_idx: 1,
                                            },
                                            variables: ArenaIdxRange(
                                                1..1,
                                            ),
                                        },
                                    ),
                                    heavy_arrow_token: Ok(
                                        HeavyArrowRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                    ),
                                    stmts: Ok(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                                SynCaseBranch {
                                    vertical_token: VerticalRegionalToken(
                                        RegionalTokenIdx(
                                            24,
                                        ),
                                    ),
                                    case_pattern_syn_obelisk: Ok(
                                        CasePatternSyndicate {
                                            syn_pattern_root: CaseSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
                                            variables: ArenaIdxRange(
                                                1..1,
                                            ),
                                        },
                                    ),
                                    heavy_arrow_token: Ok(
                                        HeavyArrowRegionalToken(
                                            RegionalTokenIdx(
                                                28,
                                            ),
                                        ),
                                    ),
                                    stmts: Ok(
                                        ArenaIdxRange(
                                            2..3,
                                        ),
                                    ),
                                },
                            ],
                        },
                    ],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [
                            SynPatternExprData::UnitTypeVariant {
                                path_expr_idx: 2,
                                path: TypeVariantPath(
                                    ItemPathId {
                                        data: ItemPathData::TypeVariant(
                                            TypeVariantPathData {
                                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                ident: `Yes`,
                                                index: U8(
                                                    0,
                                                ),
                                            },
                                        ),
                                    },
                                ),
                            },
                            SynPatternExprData::UnitTypeVariant {
                                path_expr_idx: 10,
                                path: TypeVariantPath(
                                    ItemPathId {
                                        data: ItemPathData::TypeVariant(
                                            TypeVariantPathData {
                                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                ident: `No`,
                                                index: U8(
                                                    1,
                                                ),
                                            },
                                        ),
                                    },
                                ),
                            },
                        ],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [
                            Pure,
                            Pure,
                        ],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [
                        [],
                        [],
                    ],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [
                            InheritedSynSymbol {
                                parent_symbol_idx: Current(
                                    1,
                                ),
                                modifier: Const,
                                kind: InheritedSynSymbolKind::TemplateParameter(
                                    InheritedTemplateParameterSynSymbol::Type {
                                        ident: `Label`,
                                    },
                                ),
                            },
                            InheritedSynSymbol {
                                parent_symbol_idx: Current(
                                    2,
                                ),
                                modifier: Const,
                                kind: InheritedSynSymbolKind::TemplateParameter(
                                    InheritedTemplateParameterSynSymbol::Constant {
                                        ident: `label`,
                                    },
                                ),
                            },
                            InheritedSynSymbol {
                                parent_symbol_idx: Current(
                                    1,
                                ),
                                modifier: Pure,
                                kind: InheritedSynSymbolKind::ParenateParameter {
                                    ident: `one_vs_all`,
                                },
                            },
                        ],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Case,
                        syn_pattern_expr_idx: 1,
                    },
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Case,
                        syn_pattern_expr_idx: 2,
                    },
                ],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 6,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 9,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::BlockExpr,
                        syn_expr_idx: 10,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                AssociatedItem(
                    TraitForTypeItem(
                        TraitForTypeItemSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 463,
                                },
                            ),
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                InheritedSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 540,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    inherited_syn_symbol_idx: 3,
                                    inherited_syn_symbol_kind: ParenateParameter {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 540,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 96,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 96,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 6,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 74,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 9,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 8,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 465,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    4,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                InheritedSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 537,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: TemplateParameter(
                                        Constant {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 537,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: Ethereal(
                                        Symbol(
                                            EtherealTermSymbol(
                                                Id {
                                                    value: 1,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: Ethereal(
                                            Symbol(
                                                EtherealTermSymbol(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    4,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Const,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                5,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    5,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 14,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 73,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    7,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 9,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    8,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            10,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                10,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Unit {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 23,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            8,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    8,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            3..4,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        6,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Match {
                                    match_token: MatchRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    match_target: SemaExprIdx(
                                        1,
                                    ),
                                    eol_with_token: EolWithRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    case_branches: [
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                            case_pattern_sema_obelisk: CasePatternSemaSyndicate {
                                                syn_pattern_root: CaseSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..1,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    8,
                                                ),
                                            ),
                                            stmts: SemaStmtIdxRange(
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        },
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    24,
                                                ),
                                            ),
                                            case_pattern_sema_obelisk: CasePatternSemaSyndicate {
                                                syn_pattern_root: CaseSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 2,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..1,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    28,
                                                ),
                                            ),
                                            stmts: SemaStmtIdxRange(
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    10,
                    (
                        SemaExprIdx(
                            10,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 96,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 96,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
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
                                    place: Some(
                                        Const,
                                    ),
                                    base: Ethereal(
                                        Symbol(
                                            EtherealTermSymbol(
                                                Id {
                                                    value: 1,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 96,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 1,
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
                                                value: 4,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                        None,
                    ],
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
                        entries: [
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        2,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    5,
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 88,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        2,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 23,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: TypeOntology {
                                    path: TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 72,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 90,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: ResolvedEthereal(
                                    Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 50,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        3,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: Some(
                                                Const,
                                            ),
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleFrom {
                                            target: FluffyTerm {
                                                place: Some(
                                                    Const,
                                                ),
                                                base: Ethereal(
                                                    Symbol(
                                                        EtherealTermSymbol(
                                                            Id {
                                                                value: 1,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: TypeOntology {
                                    path: TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 464,
                                            },
                                        ),
                                    ),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 464,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 88,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                5,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: ResolvedEthereal(
                                    Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 49,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        7,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 88,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 88,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 88,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        7,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 23,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: TypeOntology {
                                    path: TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 72,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    7,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    8,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 90,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            8,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: ResolvedEthereal(
                                    Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 52,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 9,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 96,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: Hollow(
                                                            HollowTerm(
                                                                2,
                                                            ),
                                                        ),
                                                    },
                                                    variant: Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: Hollow(
                                                            HollowTerm(
                                                                5,
                                                            ),
                                                        ),
                                                    },
                                                    variant: Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                4,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    4,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: Ethereal(
                                            Symbol(
                                                EtherealTermSymbol(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                5,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 90,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 88,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expectation(
                                            6,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expectation(
                                            6,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                10,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: Hollow(
                                                            HollowTerm(
                                                                9,
                                                            ),
                                                        ),
                                                    },
                                                    variant: Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                8,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    8,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 23,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 90,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 88,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expectation(
                                            11,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expectation(
                                            11,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                8,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 90,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 90,
                        },
                    ),
                ),
            ),
            self_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 88,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TraitForTypeItem(
                                    TraitForTypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitForTypeItemPath(
                                                ItemPathId {
                                                    data: ItemPathData::AssociatedItem(
                                                        AssociatedItemPathData::TraitForTypeItem(
                                                            TraitForTypeItemPathData {
                                                                impl_block: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `unveil`,
                                                                item_kind: AssociatedFunctionFn,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: Some(
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: SynNodeRegionPath::Decl(
                                            ItemSynNodePath::ImplBlock(
                                                ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                    TraitForTypeImplBlockSynNodePath(
                                                        ItemSynNodePathId {
                                                            data: ItemSynNodePathData::ImplBlock(
                                                                ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                    TraitForTypeImplBlockSynNodePathData {
                                                                        path: TraitForTypeImplBlock {
                                                                            data: TraitForTypeImplBlockPathData {
                                                                                module_path: `malamute`,
                                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                                ty_sketch: TypeSketch::Path(
                                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                                ),
                                                                                disambiguator: 0,
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        syn_expr_arena: Arena {
                                            data: [
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 3,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Trait(
                                                                TraitPath(`core::ops::Unveil`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 4,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 2,
                                                    argument_expr_idx: 3,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 4,
                                                    argument_expr_idx: 5,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 6,
                                                    argument_expr_idx: 7,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 5,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 9,
                                                    argument_expr_idx: 10,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        25,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 11,
                                                    argument_expr_idx: 12,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `core`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                14,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::Module(
                                                        `core`,
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Subitem {
                                                    parent: 1,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            15,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `ops`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        PrincipalEntityPath::Module(
                                                            `core::ops`,
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Subitem {
                                                    parent: 2,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            17,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Unveil`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Trait(
                                                                TraitPath(`core::ops::Unveil`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `OneVsAllResult`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                19,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                23,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                            ],
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
                                                                syn_attrs: [
                                                                    Phantom(
                                                                        PoundRegionalToken(
                                                                            RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        PhantomRegionalToken {
                                                                            token_idx: RegionalTokenIdx(
                                                                                4,
                                                                            ),
                                                                        },
                                                                    ),
                                                                ],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `Label`,
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
                                                            13,
                                                        ),
                                                        access_end: None,
                                                        data: CurrentSynSymbolData::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [
                                                                    Phantom(
                                                                        PoundRegionalToken(
                                                                            RegionalTokenIdx(
                                                                                7,
                                                                            ),
                                                                        ),
                                                                        PhantomRegionalToken {
                                                                            token_idx: RegionalTokenIdx(
                                                                                8,
                                                                            ),
                                                                        },
                                                                    ),
                                                                ],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `label`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        10,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 1,
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
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::ConstantImplicitParameterType,
                                                syn_expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Trait,
                                                syn_expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::SelfType,
                                                syn_expr_idx: 13,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [],
                                    },
                                },
                            ),
                            path: SynNodeRegionPath::Decl(
                                ItemSynNodePath::AssociatedItem(
                                    AssociatedItemSynNodePath::TraitForTypeItem(
                                        TraitForTypeItemSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::AssociatedItem(
                                                    AssociatedItemSynNodePathData::TraitForTypeItem(
                                                        TraitForTypeItemSynNodePathData {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitForTypeItemPath(
                                                                    ItemPathId {
                                                                        data: ItemPathData::AssociatedItem(
                                                                            AssociatedItemPathData::TraitForTypeItem(
                                                                                TraitForTypeItemPathData {
                                                                                    impl_block: TraitForTypeImplBlock {
                                                                                        data: TraitForTypeImplBlockPathData {
                                                                                            module_path: `malamute`,
                                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                                            ty_sketch: TypeSketch::Path(
                                                                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                                                                            ),
                                                                                            disambiguator: 0,
                                                                                        },
                                                                                    },
                                                                                    ident: `unveil`,
                                                                                    item_kind: AssociatedFunctionFn,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
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
                            syn_expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::InheritedSynSymbol {
                                        ident: `Label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                        inherited_syn_symbol_idx: 1,
                                        inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                            InheritedTemplateParameterSynSymbol::Type {
                                                ident: `Label`,
                                            },
                                        ),
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 1,
                                        argument_expr_idx: 2,
                                    },
                                    SynExprData::InheritedSynSymbol {
                                        ident: `label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                        inherited_syn_symbol_idx: 2,
                                        inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                            InheritedTemplateParameterSynSymbol::Constant {
                                                ident: `label`,
                                            },
                                        ),
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 3,
                                        argument_expr_idx: 4,
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 4,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::ops::ControlFlow`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::SelfType(
                                        RegionalTokenIdx(
                                            17,
                                        ),
                                    ),
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 6,
                                        argument_expr_idx: 7,
                                    },
                                    SynExprData::FunctionApplicationOrCall {
                                        function: 8,
                                        template_arguments: None,
                                        lpar_regional_token_idx: RegionalTokenIdx(
                                            18,
                                        ),
                                        items: [],
                                        rpar_regional_token_idx: RegionalTokenIdx(
                                            19,
                                        ),
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `OneVsAllResult`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `core`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::Module(
                                            `core`,
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Subitem {
                                        parent: 2,
                                        colon_colon_token: ColonColonRegionalToken(
                                            RegionalTokenIdx(
                                                13,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `ops`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                            },
                                        ),
                                        path: Ok(
                                            PrincipalEntityPath::Module(
                                                `core::ops`,
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Subitem {
                                        parent: 3,
                                        colon_colon_token: ColonColonRegionalToken(
                                            RegionalTokenIdx(
                                                15,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `ControlFlow`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                            },
                                        ),
                                        path: Ok(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::ops::ControlFlow`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: SynPatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [
                                        SynPatternExprData::Ident {
                                            symbol_modifier_tokens: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `one_vs_all_result`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                        },
                                    ],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [
                                        Pure,
                                    ],
                                },
                                pattern_symbol_arena: Arena {
                                    data: [
                                        SynPatternSymbol::Atom(
                                            1,
                                        ),
                                    ],
                                },
                                pattern_symbol_maps: [
                                    [
                                        (
                                            `one_vs_all_result`,
                                            1,
                                        ),
                                    ],
                                ],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [
                                        Pure,
                                    ],
                                },
                            },
                            symbol_region: SynSymbolRegionData {
                                inherited_syn_symbol_arena: Arena {
                                    data: [
                                        InheritedSynSymbol {
                                            parent_symbol_idx: Current(
                                                1,
                                            ),
                                            modifier: Const,
                                            kind: InheritedSynSymbolKind::TemplateParameter(
                                                InheritedTemplateParameterSynSymbol::Type {
                                                    ident: `Label`,
                                                },
                                            ),
                                        },
                                        InheritedSynSymbol {
                                            parent_symbol_idx: Current(
                                                2,
                                            ),
                                            modifier: Const,
                                            kind: InheritedSynSymbolKind::TemplateParameter(
                                                InheritedTemplateParameterSynSymbol::Constant {
                                                    ident: `label`,
                                                },
                                            ),
                                        },
                                    ],
                                },
                                current_syn_symbol_arena: Arena {
                                    data: [
                                        CurrentSynSymbol {
                                            modifier: Pure,
                                            access_start: RegionalTokenIdx(
                                                6,
                                            ),
                                            access_end: None,
                                            data: CurrentSynSymbolData::ParenateRegularParameter {
                                                ident: `one_vs_all_result`,
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                    ],
                                },
                                allow_self_type: True,
                                allow_self_value: True,
                                pattern_ty_constraints: [
                                    (
                                        OrdinaryParenateParameter {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 1,
                                            },
                                            ty_expr_idx: 5,
                                        },
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                ],
                            },
                            syn_pattern_expr_roots: [
                                SynPatternExprRoot {
                                    kind: SynPatternExprRootKind::Parenate,
                                    syn_pattern_expr_idx: 1,
                                },
                            ],
                            syn_expr_roots: [
                                SynExprRoot {
                                    kind: SynExprRootKind::ExplicitParameterType,
                                    syn_expr_idx: 5,
                                },
                                SynExprRoot {
                                    kind: SynExprRootKind::ReturnType,
                                    syn_expr_idx: 9,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [
                                (
                                    1,
                                    1,
                                ),
                            ],
                        },
                    },
                ),
                path: SynNodeRegionPath::Defn(
                    ItemSynNodePath::AssociatedItem(
                        AssociatedItemSynNodePath::TraitForTypeItem(
                            TraitForTypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TraitForTypeItem(
                                            TraitForTypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath(
                                                        ItemPathId {
                                                            data: ItemPathData::AssociatedItem(
                                                                AssociatedItemPathData::TraitForTypeItem(
                                                                    TraitForTypeItemPathData {
                                                                        impl_block: TraitForTypeImplBlock {
                                                                            data: TraitForTypeImplBlockPathData {
                                                                                module_path: `malamute`,
                                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                                ty_sketch: TypeSketch::Path(
                                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                                ),
                                                                                disambiguator: 0,
                                                                            },
                                                                        },
                                                                        ident: `unveil`,
                                                                        item_kind: AssociatedFunctionFn,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
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
                syn_expr_arena: Arena {
                    data: [
                        SynExprData::InheritedSynSymbol {
                            ident: `one_vs_all_result`,
                            regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            inherited_syn_symbol_idx: 3,
                            inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                ident: `one_vs_all_result`,
                            },
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 6,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    ident: `Break`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 8,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ident: `Yes`,
                                                    index: U8(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynExprData::FunctionApplicationOrCall {
                            function: 2,
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                16,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 3,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                20,
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 14,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    ident: `Break`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 16,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ident: `No`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynExprData::FunctionApplicationOrCall {
                            function: 5,
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                33,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 6,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                37,
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 22,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    ident: `Continue`,
                                                    index: U8(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Unit {
                            lpar_regional_token_idx: RegionalTokenIdx(
                                51,
                            ),
                            rpar_regional_token_idx: RegionalTokenIdx(
                                52,
                            ),
                        },
                        SynExprData::FunctionApplicationOrCall {
                            function: 8,
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                50,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 9,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                53,
                            ),
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                4..5,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAllResult`,
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 1,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    6,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ConfidentYes`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                    ident: `ConfidentYes`,
                                                    index: U8(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `core`,
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::Module(
                                `core`,
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 3,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    10,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ops`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::Module(
                                    `core::ops`,
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 4,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    12,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ControlFlow`,
                                    regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::ops::ControlFlow`, `Enum`),
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 5,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    14,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Break`,
                                    regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    ident: `Break`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAll`,
                                    regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 7,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    18,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Yes`,
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ident: `Yes`,
                                                    index: U8(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAllResult`,
                                    regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 9,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    23,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ConfidentNo`,
                                    regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                    ident: `ConfidentNo`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `core`,
                                    regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::Module(
                                `core`,
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 11,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    27,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ops`,
                                    regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::Module(
                                    `core::ops`,
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 12,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    29,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ControlFlow`,
                                    regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::ops::ControlFlow`, `Enum`),
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 13,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    31,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Break`,
                                    regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    ident: `Break`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAll`,
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 15,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    35,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `No`,
                                    regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ident: `No`,
                                                    index: U8(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAllResult`,
                                    regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 17,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    40,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Unconfident`,
                                    regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                    ident: `Unconfident`,
                                                    index: U8(
                                                        2,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `core`,
                                    regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::Module(
                                `core`,
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 19,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    44,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ops`,
                                    regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::Module(
                                    `core::ops`,
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 20,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    46,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `ControlFlow`,
                                    regional_token_idx: RegionalTokenIdx(
                                        47,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::ops::ControlFlow`, `Enum`),
                                    ),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 21,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    48,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Continue`,
                                    regional_token_idx: RegionalTokenIdx(
                                        49,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    ident: `Continue`,
                                                    index: U8(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 4,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                        SynStmtData::Eval {
                            expr_idx: 7,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                        SynStmtData::Eval {
                            expr_idx: 10,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                        SynStmtData::Match {
                            match_token: MatchRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    1,
                                ),
                            },
                            match_expr: Ok(
                                1,
                            ),
                            eol_with_token: Ok(
                                EolWithRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            ),
                            case_branches: [
                                SynCaseBranch {
                                    vertical_token: VerticalRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    case_pattern_syn_obelisk: Ok(
                                        CasePatternSyndicate {
                                            syn_pattern_root: CaseSynPatternExprRoot {
                                                syn_pattern_expr_idx: 1,
                                            },
                                            variables: ArenaIdxRange(
                                                1..1,
                                            ),
                                        },
                                    ),
                                    heavy_arrow_token: Ok(
                                        HeavyArrowRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                    ),
                                    stmts: Ok(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                                SynCaseBranch {
                                    vertical_token: VerticalRegionalToken(
                                        RegionalTokenIdx(
                                            21,
                                        ),
                                    ),
                                    case_pattern_syn_obelisk: Ok(
                                        CasePatternSyndicate {
                                            syn_pattern_root: CaseSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
                                            variables: ArenaIdxRange(
                                                1..1,
                                            ),
                                        },
                                    ),
                                    heavy_arrow_token: Ok(
                                        HeavyArrowRegionalToken(
                                            RegionalTokenIdx(
                                                25,
                                            ),
                                        ),
                                    ),
                                    stmts: Ok(
                                        ArenaIdxRange(
                                            2..3,
                                        ),
                                    ),
                                },
                                SynCaseBranch {
                                    vertical_token: VerticalRegionalToken(
                                        RegionalTokenIdx(
                                            38,
                                        ),
                                    ),
                                    case_pattern_syn_obelisk: Ok(
                                        CasePatternSyndicate {
                                            syn_pattern_root: CaseSynPatternExprRoot {
                                                syn_pattern_expr_idx: 3,
                                            },
                                            variables: ArenaIdxRange(
                                                1..1,
                                            ),
                                        },
                                    ),
                                    heavy_arrow_token: Ok(
                                        HeavyArrowRegionalToken(
                                            RegionalTokenIdx(
                                                42,
                                            ),
                                        ),
                                    ),
                                    stmts: Ok(
                                        ArenaIdxRange(
                                            3..4,
                                        ),
                                    ),
                                },
                            ],
                        },
                    ],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [
                            SynPatternExprData::UnitTypeVariant {
                                path_expr_idx: 2,
                                path: TypeVariantPath(
                                    ItemPathId {
                                        data: ItemPathData::TypeVariant(
                                            TypeVariantPathData {
                                                parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                ident: `ConfidentYes`,
                                                index: U8(
                                                    0,
                                                ),
                                            },
                                        ),
                                    },
                                ),
                            },
                            SynPatternExprData::UnitTypeVariant {
                                path_expr_idx: 10,
                                path: TypeVariantPath(
                                    ItemPathId {
                                        data: ItemPathData::TypeVariant(
                                            TypeVariantPathData {
                                                parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                ident: `ConfidentNo`,
                                                index: U8(
                                                    1,
                                                ),
                                            },
                                        ),
                                    },
                                ),
                            },
                            SynPatternExprData::UnitTypeVariant {
                                path_expr_idx: 18,
                                path: TypeVariantPath(
                                    ItemPathId {
                                        data: ItemPathData::TypeVariant(
                                            TypeVariantPathData {
                                                parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                ident: `Unconfident`,
                                                index: U8(
                                                    2,
                                                ),
                                            },
                                        ),
                                    },
                                ),
                            },
                        ],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [
                            Pure,
                            Pure,
                            Pure,
                        ],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [
                        [],
                        [],
                        [],
                    ],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [
                            InheritedSynSymbol {
                                parent_symbol_idx: Current(
                                    1,
                                ),
                                modifier: Const,
                                kind: InheritedSynSymbolKind::TemplateParameter(
                                    InheritedTemplateParameterSynSymbol::Type {
                                        ident: `Label`,
                                    },
                                ),
                            },
                            InheritedSynSymbol {
                                parent_symbol_idx: Current(
                                    2,
                                ),
                                modifier: Const,
                                kind: InheritedSynSymbolKind::TemplateParameter(
                                    InheritedTemplateParameterSynSymbol::Constant {
                                        ident: `label`,
                                    },
                                ),
                            },
                            InheritedSynSymbol {
                                parent_symbol_idx: Current(
                                    1,
                                ),
                                modifier: Pure,
                                kind: InheritedSynSymbolKind::ParenateParameter {
                                    ident: `one_vs_all_result`,
                                },
                            },
                        ],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Case,
                        syn_pattern_expr_idx: 1,
                    },
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Case,
                        syn_pattern_expr_idx: 2,
                    },
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Case,
                        syn_pattern_expr_idx: 3,
                    },
                ],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 4,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 7,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 10,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::BlockExpr,
                        syn_expr_idx: 11,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                AssociatedItem(
                    TraitForTypeItem(
                        TraitForTypeItemSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 465,
                                },
                            ),
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                InheritedSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 545,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    inherited_syn_symbol_idx: 3,
                                    inherited_syn_symbol_kind: ParenateParameter {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 545,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 107,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 6,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 74,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 9,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 8,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 468,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    4,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    5,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    3,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 14,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 74,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    7,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 9,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    8,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            10,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                10,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 16,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 469,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    11,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    12,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            13,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                13,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        5,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            7,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    6,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 22,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 73,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    14,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 9,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    15,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            17,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                17,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Unit {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 23,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            15,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    9,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            16,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    18,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                16,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            4..5,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    21,
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        4,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            16,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Match {
                                    match_token: MatchRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    match_target: SemaExprIdx(
                                        1,
                                    ),
                                    eol_with_token: EolWithRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    case_branches: [
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                            case_pattern_sema_obelisk: CasePatternSemaSyndicate {
                                                syn_pattern_root: CaseSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..1,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    8,
                                                ),
                                            ),
                                            stmts: SemaStmtIdxRange(
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        },
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    21,
                                                ),
                                            ),
                                            case_pattern_sema_obelisk: CasePatternSemaSyndicate {
                                                syn_pattern_root: CaseSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 2,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..1,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    25,
                                                ),
                                            ),
                                            stmts: SemaStmtIdxRange(
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        },
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    38,
                                                ),
                                            ),
                                            case_pattern_sema_obelisk: CasePatternSemaSyndicate {
                                                syn_pattern_root: CaseSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 3,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..1,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    42,
                                                ),
                                            ),
                                            stmts: SemaStmtIdxRange(
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    11,
                    (
                        SemaExprIdx(
                            11,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
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
                                    place: Some(
                                        Const,
                                    ),
                                    base: Ethereal(
                                        Symbol(
                                            EtherealTermSymbol(
                                                Id {
                                                    value: 7,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 7,
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
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                        None,
                    ],
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
                        entries: [
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        2,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        2,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 23,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: TypeOntology {
                                    path: TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 72,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 101,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: ResolvedEthereal(
                                    Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        3,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        3,
                                    ),
                                    hole_kind: Any,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: TypeOntology {
                                    path: TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 467,
                                            },
                                        ),
                                    ),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 467,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    4,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    5,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        5,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    13,
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        13,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        5,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 23,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: TypeOntology {
                                    path: TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 72,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    7,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    8,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 101,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            7,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: ResolvedEthereal(
                                    Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        6,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        6,
                                    ),
                                    hole_kind: Any,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: TypeOntology {
                                    path: TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 467,
                                            },
                                        ),
                                    ),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 467,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    11,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    12,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        8,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 87,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 87,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        8,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 23,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: TypeOntology {
                                    path: TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 72,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    14,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    15,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 101,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            15,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                16,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: ResolvedEthereal(
                                    Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 54,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 16,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 107,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: Hollow(
                                                            HollowTerm(
                                                                2,
                                                            ),
                                                        ),
                                                    },
                                                    variant: Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 101,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 87,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expectation(
                                            4,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expectation(
                                            4,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expectation(
                                            5,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                4,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expectation(
                                            5,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                5,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                10,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: Hollow(
                                                            HollowTerm(
                                                                9,
                                                            ),
                                                        ),
                                                    },
                                                    variant: Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                7,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    7,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                13,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 101,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 87,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expectation(
                                            11,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expectation(
                                            11,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                8,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expectation(
                                            12,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                11,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expectation(
                                            12,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                12,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                17,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: Hollow(
                                                            HollowTerm(
                                                                16,
                                                            ),
                                                        ),
                                                    },
                                                    variant: Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                15,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Hollow(
                                                HollowTerm(
                                                    15,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 23,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 101,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                16,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 87,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expectation(
                                            18,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                14,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expectation(
                                            18,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                15,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 101,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 101,
                        },
                    ),
                ),
            ),
            self_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 87,
                        },
                    ),
                ),
            ),
        },
    },
]
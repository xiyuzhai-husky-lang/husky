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
                                                    parent_ty_path: TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 465,
                                                            },
                                                        ),
                                                    ),
                                                    ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 542,
                                                            },
                                                        ),
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
                                                    parent_ty_path: TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 465,
                                                            },
                                                        ),
                                                    ),
                                                    ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 542,
                                                            },
                                                        ),
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
                                    value: 459,
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
                                                    value: 467,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 117,
                                                },
                                            ),
                                        ),
                                    ),
                                },
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
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 117,
                                                },
                                            ),
                                        ),
                                    ),
                                },
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
                                            Trivial(
                                                TrivialFluffyCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
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
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 117,
                                                },
                                            ),
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
                                                    value: 10,
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
                                                value: 10,
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
                                                value: 11,
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
                        entries: [],
                        first_unresolved_term_idx: 0,
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
                                                            value: 117,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 117,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
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
                                                            value: 117,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 117,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
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
                            value: 117,
                        },
                    ),
                ),
            ),
            self_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 117,
                        },
                    ),
                ),
            ),
        },
    },
]
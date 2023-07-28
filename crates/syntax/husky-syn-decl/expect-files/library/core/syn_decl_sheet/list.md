Ok(
    SynDeclSheet {
        [salsa id]: 7,
        decls: [
            (
                ItemPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::list::List`, `Extern`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemSynDecl::Type(
                        TypeSynDecl::Extern(
                            ExternTypeSynDecl {
                                path: TypePath(`core::list::List`, `Extern`),
                                template_parameters: [
                                    TemplateParameterDecl {
                                        annotated_variance_token: Some(
                                            VarianceToken::Covariant(
                                                CovariantToken {
                                                    token_idx: TokenIdx(
                                                        4,
                                                    ),
                                                },
                                            ),
                                        ),
                                        symbol: 0,
                                        variant: TemplateParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `E`,
                                                token_idx: TokenIdx(
                                                    5,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                ],
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::ModuleItem(
                                                ModuleItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`core::list::List`, `Extern`),
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
                                        pattern_expr_region: PatternSynExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_infos: [],
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
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            6,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
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
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockPath(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                ),
                Decl::ImplBlock(
                    ImplBlockSynDecl::Type(
                        TypeImplBlockSynDecl {
                            path: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            template_parameters: [
                                TemplateParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `E`,
                                            token_idx: TokenIdx(
                                                10,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                            ],
                            self_ty_expr: SelfTypeExpr {
                                expr: 2,
                            },
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TypeImplBlock(
                                                TypeImplBlockSynNodePath {
                                                    path: TypeImplBlockPath {
                                                        module_path: `core::list`,
                                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::list::List`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `E`,
                                                token_idx: TokenIdx(
                                                    13,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `E`,
                                                            token_idx: TokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `List`,
                                                        token_idx: TokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::list::List`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternSynExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_infos: [],
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
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        11,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    10,
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
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: SelfType,
                                            expr_idx: 2,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `ilen`,
                            item_kind: MethodFn,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MethodFn(
                            TypeMethodFnSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::list`,
                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `ilen`,
                                    item_kind: MethodFn,
                                },
                                template_parameters: [],
                                self_parameter: None,
                                parenic_parameters: [],
                                return_ty: Some(
                                    ReturnTypeExprBeforeColon {
                                        expr: 0,
                                    },
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntitySynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TypeImplBlock(
                                                                TypeImplBlockSynNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `core::list`,
                                                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 0,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::list::List`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    13,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E`,
                                                                            token_idx: TokenIdx(
                                                                                10,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 0,
                                                                argument_expr_idx: 1,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `List`,
                                                                        token_idx: TokenIdx(
                                                                            12,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::list::List`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternSynExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
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
                                                            data: [
                                                                CurrentSynSymbol {
                                                                    modifier: Const,
                                                                    access_start: TokenIdx(
                                                                        11,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E`,
                                                                                token_idx: TokenIdx(
                                                                                    10,
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
                                                                ImplicitTypeParameter,
                                                                ArenaIdxRange(
                                                                    0..1,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `core::list`,
                                                                    ty_path: TypePath(`core::list::List`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `ilen`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                21,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternSynExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_infos: [],
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
                                                data: [
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        modifier: Const,
                                                        kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `E`,
                                                            },
                                                        ),
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `push`,
                            item_kind: MethodFn,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MethodFn(
                            TypeMethodFnSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::list`,
                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `push`,
                                    item_kind: MethodFn,
                                },
                                template_parameters: [],
                                self_parameter: Some(
                                    SelfParameterDeclPattern::Mut {
                                        mut_token: MutToken {
                                            token_idx: TokenIdx(
                                                27,
                                            ),
                                        },
                                        self_value_token: SelfValueToken {
                                            token_idx: TokenIdx(
                                                28,
                                            ),
                                        },
                                    },
                                ),
                                parenic_parameters: [
                                    SpecificParameterDecl::Regular {
                                        pattern: 0,
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon: ColonToken(
                                            TokenIdx(
                                                31,
                                            ),
                                        ),
                                        ty: 0,
                                    },
                                ],
                                return_ty: None,
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntitySynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TypeImplBlock(
                                                                TypeImplBlockSynNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `core::list`,
                                                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 0,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::list::List`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    13,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E`,
                                                                            token_idx: TokenIdx(
                                                                                10,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 0,
                                                                argument_expr_idx: 1,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `List`,
                                                                        token_idx: TokenIdx(
                                                                            12,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::list::List`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternSynExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
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
                                                            data: [
                                                                CurrentSynSymbol {
                                                                    modifier: Const,
                                                                    access_start: TokenIdx(
                                                                        11,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E`,
                                                                                token_idx: TokenIdx(
                                                                                    10,
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
                                                                ImplicitTypeParameter,
                                                                ArenaIdxRange(
                                                                    0..1,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `core::list`,
                                                                    ty_path: TypePath(`core::list::List`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `push`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::InheritedSymbol {
                                                    ident: `E`,
                                                    token_idx: TokenIdx(
                                                        32,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternSynExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternSynExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `e`,
                                                            token_idx: TokenIdx(
                                                                30,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    None,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    PatternSynSymbol::Atom(
                                                        0,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `e`,
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    None,
                                                ],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        modifier: Const,
                                                        kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `E`,
                                                            },
                                                        ),
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            31,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ExplicitRegularParameter {
                                                            ident: `e`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 0,
                                                        ty_expr_idx: 0,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `first`,
                            item_kind: MethodFn,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MethodFn(
                            TypeMethodFnSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::list`,
                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `first`,
                                    item_kind: MethodFn,
                                },
                                template_parameters: [],
                                self_parameter: None,
                                parenic_parameters: [],
                                return_ty: Some(
                                    ReturnTypeExprBeforeColon {
                                        expr: 2,
                                    },
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntitySynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TypeImplBlock(
                                                                TypeImplBlockSynNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `core::list`,
                                                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 0,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::list::List`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    13,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E`,
                                                                            token_idx: TokenIdx(
                                                                                10,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 0,
                                                                argument_expr_idx: 1,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `List`,
                                                                        token_idx: TokenIdx(
                                                                            12,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::list::List`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternSynExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
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
                                                            data: [
                                                                CurrentSynSymbol {
                                                                    modifier: Const,
                                                                    access_start: TokenIdx(
                                                                        11,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E`,
                                                                                token_idx: TokenIdx(
                                                                                    10,
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
                                                                ImplicitTypeParameter,
                                                                ArenaIdxRange(
                                                                    0..1,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `core::list`,
                                                                    ty_path: TypePath(`core::list::List`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `first`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::option::Option`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `E`,
                                                    token_idx: TokenIdx(
                                                        42,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 0,
                                                    argument_expr_idx: 1,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Option`,
                                                            token_idx: TokenIdx(
                                                                41,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::option::Option`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternSynExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_infos: [],
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
                                                data: [
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        modifier: Const,
                                                        kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `E`,
                                                            },
                                                        ),
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 2,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `last`,
                            item_kind: MethodFn,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MethodFn(
                            TypeMethodFnSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::list`,
                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `last`,
                                    item_kind: MethodFn,
                                },
                                template_parameters: [],
                                self_parameter: None,
                                parenic_parameters: [],
                                return_ty: Some(
                                    ReturnTypeExprBeforeColon {
                                        expr: 2,
                                    },
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntitySynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TypeImplBlock(
                                                                TypeImplBlockSynNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `core::list`,
                                                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 0,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::list::List`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    13,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E`,
                                                                            token_idx: TokenIdx(
                                                                                10,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 0,
                                                                argument_expr_idx: 1,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `List`,
                                                                        token_idx: TokenIdx(
                                                                            12,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::list::List`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternSynExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
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
                                                            data: [
                                                                CurrentSynSymbol {
                                                                    modifier: Const,
                                                                    access_start: TokenIdx(
                                                                        11,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E`,
                                                                                token_idx: TokenIdx(
                                                                                    10,
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
                                                                ImplicitTypeParameter,
                                                                ArenaIdxRange(
                                                                    0..1,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `core::list`,
                                                                    ty_path: TypePath(`core::list::List`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `last`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::option::Option`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `E`,
                                                    token_idx: TokenIdx(
                                                        51,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 0,
                                                    argument_expr_idx: 1,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Option`,
                                                            token_idx: TokenIdx(
                                                                50,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::option::Option`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternSynExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_infos: [],
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
                                                data: [
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        modifier: Const,
                                                        kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `E`,
                                                            },
                                                        ),
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 2,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `pop`,
                            item_kind: MethodFn,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MethodFn(
                            TypeMethodFnSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::list`,
                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `pop`,
                                    item_kind: MethodFn,
                                },
                                template_parameters: [],
                                self_parameter: Some(
                                    SelfParameterDeclPattern::Mut {
                                        mut_token: MutToken {
                                            token_idx: TokenIdx(
                                                57,
                                            ),
                                        },
                                        self_value_token: SelfValueToken {
                                            token_idx: TokenIdx(
                                                58,
                                            ),
                                        },
                                    },
                                ),
                                parenic_parameters: [],
                                return_ty: Some(
                                    ReturnTypeExprBeforeColon {
                                        expr: 2,
                                    },
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntitySynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TypeImplBlock(
                                                                TypeImplBlockSynNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `core::list`,
                                                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 0,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::list::List`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    13,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E`,
                                                                            token_idx: TokenIdx(
                                                                                10,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 0,
                                                                argument_expr_idx: 1,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `List`,
                                                                        token_idx: TokenIdx(
                                                                            12,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::list::List`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternSynExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
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
                                                            data: [
                                                                CurrentSynSymbol {
                                                                    modifier: Const,
                                                                    access_start: TokenIdx(
                                                                        11,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E`,
                                                                                token_idx: TokenIdx(
                                                                                    10,
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
                                                                ImplicitTypeParameter,
                                                                ArenaIdxRange(
                                                                    0..1,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `core::list`,
                                                                    ty_path: TypePath(`core::list::List`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `pop`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::option::Option`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `E`,
                                                    token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 0,
                                                    argument_expr_idx: 1,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Option`,
                                                            token_idx: TokenIdx(
                                                                61,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::option::Option`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternSynExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_infos: [],
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
                                                data: [
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        modifier: Const,
                                                        kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `E`,
                                                            },
                                                        ),
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 2,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)
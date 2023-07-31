Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Extern(
                    ExternTypeSynDefn {
                        path: TypePath(`core::logic::Prop`, `Extern`),
                        decl: ExternTypeSynDecl {
                            path: TypePath(`core::logic::Prop`, `Extern`),
                            template_parameters: [],
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`core::logic::Prop`, `Extern`),
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
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Structure(
                    StructureTypeSynDefn {
                        path: TypePath(`core::logic::LogicAnd`, `Structure`),
                        decl: StructureTypeSynDecl {
                            path: TypePath(`core::logic::LogicAnd`, `Structure`),
                            template_parameters: [
                                TemplateParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `P`,
                                            token_idx: TokenIdx(
                                                8,
                                            ),
                                        },
                                        traits: Some(
                                            (
                                                ColonToken(
                                                    TokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ),
                                    },
                                },
                                TemplateParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `Q`,
                                            token_idx: TokenIdx(
                                                12,
                                            ),
                                        },
                                        traits: Some(
                                            (
                                                ColonToken(
                                                    TokenIdx(
                                                        13,
                                                    ),
                                                ),
                                                1,
                                            ),
                                        ),
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
                                                        path: TypePath(`core::logic::LogicAnd`, `Structure`),
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
                                                            TypePath(`core::logic::Prop`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::logic::Prop`, `Extern`),
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
                                                        ident: `Prop`,
                                                        token_idx: TokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::logic::Prop`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Prop`,
                                                        token_idx: TokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::logic::Prop`, `Extern`),
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
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `P`,
                                                                token_idx: TokenIdx(
                                                                    8,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Q`,
                                                                token_idx: TokenIdx(
                                                                    12,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: Traits,
                                            expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: Traits,
                                            expr_idx: 1,
                                        },
                                    ],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Inductive(
                    InductiveTypeSynDefn {
                        path: TypePath(`core::logic::LogicOr`, `Inductive`),
                        decl: InductiveTypeSynDecl {
                            path: TypePath(`core::logic::LogicOr`, `Inductive`),
                            template_parameters: [
                                TemplateParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `P`,
                                            token_idx: TokenIdx(
                                                30,
                                            ),
                                        },
                                        traits: Some(
                                            (
                                                ColonToken(
                                                    TokenIdx(
                                                        31,
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ),
                                    },
                                },
                                TemplateParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `Q`,
                                            token_idx: TokenIdx(
                                                34,
                                            ),
                                        },
                                        traits: Some(
                                            (
                                                ColonToken(
                                                    TokenIdx(
                                                        35,
                                                    ),
                                                ),
                                                1,
                                            ),
                                        ),
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
                                                        path: TypePath(`core::logic::LogicOr`, `Inductive`),
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
                                                            TypePath(`core::logic::Prop`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::logic::Prop`, `Extern`),
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
                                                        ident: `Prop`,
                                                        token_idx: TokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::logic::Prop`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Prop`,
                                                        token_idx: TokenIdx(
                                                            36,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::logic::Prop`, `Extern`),
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
                                                        31,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `P`,
                                                                token_idx: TokenIdx(
                                                                    30,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        35,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Q`,
                                                                token_idx: TokenIdx(
                                                                    34,
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
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: Traits,
                                            expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: Traits,
                                            expr_idx: 1,
                                        },
                                    ],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)
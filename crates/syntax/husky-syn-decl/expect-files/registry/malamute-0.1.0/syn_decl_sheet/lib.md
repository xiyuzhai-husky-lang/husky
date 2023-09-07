Ok(
    SynDeclSheet {
        [salsa id]: 48,
        decls: [
            (
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::Class`, `Enum`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Type(
                        TypeSynDecl::Enum(
                            EnumTypeSynDecl {
                                path: TypePath(`malamute::Class`, `Enum`),
                                template_parameters: [
                                    TemplateParameterObelisk {
                                        annotated_variance_token: None,
                                        symbol: 0,
                                        variant: TemplateParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    14,
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
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`malamute::Class`, `Enum`),
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
                                                            15,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        14,
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
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Type(
                        TypeSynDecl::Enum(
                            EnumTypeSynDecl {
                                path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_parameters: [
                                    TemplateParameterObelisk {
                                        annotated_variance_token: None,
                                        symbol: 0,
                                        variant: TemplateParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    36,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                    TemplateParameterObelisk {
                                        annotated_variance_token: None,
                                        symbol: 1,
                                        variant: TemplateParameterDeclPatternVariant::Constant {
                                            const_token: ConstToken {
                                                token_idx: TokenIdx(
                                                    38,
                                                ),
                                            },
                                            ident_token: IdentToken {
                                                ident: `label`,
                                                token_idx: TokenIdx(
                                                    39,
                                                ),
                                            },
                                            colon_token: ColonToken(
                                                TokenIdx(
                                                    40,
                                                ),
                                            ),
                                            ty_expr: 0,
                                        },
                                    },
                                ],
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::CurrentSymbol {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        41,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    36,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
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
                                                            37,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        36,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            42,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                ident_token: IdentToken {
                                                                    ident: `label`,
                                                                    token_idx: TokenIdx(
                                                                        39,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 0,
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
                                                kind: ConstantImplicitParameterType,
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
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Type(
                        TypeSynDecl::Enum(
                            EnumTypeSynDecl {
                                path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                template_parameters: [
                                    TemplateParameterObelisk {
                                        annotated_variance_token: None,
                                        symbol: 0,
                                        variant: TemplateParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    86,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                    TemplateParameterObelisk {
                                        annotated_variance_token: None,
                                        symbol: 1,
                                        variant: TemplateParameterDeclPatternVariant::Constant {
                                            const_token: ConstToken {
                                                token_idx: TokenIdx(
                                                    88,
                                                ),
                                            },
                                            ident_token: IdentToken {
                                                ident: `label`,
                                                token_idx: TokenIdx(
                                                    89,
                                                ),
                                            },
                                            colon_token: ColonToken(
                                                TokenIdx(
                                                    90,
                                                ),
                                            ),
                                            ty_expr: 0,
                                        },
                                    },
                                ],
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::CurrentSymbol {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        91,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    86,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
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
                                                            87,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        86,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            92,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                ident_token: IdentToken {
                                                                    ident: `label`,
                                                                    token_idx: TokenIdx(
                                                                        89,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 0,
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
                                                kind: ConstantImplicitParameterType,
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
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`malamute::narrow_down`, `Gn`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Fugitive(
                        FugitiveSynDecl::Gn(
                            GnSynDecl {
                                path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                template_parameters: [
                                    TemplateParameterObelisk {
                                        annotated_variance_token: None,
                                        symbol: 0,
                                        variant: TemplateParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    130,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                    TemplateParameterObelisk {
                                        annotated_variance_token: None,
                                        symbol: 1,
                                        variant: TemplateParameterDeclPatternVariant::Constant {
                                            const_token: ConstToken {
                                                token_idx: TokenIdx(
                                                    132,
                                                ),
                                            },
                                            ident_token: IdentToken {
                                                ident: `label`,
                                                token_idx: TokenIdx(
                                                    133,
                                                ),
                                            },
                                            colon_token: ColonToken(
                                                TokenIdx(
                                                    134,
                                                ),
                                            ),
                                            ty_expr: 0,
                                        },
                                    },
                                ],
                                parenate_parameters: [
                                    SpecificParameterObelisk::Variadic {
                                        dot_dot_dot_token: DotDotDotToken(
                                            TokenIdx(
                                                138,
                                            ),
                                        ),
                                        variadic_variant: VariadicVariant::Vec {
                                            lbox_token: LboxToken(
                                                TokenIdx(
                                                    139,
                                                ),
                                            ),
                                            rbox_token: RboxToken(
                                                TokenIdx(
                                                    140,
                                                ),
                                            ),
                                        },
                                        symbol_modifier_keyword_group: None,
                                        ident_token: IdentToken {
                                            ident: `f`,
                                            token_idx: TokenIdx(
                                                141,
                                            ),
                                        },
                                        variable: 2,
                                        colon: ColonToken(
                                            TokenIdx(
                                                142,
                                            ),
                                        ),
                                        ty: 1,
                                    },
                                    SpecificParameterObelisk::Keyed {
                                        pattern: 0,
                                        symbol_modifier_keyword_group: None,
                                        ident_token: IdentToken {
                                            ident: `skip`,
                                            token_idx: TokenIdx(
                                                145,
                                            ),
                                        },
                                        variable: 3,
                                        colon: ColonToken(
                                            TokenIdx(
                                                146,
                                            ),
                                        ),
                                        ty: 2,
                                        eq_token: EqToken(
                                            TokenIdx(
                                                148,
                                            ),
                                        ),
                                        default: Right(
                                            3,
                                        ),
                                    },
                                ],
                                return_ty: Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 8,
                                    },
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::CurrentSymbol {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        135,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    130,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    TokenIdx(
                                                        149,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        154,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    130,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 4,
                                                    argument_expr_idx: 5,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `label`,
                                                    token_idx: TokenIdx(
                                                        155,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    133,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 6,
                                                    argument_expr_idx: 7,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                143,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                147,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `OneVsAllResult`,
                                                            token_idx: TokenIdx(
                                                                153,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `skip`,
                                                            token_idx: TokenIdx(
                                                                145,
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
                                                    SynPatternSymbol::Atom(
                                                        0,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `skip`,
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
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            131,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        130,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            136,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                ident_token: IdentToken {
                                                                    ident: `label`,
                                                                    token_idx: TokenIdx(
                                                                        133,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 0,
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            139,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateVariadicParameter {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `f`,
                                                                token_idx: TokenIdx(
                                                                    141,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            146,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `skip`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
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
                                                (
                                                    ExplicitVariadicParameter {
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 0,
                                                        ty_expr_idx: 2,
                                                    },
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ConstantImplicitParameterType,
                                                expr_idx: 0,
                                            },
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: ExplicitParameterDefaultValue {
                                                    ty_expr_idx: 2,
                                                },
                                                expr_idx: 3,
                                            },
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 8,
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
                ItemPath::ImplBlock(
                    ImplBlockPath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockPath {
                            module_path: `malamute`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`malamute::Class`, `Enum`),
                            ),
                            disambiguator: 0,
                        },
                    ),
                ),
                SynDecl::ImplBlock(
                    ImplBlockSynDecl::TraitForType(
                        TraitForTypeImplBlockSynDecl {
                            path: TraitForTypeImplBlockPath {
                                module_path: `malamute`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`malamute::Class`, `Enum`),
                                ),
                                disambiguator: 0,
                            },
                            template_parameters: [
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `Label`,
                                            token_idx: TokenIdx(
                                                49,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Constant {
                                        const_token: ConstToken {
                                            token_idx: TokenIdx(
                                                51,
                                            ),
                                        },
                                        ident_token: IdentToken {
                                            ident: `label`,
                                            token_idx: TokenIdx(
                                                52,
                                            ),
                                        },
                                        colon_token: ColonToken(
                                            TokenIdx(
                                                53,
                                            ),
                                        ),
                                        ty_expr: 0,
                                    },
                                },
                            ],
                            trai_expr: TraitObelisk {
                                expr: 7,
                            },
                            self_ty_decl: PathLeadingExpr(
                                SelfTypeObelisk {
                                    expr: 10,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                TraitForTypeImplBlockSynNodePath {
                                                    path: TraitForTypeImplBlockPath {
                                                        module_path: `malamute`,
                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`malamute::Class`, `Enum`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    54,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                49,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::ops::Unveil`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    62,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                49,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `label`,
                                                token_idx: TokenIdx(
                                                    63,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                        ident_token: IdentToken {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                52,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 5,
                                                argument_expr_idx: 6,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::Class`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    66,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                49,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 8,
                                                argument_expr_idx: 9,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `core`,
                                                        token_idx: TokenIdx(
                                                            56,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    `core`,
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Subitem {
                                                parent: 0,
                                                scope_resolution_token: ScopeResolutionToken(
                                                    TokenIdx(
                                                        57,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentToken {
                                                        ident: `ops`,
                                                        token_idx: TokenIdx(
                                                            58,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::Module(
                                                        `core::ops`,
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Subitem {
                                                parent: 1,
                                                scope_resolution_token: ScopeResolutionToken(
                                                    TokenIdx(
                                                        59,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentToken {
                                                        ident: `Unveil`,
                                                        token_idx: TokenIdx(
                                                            60,
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
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `OneVsAll`,
                                                        token_idx: TokenIdx(
                                                            61,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Class`,
                                                        token_idx: TokenIdx(
                                                            65,
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
                                                        50,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    49,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        55,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    52,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
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
                                            kind: ConstantImplicitParameterType,
                                            expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: Trait,
                                            expr_idx: 7,
                                        },
                                        SynExprRoot {
                                            kind: SelfType,
                                            expr_idx: 10,
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
                    AssociatedItemPath::TraitForTypeItem(
                        TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `malamute`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`malamute::Class`, `Enum`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `Output`,
                            item_kind: AssociatedType,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TraitForTypeItem(
                        TraitForTypeItemSynDecl::AssociatedType(
                            TraitForTypeAssociatedTypeSynDecl {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `malamute`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`malamute::Class`, `Enum`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `Output`,
                                    item_kind: AssociatedType,
                                },
                                template_parameters: [],
                                ty_term_expr_idx: 0,
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                TraitForTypeImplBlockSynNodePath {
                                                                    path: TraitForTypeImplBlockPath {
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`malamute::Class`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::CurrentSymbol {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    54,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `Label`,
                                                                            token_idx: TokenIdx(
                                                                                49,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 2,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Trait(
                                                                            TraitPath(`core::ops::Unveil`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 3,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 1,
                                                                argument_expr_idx: 2,
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    62,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `Label`,
                                                                            token_idx: TokenIdx(
                                                                                49,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 3,
                                                                argument_expr_idx: 4,
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    63,
                                                                ),
                                                                current_symbol_idx: 1,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                        ident_token: IdentToken {
                                                                            ident: `label`,
                                                                            token_idx: TokenIdx(
                                                                                52,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 5,
                                                                argument_expr_idx: 6,
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 4,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`malamute::Class`, `Enum`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    66,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `Label`,
                                                                            token_idx: TokenIdx(
                                                                                49,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 8,
                                                                argument_expr_idx: 9,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `core`,
                                                                        token_idx: TokenIdx(
                                                                            56,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::Module(
                                                                    `core`,
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Subitem {
                                                                parent: 0,
                                                                scope_resolution_token: ScopeResolutionToken(
                                                                    TokenIdx(
                                                                        57,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentToken {
                                                                        ident: `ops`,
                                                                        token_idx: TokenIdx(
                                                                            58,
                                                                        ),
                                                                    },
                                                                ),
                                                                path: Ok(
                                                                    PrincipalEntityPath::Module(
                                                                        `core::ops`,
                                                                    ),
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Subitem {
                                                                parent: 1,
                                                                scope_resolution_token: ScopeResolutionToken(
                                                                    TokenIdx(
                                                                        59,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentToken {
                                                                        ident: `Unveil`,
                                                                        token_idx: TokenIdx(
                                                                            60,
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
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `OneVsAll`,
                                                                        token_idx: TokenIdx(
                                                                            61,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `Class`,
                                                                        token_idx: TokenIdx(
                                                                            65,
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
                                                                        50,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                        syn_attrs: TemplateParameterSynAttrs {
                                                                            syn_attrs: [],
                                                                        },
                                                                        annotated_variance_token: None,
                                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `Label`,
                                                                                token_idx: TokenIdx(
                                                                                    49,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                CurrentSynSymbol {
                                                                    modifier: Const,
                                                                    access_start: TokenIdx(
                                                                        55,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                        syn_attrs: TemplateParameterSynAttrs {
                                                                            syn_attrs: [],
                                                                        },
                                                                        annotated_variance_token: None,
                                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                            ident_token: IdentToken {
                                                                                ident: `label`,
                                                                                token_idx: TokenIdx(
                                                                                    52,
                                                                                ),
                                                                            },
                                                                            ty_expr_idx: 0,
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
                                                            kind: ConstantImplicitParameterType,
                                                            expr_idx: 0,
                                                        },
                                                        SynExprRoot {
                                                            kind: Trait,
                                                            expr_idx: 7,
                                                        },
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            expr_idx: 10,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TraitForTypeItemPath {
                                                                impl_block: TraitForTypeImplBlockPath {
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`malamute::Class`, `Enum`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `Output`,
                                                                item_kind: AssociatedType,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::Unit {
                                                    lpar_token_idx: TokenIdx(
                                                        71,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        72,
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
                                        pattern_expr_region: SynPatternExprRegion {
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
                                                        kind: InheritedSynSymbolKind::TemplateParameter(
                                                            InheritedTemplateParameterSynSymbol::Type {
                                                                ident: `Label`,
                                                            },
                                                        ),
                                                    },
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
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
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: AssociatedTypeTerm,
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
                ItemPath::ImplBlock(
                    ImplBlockPath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockPath {
                            module_path: `malamute`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                            disambiguator: 0,
                        },
                    ),
                ),
                SynDecl::ImplBlock(
                    ImplBlockSynDecl::TraitForType(
                        TraitForTypeImplBlockSynDecl {
                            path: TraitForTypeImplBlockPath {
                                module_path: `malamute`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                                disambiguator: 0,
                            },
                            template_parameters: [
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `Label`,
                                            token_idx: TokenIdx(
                                                101,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Constant {
                                        const_token: ConstToken {
                                            token_idx: TokenIdx(
                                                103,
                                            ),
                                        },
                                        ident_token: IdentToken {
                                            ident: `label`,
                                            token_idx: TokenIdx(
                                                104,
                                            ),
                                        },
                                        colon_token: ColonToken(
                                            TokenIdx(
                                                105,
                                            ),
                                        ),
                                        ty_expr: 0,
                                    },
                                },
                            ],
                            trai_expr: TraitObelisk {
                                expr: 7,
                            },
                            self_ty_decl: PathLeadingExpr(
                                SelfTypeObelisk {
                                    expr: 12,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                TraitForTypeImplBlockSynNodePath {
                                                    path: TraitForTypeImplBlockPath {
                                                        module_path: `malamute`,
                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    106,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                101,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::ops::Unveil`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    114,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                101,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `label`,
                                                token_idx: TokenIdx(
                                                    115,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                        ident_token: IdentToken {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                104,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 5,
                                                argument_expr_idx: 6,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    118,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                101,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 8,
                                                argument_expr_idx: 9,
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `label`,
                                                token_idx: TokenIdx(
                                                    119,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                        ident_token: IdentToken {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                104,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 10,
                                                argument_expr_idx: 11,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `core`,
                                                        token_idx: TokenIdx(
                                                            108,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    `core`,
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Subitem {
                                                parent: 0,
                                                scope_resolution_token: ScopeResolutionToken(
                                                    TokenIdx(
                                                        109,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentToken {
                                                        ident: `ops`,
                                                        token_idx: TokenIdx(
                                                            110,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::Module(
                                                        `core::ops`,
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Subitem {
                                                parent: 1,
                                                scope_resolution_token: ScopeResolutionToken(
                                                    TokenIdx(
                                                        111,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentToken {
                                                        ident: `Unveil`,
                                                        token_idx: TokenIdx(
                                                            112,
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
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `OneVsAllResult`,
                                                        token_idx: TokenIdx(
                                                            113,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `OneVsAll`,
                                                        token_idx: TokenIdx(
                                                            117,
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
                                                        102,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    101,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        107,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    104,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
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
                                            kind: ConstantImplicitParameterType,
                                            expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: Trait,
                                            expr_idx: 7,
                                        },
                                        SynExprRoot {
                                            kind: SelfType,
                                            expr_idx: 12,
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
                    AssociatedItemPath::TraitForTypeItem(
                        TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `malamute`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `Output`,
                            item_kind: AssociatedType,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TraitForTypeItem(
                        TraitForTypeItemSynDecl::AssociatedType(
                            TraitForTypeAssociatedTypeSynDecl {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `malamute`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `Output`,
                                    item_kind: AssociatedType,
                                },
                                template_parameters: [],
                                ty_term_expr_idx: 0,
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                TraitForTypeImplBlockSynNodePath {
                                                                    path: TraitForTypeImplBlockPath {
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::CurrentSymbol {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    106,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `Label`,
                                                                            token_idx: TokenIdx(
                                                                                101,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 2,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Trait(
                                                                            TraitPath(`core::ops::Unveil`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 3,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 1,
                                                                argument_expr_idx: 2,
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    114,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `Label`,
                                                                            token_idx: TokenIdx(
                                                                                101,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 3,
                                                                argument_expr_idx: 4,
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    115,
                                                                ),
                                                                current_symbol_idx: 1,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                        ident_token: IdentToken {
                                                                            ident: `label`,
                                                                            token_idx: TokenIdx(
                                                                                104,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 5,
                                                                argument_expr_idx: 6,
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 4,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    118,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `Label`,
                                                                            token_idx: TokenIdx(
                                                                                101,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 8,
                                                                argument_expr_idx: 9,
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    119,
                                                                ),
                                                                current_symbol_idx: 1,
                                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                        ident_token: IdentToken {
                                                                            ident: `label`,
                                                                            token_idx: TokenIdx(
                                                                                104,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 10,
                                                                argument_expr_idx: 11,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `core`,
                                                                        token_idx: TokenIdx(
                                                                            108,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::Module(
                                                                    `core`,
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Subitem {
                                                                parent: 0,
                                                                scope_resolution_token: ScopeResolutionToken(
                                                                    TokenIdx(
                                                                        109,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentToken {
                                                                        ident: `ops`,
                                                                        token_idx: TokenIdx(
                                                                            110,
                                                                        ),
                                                                    },
                                                                ),
                                                                path: Ok(
                                                                    PrincipalEntityPath::Module(
                                                                        `core::ops`,
                                                                    ),
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Subitem {
                                                                parent: 1,
                                                                scope_resolution_token: ScopeResolutionToken(
                                                                    TokenIdx(
                                                                        111,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentToken {
                                                                        ident: `Unveil`,
                                                                        token_idx: TokenIdx(
                                                                            112,
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
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `OneVsAllResult`,
                                                                        token_idx: TokenIdx(
                                                                            113,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                    ),
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `OneVsAll`,
                                                                        token_idx: TokenIdx(
                                                                            117,
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
                                                                        102,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                        syn_attrs: TemplateParameterSynAttrs {
                                                                            syn_attrs: [],
                                                                        },
                                                                        annotated_variance_token: None,
                                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `Label`,
                                                                                token_idx: TokenIdx(
                                                                                    101,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                CurrentSynSymbol {
                                                                    modifier: Const,
                                                                    access_start: TokenIdx(
                                                                        107,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                        syn_attrs: TemplateParameterSynAttrs {
                                                                            syn_attrs: [],
                                                                        },
                                                                        annotated_variance_token: None,
                                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                            ident_token: IdentToken {
                                                                                ident: `label`,
                                                                                token_idx: TokenIdx(
                                                                                    104,
                                                                                ),
                                                                            },
                                                                            ty_expr_idx: 0,
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
                                                            kind: ConstantImplicitParameterType,
                                                            expr_idx: 0,
                                                        },
                                                        SynExprRoot {
                                                            kind: Trait,
                                                            expr_idx: 7,
                                                        },
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            expr_idx: 12,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TraitForTypeItemPath {
                                                                impl_block: TraitForTypeImplBlockPath {
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `Output`,
                                                                item_kind: AssociatedType,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::Unit {
                                                    lpar_token_idx: TokenIdx(
                                                        124,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        125,
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
                                        pattern_expr_region: SynPatternExprRegion {
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
                                                        kind: InheritedSynSymbolKind::TemplateParameter(
                                                            InheritedTemplateParameterSynSymbol::Type {
                                                                ident: `Label`,
                                                            },
                                                        ),
                                                    },
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
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
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: AssociatedTypeTerm,
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
        ],
    },
)
Ok(
    SynNodeDeclSheet {
        [salsa id]: 10,
        decls: [
            (
                EntitySynNodePath::ModuleItem(
                    ModuleItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::mem::Ref`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::ModuleItem(
                    ModuleItemSynNodeDecl::Type(
                        TypeSynNodeDecl::Extern(
                            ExternTypeSynNodeDecl {
                                syn_node_path: TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::mem::Ref`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 0,
                                template_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    3,
                                                ),
                                            ),
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
                                                    variant: TemplateParameterDeclPatternVariant::Lifetime {
                                                        label_token: LifetimeLabelToken {
                                                            label: `'a`,
                                                            token_idx: TokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                                TemplateParameterDecl {
                                                    annotated_variance_token: Some(
                                                        VarianceToken::Covariant(
                                                            CovariantToken {
                                                                token_idx: TokenIdx(
                                                                    7,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    symbol: 1,
                                                    variant: TemplateParameterDeclPatternVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `E`,
                                                            token_idx: TokenIdx(
                                                                8,
                                                            ),
                                                        },
                                                        traits: None,
                                                    },
                                                },
                                            ],
                                            commas: [
                                                CommaToken(
                                                    TokenIdx(
                                                        6,
                                                    ),
                                                ),
                                            ],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RightAngleBracketToken(
                                                TokenIdx(
                                                    9,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::ModuleItem(
                                                ModuleItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`core::mem::Ref`, `Extern`),
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
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                                attrs: TemplateParameterAttrs {
                                                                    phantom: false,
                                                                },
                                                            },
                                                            annotated_variance_token: Some(
                                                                VarianceToken::Covariant(
                                                                    CovariantToken {
                                                                        token_idx: TokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Lifetime {
                                                                label_token: LifetimeLabelToken {
                                                                    label: `'a`,
                                                                    token_idx: TokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            9,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                                attrs: TemplateParameterAttrs {
                                                                    phantom: false,
                                                                },
                                                            },
                                                            annotated_variance_token: Some(
                                                                VarianceToken::Covariant(
                                                                    CovariantToken {
                                                                        token_idx: TokenIdx(
                                                                            7,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        8,
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
                                        roots: [],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntitySynNodePath::ModuleItem(
                    ModuleItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::mem::RefMut`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::ModuleItem(
                    ModuleItemSynNodeDecl::Type(
                        TypeSynNodeDecl::Extern(
                            ExternTypeSynNodeDecl {
                                syn_node_path: TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::mem::RefMut`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 1,
                                template_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    14,
                                                ),
                                            ),
                                            template_parameters: [
                                                TemplateParameterDecl {
                                                    annotated_variance_token: Some(
                                                        VarianceToken::Covariant(
                                                            CovariantToken {
                                                                token_idx: TokenIdx(
                                                                    15,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    symbol: 0,
                                                    variant: TemplateParameterDeclPatternVariant::Lifetime {
                                                        label_token: LifetimeLabelToken {
                                                            label: `'a`,
                                                            token_idx: TokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    },
                                                },
                                                TemplateParameterDecl {
                                                    annotated_variance_token: Some(
                                                        VarianceToken::Invariant(
                                                            InvariantToken {
                                                                token_idx: TokenIdx(
                                                                    18,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    symbol: 1,
                                                    variant: TemplateParameterDeclPatternVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `E`,
                                                            token_idx: TokenIdx(
                                                                19,
                                                            ),
                                                        },
                                                        traits: None,
                                                    },
                                                },
                                            ],
                                            commas: [
                                                CommaToken(
                                                    TokenIdx(
                                                        17,
                                                    ),
                                                ),
                                            ],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RightAngleBracketToken(
                                                TokenIdx(
                                                    20,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::ModuleItem(
                                                ModuleItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`core::mem::RefMut`, `Extern`),
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
                                                            17,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                                attrs: TemplateParameterAttrs {
                                                                    phantom: false,
                                                                },
                                                            },
                                                            annotated_variance_token: Some(
                                                                VarianceToken::Covariant(
                                                                    CovariantToken {
                                                                        token_idx: TokenIdx(
                                                                            15,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Lifetime {
                                                                label_token: LifetimeLabelToken {
                                                                    label: `'a`,
                                                                    token_idx: TokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            20,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                                attrs: TemplateParameterAttrs {
                                                                    phantom: false,
                                                                },
                                                            },
                                                            annotated_variance_token: Some(
                                                                VarianceToken::Invariant(
                                                                    InvariantToken {
                                                                        token_idx: TokenIdx(
                                                                            18,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        19,
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
                                        roots: [],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntitySynNodePath::ModuleItem(
                    ModuleItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::mem::Leash`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::ModuleItem(
                    ModuleItemSynNodeDecl::Type(
                        TypeSynNodeDecl::Extern(
                            ExternTypeSynNodeDecl {
                                syn_node_path: TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::mem::Leash`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 2,
                                template_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    25,
                                                ),
                                            ),
                                            template_parameters: [
                                                TemplateParameterDecl {
                                                    annotated_variance_token: Some(
                                                        VarianceToken::Covariant(
                                                            CovariantToken {
                                                                token_idx: TokenIdx(
                                                                    26,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    symbol: 0,
                                                    variant: TemplateParameterDeclPatternVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `E`,
                                                            token_idx: TokenIdx(
                                                                27,
                                                            ),
                                                        },
                                                        traits: None,
                                                    },
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RightAngleBracketToken(
                                                TokenIdx(
                                                    28,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::ModuleItem(
                                                ModuleItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`core::mem::Leash`, `Extern`),
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
                                                            28,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                                attrs: TemplateParameterAttrs {
                                                                    phantom: false,
                                                                },
                                                            },
                                                            annotated_variance_token: Some(
                                                                VarianceToken::Covariant(
                                                                    CovariantToken {
                                                                        token_idx: TokenIdx(
                                                                            26,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        27,
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
                EntitySynNodePath::ImplBlock(
                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `core::mem`,
                                trai_path: TraitPath(`core::marker::Copy`),
                                ty_sketch: Path(
                                    TypePath(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::ImplBlock(
                    ImplBlockSynNodeDecl::TraitForType(
                        TraitForTypeImplBlockSynNodeDecl {
                            syn_node_path: TraitForTypeImplBlockSynNodePath {
                                path: TraitForTypeImplBlockPath {
                                    module_path: `core::mem`,
                                    trai_path: TraitPath(`core::marker::Copy`),
                                    ty_sketch: Path(
                                        TypePath(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 3,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    30,
                                ),
                            },
                            template_parameter_decl_list: Ok(
                                Some(
                                    Generics {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                31,
                                            ),
                                        ),
                                        template_parameters: [
                                            TemplateParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: TemplateParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                33,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            trai_expr: TraitExpr {
                                expr: 0,
                            },
                            for_token: ConnectionForToken {
                                token_idx: TokenIdx(
                                    35,
                                ),
                            },
                            self_ty_decl: PathLeadingExpr(
                                SelfTypeExpr {
                                    expr: 3,
                                },
                            ),
                            eol_colon: Ok(
                                EolToken::Semicolon(
                                    EolSemicolonToken {
                                        token_idx: TokenIdx(
                                            38,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                TraitForTypeImplBlockSynNodePath {
                                                    path: TraitForTypeImplBlockPath {
                                                        module_path: `core::mem`,
                                                        trai_path: TraitPath(`core::marker::Copy`),
                                                        ty_sketch: Path(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
                                                        ),
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
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::marker::Copy`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::mem::Leash`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `E`,
                                                token_idx: TokenIdx(
                                                    37,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `E`,
                                                            token_idx: TokenIdx(
                                                                32,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Copy`,
                                                        token_idx: TokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`core::marker::Copy`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Leash`,
                                                        token_idx: TokenIdx(
                                                            36,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::mem::Leash`, `Extern`),
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
                                                        33,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                            attrs: TemplateParameterAttrs {
                                                                phantom: false,
                                                            },
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    32,
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
                                    roots: [
                                        SynExprRoot {
                                            kind: Trait,
                                            expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SelfType,
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
        ],
    },
)
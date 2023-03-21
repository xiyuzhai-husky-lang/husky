Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::mem::Ref`, `Extern`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::Extern(
                            ExternTypeDefn {
                                path: TypePath(`core::mem::Ref`, `Extern`),
                                decl: ExternTypeDecl {
                                    path: TypePath(`core::mem::Ref`, `Extern`),
                                    ast_idx: 0,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::mem::Ref`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                6,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Lifetime {
                                                                    label_token: LifetimeLabelToken {
                                                                        label: `'a`,
                                                                        token_idx: TokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                9,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
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
                                                    ImplicitTypeParameter,
                                                    ImplicitTypeParameter,
                                                ],
                                            },
                                            roots: [],
                                        },
                                    },
                                    implicit_parameter_decl_list: Ok(
                                        Some(
                                            ImplicitParameterDeclList {
                                                langle: LeftAngleBracketOrLessThanToken(
                                                    TokenIdx(
                                                        3,
                                                    ),
                                                ),
                                                implicit_parameters: [
                                                    ImplicitParameterDecl {
                                                        pattern: ImplicitParameterDeclPattern {
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
                                                            variant: ImplicitParameterDeclPatternVariant::Lifetime {
                                                                label_token: LifetimeLabelToken {
                                                                    label: `'a`,
                                                                    token_idx: TokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                        traits: None,
                                                    },
                                                    ImplicitParameterDecl {
                                                        pattern: ImplicitParameterDeclPattern {
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
                                                            variant: ImplicitParameterDeclPatternVariant::Type0 {
                                                                ident_token: IdentToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        8,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                        traits: None,
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
                                                rangle: Ok(
                                                    RightAngleBracketToken(
                                                        TokenIdx(
                                                            9,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::mem::RefMut`, `Extern`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::Extern(
                            ExternTypeDefn {
                                path: TypePath(`core::mem::RefMut`, `Extern`),
                                decl: ExternTypeDecl {
                                    path: TypePath(`core::mem::RefMut`, `Extern`),
                                    ast_idx: 1,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::mem::RefMut`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                17,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Lifetime {
                                                                    label_token: LifetimeLabelToken {
                                                                        label: `'a`,
                                                                        token_idx: TokenIdx(
                                                                            16,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                20,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
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
                                                    ImplicitTypeParameter,
                                                    ImplicitTypeParameter,
                                                ],
                                            },
                                            roots: [],
                                        },
                                    },
                                    implicit_parameter_decl_list: Ok(
                                        Some(
                                            ImplicitParameterDeclList {
                                                langle: LeftAngleBracketOrLessThanToken(
                                                    TokenIdx(
                                                        14,
                                                    ),
                                                ),
                                                implicit_parameters: [
                                                    ImplicitParameterDecl {
                                                        pattern: ImplicitParameterDeclPattern {
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
                                                            variant: ImplicitParameterDeclPatternVariant::Lifetime {
                                                                label_token: LifetimeLabelToken {
                                                                    label: `'a`,
                                                                    token_idx: TokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                        traits: None,
                                                    },
                                                    ImplicitParameterDecl {
                                                        pattern: ImplicitParameterDeclPattern {
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
                                                            variant: ImplicitParameterDeclPatternVariant::Type0 {
                                                                ident_token: IdentToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        19,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                        traits: None,
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
                                                rangle: Ok(
                                                    RightAngleBracketToken(
                                                        TokenIdx(
                                                            20,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::mem::Leash`, `Extern`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::Extern(
                            ExternTypeDefn {
                                path: TypePath(`core::mem::Leash`, `Extern`),
                                decl: ExternTypeDecl {
                                    path: TypePath(`core::mem::Leash`, `Extern`),
                                    ast_idx: 2,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::mem::Leash`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                28,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
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
                                                    ImplicitTypeParameter,
                                                ],
                                            },
                                            roots: [],
                                        },
                                    },
                                    implicit_parameter_decl_list: Ok(
                                        Some(
                                            ImplicitParameterDeclList {
                                                langle: LeftAngleBracketOrLessThanToken(
                                                    TokenIdx(
                                                        25,
                                                    ),
                                                ),
                                                implicit_parameters: [
                                                    ImplicitParameterDecl {
                                                        pattern: ImplicitParameterDeclPattern {
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
                                                            variant: ImplicitParameterDeclPatternVariant::Type0 {
                                                                ident_token: IdentToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        27,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                        traits: None,
                                                    },
                                                ],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rangle: Ok(
                                                    RightAngleBracketToken(
                                                        TokenIdx(
                                                            28,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)
Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::array::Array`, `Extern`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::Extern(
                            ExternTypeDecl {
                                path: TypePath(`core::array::Array`, `Extern`),
                                ast_idx: 0,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::array::Array`, `Extern`),
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
                                        symbol_region: SymbolRegion {
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
                                implicit_parameter_decl_list: Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                3,
                                            ),
                                        ),
                                        implicit_parameters: [],
                                        commas: [],
                                        decl_list_result: Err(
                                            DeclExprError::Original(
                                                OriginalDeclExprError::ExpectedImplicitParameterDecl(
                                                    TokenStreamState {
                                                        next_token_idx: TokenIdx(
                                                            4,
                                                        ),
                                                        drained: false,
                                                    },
                                                ),
                                            ),
                                        ),
                                        rangle: Err(
                                            DeclExprError::Original(
                                                OriginalDeclExprError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                                                    langle_token_idx: TokenIdx(
                                                        3,
                                                    ),
                                                    token_stream_state: TokenStreamState {
                                                        next_token_idx: TokenIdx(
                                                            4,
                                                        ),
                                                        drained: false,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)
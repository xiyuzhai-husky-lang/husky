Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::mem::Ref`, `Alien`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::Alien(
                            AlienTypeDecl {
                                path: TypePath(`core::mem::Ref`, `Alien`),
                                ast_idx: 0,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::mem::Ref`, `Alien`),
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
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [],
                                    },
                                },
                                implicit_parameter_decl_list: Err(
                                    Original(
                                        ExpectRightAngleBracketForImplicitParameterDeclList {
                                            langle_token_idx: TokenIdx(
                                                3,
                                            ),
                                            current_token_idx: TokenIdx(
                                                4,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::mem::RefMut`, `Alien`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::Alien(
                            AlienTypeDecl {
                                path: TypePath(`core::mem::RefMut`, `Alien`),
                                ast_idx: 1,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::mem::RefMut`, `Alien`),
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
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [],
                                    },
                                },
                                implicit_parameter_decl_list: Err(
                                    Original(
                                        ExpectRightAngleBracketForImplicitParameterDeclList {
                                            langle_token_idx: TokenIdx(
                                                14,
                                            ),
                                            current_token_idx: TokenIdx(
                                                15,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)
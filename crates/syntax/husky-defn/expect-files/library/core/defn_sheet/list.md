Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::list::List`, `Alien`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::Alien(
                            AlienTypeDefn {
                                path: TypePath(`core::list::List`, `Alien`),
                                decl: AlienTypeDecl {
                                    path: TypePath(`core::list::List`, `Alien`),
                                    ast_idx: 0,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::list::List`, `Alien`),
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
                                    implicit_parameter_decl_list: Ok(
                                        Some(
                                            ImplicitParameterDeclList {
                                                langle: LeftAngleBracketOrLessThanToken(
                                                    TokenIdx(
                                                        3,
                                                    ),
                                                ),
                                                implicit_parameters: [],
                                                commas: [],
                                                decl_list_result: Err(
                                                    Original(
                                                        ExpectImplicitParameterDecl(
                                                            TokenIdx(
                                                                4,
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                rangle: Err(
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
                                },
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)
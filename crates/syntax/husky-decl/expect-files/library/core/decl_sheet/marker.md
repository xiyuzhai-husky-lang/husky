Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::marker::Copy`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::marker::Copy`),
                            ast_idx: 0,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::marker::Copy`),
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
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::marker::Sized`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::marker::Sized`),
                            ast_idx: 1,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::marker::Sized`),
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
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
        ],
    },
)
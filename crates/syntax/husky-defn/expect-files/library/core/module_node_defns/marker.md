Ok(
    [
        NodeDefn::ModuleItem(
            ModuleItemNodeDefn::Trait(
                TraitNodeDefn {
                    node_path: TraitNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::marker::Copy`),
                            disambiguator: 0,
                        },
                    },
                    node_decl: TraitNodeDecl {
                        node_path: TraitNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::marker::Copy`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 0,
                        implicit_parameter_decl_list: Ok(
                            None,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::marker::Copy`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_entity_path_expr_arena: Arena {
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
                    },
                },
            ),
        ),
        NodeDefn::ModuleItem(
            ModuleItemNodeDefn::Trait(
                TraitNodeDefn {
                    node_path: TraitNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::marker::Sized`),
                            disambiguator: 0,
                        },
                    },
                    node_decl: TraitNodeDecl {
                        node_path: TraitNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::marker::Sized`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 1,
                        implicit_parameter_decl_list: Ok(
                            None,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::marker::Sized`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_entity_path_expr_arena: Arena {
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
                    },
                },
            ),
        ),
    ],
)
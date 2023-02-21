Ok(
    DeclSheet {
        decls: [
            Ok(
                Decl::Type(
                    TypeDecl::Inductive(
                        InductiveTypeDecl {
                            path: TypePath(`natural_number_game::Nat`, `Inductive`),
                            ast_idx: 3,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`natural_number_game::Nat`, `Inductive`),
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
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Decl::Type(
                    TypeDecl::Structure(
                        StructureTypeDecl {
                            path: TypePath(`natural_number_game::OddNat`, `Structure`),
                            ast_idx: 9,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`natural_number_game::OddNat`, `Structure`),
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
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Decl::Type(
                    TypeDecl::Structure(
                        StructureTypeDecl {
                            path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                            ast_idx: 10,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`natural_number_game::EvenNat`, `Structure`),
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
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Decl::ImplBlock(
                    ImplBlockDecl::TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 6,
                            impl_block: ImplBlock {
                                id: ImplBlockId {
                                    module_path: `natural_number_game`,
                                    impl_block_kind: ImplBlockKind::Type {
                                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                    },
                                },
                                ast_idx: 6,
                                body: ArenaIdxRange(
                                    0..3,
                                ),
                                variant: ImplBlockVariant::Type {
                                    ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        11,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::ImplBlock(
                                            ImplBlockId {
                                                module_path: `natural_number_game`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                },
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`natural_number_game::Nat`, `Inductive`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    10,
                                                ),
                                                ident: `Nat`,
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`natural_number_game::Nat`, `Inductive`),
                                                    ),
                                                ),
                                            },
                                        ],
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
                                    roots: [
                                        ExprRoot {
                                            kind: SelfType,
                                            expr: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            Ok(
                Decl::AssociatedItem(
                    AssociatedItemDecl::TypeItem(
                        TypeItemDecl::Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                        ident: `add`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: `natural_number_game`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                            },
                                        },
                                        ident: `add`,
                                    },
                                    path: Some(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath {
                                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                ident: `add`,
                                                ty_item_kind: Memo,
                                            },
                                        ),
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `natural_number_game`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                            },
                                        },
                                        ast_idx: 6,
                                        body: ArenaIdxRange(
                                            0..3,
                                        ),
                                        variant: ImplBlockVariant::Type {
                                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                        },
                                    },
                                    ast_idx: 0,
                                    ident: `add`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: Accessibility::PublicUnder(
                                        `natural_number_game`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 0,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclExprPath::ImplBlock(
                                                            ImplBlockId {
                                                                module_path: `natural_number_game`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    10,
                                                                ),
                                                                ident: `Nat`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
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
                                                    roots: [
                                                        ExprRoot {
                                                            kind: SelfType,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            DeclExprPath::AssociatedItem(
                                                AssociatedItemId {
                                                    impl_block_id: ImplBlockId {
                                                        module_path: `natural_number_game`,
                                                        impl_block_kind: ImplBlockKind::Type {
                                                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                        },
                                                    },
                                                    ident: `add`,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                14,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`natural_number_game::Nat`, `Inductive`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 0,
                                                    opr: Ins,
                                                    opr_token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                    ropd: 1,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`natural_number_game::Nat`, `Inductive`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 2,
                                                    opr: Curry,
                                                    opr_token_idx: TokenIdx(
                                                        16,
                                                    ),
                                                    ropd: 3,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        15,
                                                    ),
                                                    ident: `Nat`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`natural_number_game::Nat`, `Inductive`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        17,
                                                    ),
                                                    ident: `Nat`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`natural_number_game::Nat`, `Inductive`),
                                                        ),
                                                    ),
                                                },
                                            ],
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
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: ReturnType,
                                                expr: 4,
                                            },
                                        ],
                                    },
                                },
                                curry_token: Err(
                                    MissingCurry(
                                        TokenIdx(
                                            14,
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 4,
                                    },
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            18,
                                        ),
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
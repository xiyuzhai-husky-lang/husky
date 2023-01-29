Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Inductive(
                        InductiveTypeDecl {
                            path: TypePath(`natural_number_game::Nat`, `Inductive`),
                            ast_idx: 3,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            TypePath(`natural_number_game::Nat`, `Inductive`),
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
                                        ty_constraints: [],
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
                Type(
                    Structure(
                        StructureTypeDecl {
                            path: TypePath(`natural_number_game::OddNat`, `Structure`),
                            ast_idx: 9,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            TypePath(`natural_number_game::OddNat`, `Structure`),
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
                                        ty_constraints: [],
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
                Type(
                    Structure(
                        StructureTypeDecl {
                            path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                            ast_idx: 10,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            TypePath(`natural_number_game::EvenNat`, `Structure`),
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
                                        ty_constraints: [],
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
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 6,
                            impl_block: ImplBlock {
                                id: ImplBlockId {
                                    module_path: `natural_number_game`,
                                    impl_block_kind: Type {
                                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                    },
                                },
                                ast_idx: 6,
                                body: ArenaIdxRange(
                                    0..3,
                                ),
                                variant: Type {
                                    ty: TypePath(
                                        Id {
                                            value: 35,
                                        },
                                    ),
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
                                    path: Decl(
                                        ImplBlock(
                                            ImplBlock {
                                                id: ImplBlockId {
                                                    module_path: `natural_number_game`,
                                                    impl_block_kind: Type {
                                                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                    },
                                                },
                                                ast_idx: 6,
                                                body: ArenaIdxRange(
                                                    0..3,
                                                ),
                                                variant: Type {
                                                    ty: TypePath(
                                                        Id {
                                                            value: 35,
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`natural_number_game::Nat`, `Inductive`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    10,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 443,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 35,
                                                            },
                                                        ),
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
                                        ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: Type,
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
                AssociatedItem(
                    TypeItem(
                        Memo(
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
                                            module_path: ModulePath(
                                                Id {
                                                    value: 43,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 35,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                            ident: `add`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `natural_number_game`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                            },
                                        },
                                        ast_idx: 6,
                                        body: ArenaIdxRange(
                                            0..3,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 0,
                                    ident: `add`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `natural_number_game`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                    },
                                                                },
                                                                ast_idx: 6,
                                                                body: ArenaIdxRange(
                                                                    0..3,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 35,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    10,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 443,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 35,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 43,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 35,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                            ident: `add`,
                                                            ty_item_kind: Memo,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `natural_number_game`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                            },
                                                        },
                                                        ast_idx: 6,
                                                        body: ArenaIdxRange(
                                                            0..3,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 0,
                                                    ident: `add`,
                                                    associated_item_kind: TypeItem(
                                                        Memo,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `natural_number_game`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Err(
                                                    NoLeftOperandForBinaryOperator {
                                                        binary_token_idx: TokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`natural_number_game::Nat`, `Inductive`),
                                                    ),
                                                },
                                                BinaryOpn {
                                                    lopd: 0,
                                                    opr: Is,
                                                    opr_token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                    ropd: 1,
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`natural_number_game::Nat`, `Inductive`),
                                                    ),
                                                },
                                                BinaryOpn {
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
                                                Root {
                                                    token_idx: TokenIdx(
                                                        15,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 443,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        17,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 443,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
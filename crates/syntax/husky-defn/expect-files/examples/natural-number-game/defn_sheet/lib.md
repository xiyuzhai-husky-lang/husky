Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`natural_number_game::Nat`, `Inductive`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::Inductive(
                            InductiveTypeDefn {
                                path: TypePath(`natural_number_game::Nat`, `Inductive`),
                                decl: InductiveTypeDecl {
                                    path: TypePath(`natural_number_game::Nat`, `Inductive`),
                                    ast_idx: 3,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
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
                                    implicit_parameter_decl_list: Ok(
                                        None,
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
                            TypePath(`natural_number_game::OddNat`, `Structure`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::Structure(
                            StructureTypeDefn {
                                path: TypePath(`natural_number_game::OddNat`, `Structure`),
                                decl: StructureTypeDecl {
                                    path: TypePath(`natural_number_game::OddNat`, `Structure`),
                                    ast_idx: 9,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
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
                                    implicit_parameter_decl_list: Ok(
                                        None,
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
                            TypePath(`natural_number_game::EvenNat`, `Structure`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::Structure(
                            StructureTypeDefn {
                                path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                                decl: StructureTypeDecl {
                                    path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                                    ast_idx: 10,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
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
                                    implicit_parameter_decl_list: Ok(
                                        None,
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplBlockId::Type(
                        TypeImplBlockId {
                            module: `natural_number_game`,
                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplDecl::Type(
                            TypeImplDecl {
                                ast_idx: 6,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module: `natural_number_game`,
                                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 6,
                                    body: ArenaIdxRange(
                                        0..3,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        9,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                ty: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            11,
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Impl(
                                                ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module: `natural_number_game`,
                                                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
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
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module: `natural_number_game`,
                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `add`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Memo(
                                TypeMemoDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                            ident: `add`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    decl: TypeMemoDecl {
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                ident: `add`,
                                                ty_item_kind: Memo,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module: `natural_number_game`,
                                                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                        disambiguator: 0,
                                                    },
                                                ),
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
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module: `natural_number_game`,
                                                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 6,
                                                    body: ArenaIdxRange(
                                                        0..3,
                                                    ),
                                                },
                                            ),
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
                                                                DeclRegionPath::Impl(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module: `natural_number_game`,
                                                                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            expr_arena: Arena {
                                                                data: [
                                                                    Expr::EntityPath {
                                                                        entity_path_expr: 0,
                                                                        path: Some(
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
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module: `natural_number_game`,
                                                                    ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
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
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        Expr::Binary {
                                                            lopd: 0,
                                                            opr: Ins,
                                                            opr_token_idx: TokenIdx(
                                                                14,
                                                            ),
                                                            ropd: 1,
                                                        },
                                                        Expr::EntityPath {
                                                            entity_path_expr: 1,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        Expr::Binary {
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
                                            DeclExprError::Original(
                                                OriginalDeclExprError::ExpectCurry(
                                                    TokenIdx(
                                                        14,
                                                    ),
                                                ),
                                            ),
                                        ),
                                        return_ty: Ok(
                                            OutputTypeExpr {
                                                expr: 4,
                                            },
                                        ),
                                        eol_colon: Err(
                                            DeclExprError::Original(
                                                OriginalDeclExprError::ExpectEolColon(
                                                    TokenIdx(
                                                        18,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module: `natural_number_game`,
                                                                                    ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                    expr_arena: Arena {
                                                                        data: [
                                                                            Expr::EntityPath {
                                                                                entity_path_expr: 0,
                                                                                path: Some(
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
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module: `natural_number_game`,
                                                                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
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
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::Binary {
                                                                    lopd: 0,
                                                                    opr: Ins,
                                                                    opr_token_idx: TokenIdx(
                                                                        14,
                                                                    ),
                                                                    ropd: 1,
                                                                },
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 1,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::Binary {
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module: `natural_number_game`,
                                                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `add`,
                                                    },
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
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [],
                                        },
                                    },
                                    body: Err(
                                        Original(
                                            ExpectBody,
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
        ],
    },
)
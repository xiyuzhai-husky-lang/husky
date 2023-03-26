Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::clone::Clone`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Trait(
                        TraitDefn {
                            path: TraitPath(`core::clone::Clone`),
                            decl: TraitDecl {
                                path: TraitPath(`core::clone::Clone`),
                                ast_idx: 7,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`core::clone::Clone`),
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
            (
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::clone`,
                            trai_path: TraitPath(`core::clone::Clone`),
                            ty_path: TypePath(`core::num::i8`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 8,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::clone`,
                                        trai_path: TraitPath(`core::clone::Clone`),
                                        ty_path: TypePath(`core::num::i8`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 8,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            11,
                                        ),
                                    },
                                    trai_expr: 42,
                                    for_token: TokenIdx(
                                        13,
                                    ),
                                    ty_expr: 43,
                                    body: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        11,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            15,
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i8`, `Extern`),
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
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::clone::Clone`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i8`, `Extern`),
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
                                                        12,
                                                    ),
                                                    ident: `Clone`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::clone::Clone`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                    ident: `i8`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i8`, `Extern`),
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
                                                kind: Trait,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: SelfType,
                                                expr: 1,
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
                        impl_block_id: ImplBlockId::TypeAsTrait(
                            TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::i8`, `Extern`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `clone`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeAsTraitItem(
                            TypeAsTraitItemDefn::Method(
                                TypeAsTraitMethodDefn {
                                    path: Some(
                                        TraitForTypeItemPath {
                                            parent_ty: TypePath(`core::num::i8`, `Extern`),
                                            trai: TraitPath(`core::clone::Clone`),
                                            ident: `clone`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    decl: TypeAsTraitMethodDecl {
                                        path: Some(
                                            TraitForTypeItemPath {
                                                parent_ty: TypePath(`core::num::i8`, `Extern`),
                                                trai: TraitPath(`core::clone::Clone`),
                                                ident: `clone`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i8`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `clone`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath {
                                                        parent_ty: TypePath(`core::num::i8`, `Extern`),
                                                        trai: TraitPath(`core::clone::Clone`),
                                                        ident: `clone`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::TraitForType(
                                                TraitForTypeImplBlock {
                                                    id: TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i8`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 8,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                    trai_expr: 42,
                                                    for_token: TokenIdx(
                                                        13,
                                                    ),
                                                    ty_expr: 43,
                                                    body: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 1,
                                            ident: `clone`,
                                            associated_item_kind: TraitForTypeItem(
                                                MethodFn,
                                            ),
                                            accessibility: Visibility::PublicUnder(
                                                `core::clone`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 1,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::i8`, `Extern`),
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
                                                                                ModuleItemPath::Trait(
                                                                                    TraitPath(`core::clone::Clone`),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    Expr::EntityPath {
                                                                        entity_path_expr: 1,
                                                                        path: Some(
                                                                            EntityPath::ModuleItem(
                                                                                ModuleItemPath::Type(
                                                                                    TypePath(`core::num::i8`, `Extern`),
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
                                                                            12,
                                                                        ),
                                                                        ident: `Clone`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::clone::Clone`),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            14,
                                                                        ),
                                                                        ident: `i8`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::num::i8`, `Extern`),
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
                                                                    kind: Trait,
                                                                    expr: 0,
                                                                },
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 1,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::TypeAsTrait(
                                                                TraitForTypeImplBlockId {
                                                                    module_path: `core::clone`,
                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                    ty_path: TypePath(`core::num::i8`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `clone`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::SelfType(
                                                            TokenIdx(
                                                                21,
                                                            ),
                                                        ),
                                                    ],
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
                                                roots: [
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        18,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            19,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    20,
                                                ),
                                            ),
                                        ),
                                        return_ty: Ok(
                                            OutputTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: Err(
                                            DeclExprError::Original(
                                                OriginalDeclExprError::ExpectEolColon(
                                                    TokenIdx(
                                                        22,
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
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::TypeAsTrait(
                                                                                TraitForTypeImplBlockId {
                                                                                    module_path: `core::clone`,
                                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                                    ty_path: TypePath(`core::num::i8`, `Extern`),
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
                                                                                        ModuleItemPath::Trait(
                                                                                            TraitPath(`core::clone::Clone`),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            Expr::EntityPath {
                                                                                entity_path_expr: 1,
                                                                                path: Some(
                                                                                    EntityPath::ModuleItem(
                                                                                        ModuleItemPath::Type(
                                                                                            TypePath(`core::num::i8`, `Extern`),
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
                                                                                    12,
                                                                                ),
                                                                                ident: `Clone`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Trait(
                                                                                        TraitPath(`core::clone::Clone`),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    14,
                                                                                ),
                                                                                ident: `i8`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`core::num::i8`, `Extern`),
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
                                                                            kind: Trait,
                                                                            expr: 0,
                                                                        },
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 1,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::i8`, `Extern`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `clone`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::SelfType(
                                                                    TokenIdx(
                                                                        21,
                                                                    ),
                                                                ),
                                                            ],
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
                                                        roots: [
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::TypeAsTrait(
                                                            TraitForTypeImplBlockId {
                                                                module_path: `core::clone`,
                                                                trai_path: TraitPath(`core::clone::Clone`),
                                                                ty_path: TypePath(`core::num::i8`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `clone`,
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
                                        DefnError::Original(
                                            OriginalDefnError::ExpectBody,
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::clone`,
                            trai_path: TraitPath(`core::clone::Clone`),
                            ty_path: TypePath(`core::num::i16`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 9,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::clone`,
                                        trai_path: TraitPath(`core::clone::Clone`),
                                        ty_path: TypePath(`core::num::i16`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 9,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            23,
                                        ),
                                    },
                                    trai_expr: 44,
                                    for_token: TokenIdx(
                                        25,
                                    ),
                                    ty_expr: 45,
                                    body: ArenaIdxRange(
                                        2..3,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        23,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        25,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            27,
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i16`, `Extern`),
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
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::clone::Clone`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i16`, `Extern`),
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
                                                        24,
                                                    ),
                                                    ident: `Clone`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::clone::Clone`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        26,
                                                    ),
                                                    ident: `i16`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i16`, `Extern`),
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
                                                kind: Trait,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: SelfType,
                                                expr: 1,
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
                        impl_block_id: ImplBlockId::TypeAsTrait(
                            TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::i16`, `Extern`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `clone`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeAsTraitItem(
                            TypeAsTraitItemDefn::Method(
                                TypeAsTraitMethodDefn {
                                    path: Some(
                                        TraitForTypeItemPath {
                                            parent_ty: TypePath(`core::num::i16`, `Extern`),
                                            trai: TraitPath(`core::clone::Clone`),
                                            ident: `clone`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    decl: TypeAsTraitMethodDecl {
                                        path: Some(
                                            TraitForTypeItemPath {
                                                parent_ty: TypePath(`core::num::i16`, `Extern`),
                                                trai: TraitPath(`core::clone::Clone`),
                                                ident: `clone`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i16`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `clone`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath {
                                                        parent_ty: TypePath(`core::num::i16`, `Extern`),
                                                        trai: TraitPath(`core::clone::Clone`),
                                                        ident: `clone`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::TraitForType(
                                                TraitForTypeImplBlock {
                                                    id: TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i16`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 9,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                    trai_expr: 44,
                                                    for_token: TokenIdx(
                                                        25,
                                                    ),
                                                    ty_expr: 45,
                                                    body: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 2,
                                            ident: `clone`,
                                            associated_item_kind: TraitForTypeItem(
                                                MethodFn,
                                            ),
                                            accessibility: Visibility::PublicUnder(
                                                `core::clone`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 2,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::i16`, `Extern`),
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
                                                                                ModuleItemPath::Trait(
                                                                                    TraitPath(`core::clone::Clone`),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    Expr::EntityPath {
                                                                        entity_path_expr: 1,
                                                                        path: Some(
                                                                            EntityPath::ModuleItem(
                                                                                ModuleItemPath::Type(
                                                                                    TypePath(`core::num::i16`, `Extern`),
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
                                                                            24,
                                                                        ),
                                                                        ident: `Clone`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::clone::Clone`),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            26,
                                                                        ),
                                                                        ident: `i16`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::num::i16`, `Extern`),
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
                                                                    kind: Trait,
                                                                    expr: 0,
                                                                },
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 1,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::TypeAsTrait(
                                                                TraitForTypeImplBlockId {
                                                                    module_path: `core::clone`,
                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                    ty_path: TypePath(`core::num::i16`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `clone`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::SelfType(
                                                            TokenIdx(
                                                                33,
                                                            ),
                                                        ),
                                                    ],
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
                                                roots: [
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        30,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            31,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    32,
                                                ),
                                            ),
                                        ),
                                        return_ty: Ok(
                                            OutputTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: Err(
                                            DeclExprError::Original(
                                                OriginalDeclExprError::ExpectEolColon(
                                                    TokenIdx(
                                                        34,
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
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::TypeAsTrait(
                                                                                TraitForTypeImplBlockId {
                                                                                    module_path: `core::clone`,
                                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                                    ty_path: TypePath(`core::num::i16`, `Extern`),
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
                                                                                        ModuleItemPath::Trait(
                                                                                            TraitPath(`core::clone::Clone`),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            Expr::EntityPath {
                                                                                entity_path_expr: 1,
                                                                                path: Some(
                                                                                    EntityPath::ModuleItem(
                                                                                        ModuleItemPath::Type(
                                                                                            TypePath(`core::num::i16`, `Extern`),
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
                                                                                    24,
                                                                                ),
                                                                                ident: `Clone`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Trait(
                                                                                        TraitPath(`core::clone::Clone`),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    26,
                                                                                ),
                                                                                ident: `i16`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`core::num::i16`, `Extern`),
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
                                                                            kind: Trait,
                                                                            expr: 0,
                                                                        },
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 1,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::i16`, `Extern`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `clone`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::SelfType(
                                                                    TokenIdx(
                                                                        33,
                                                                    ),
                                                                ),
                                                            ],
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
                                                        roots: [
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::TypeAsTrait(
                                                            TraitForTypeImplBlockId {
                                                                module_path: `core::clone`,
                                                                trai_path: TraitPath(`core::clone::Clone`),
                                                                ty_path: TypePath(`core::num::i16`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `clone`,
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
                                        DefnError::Original(
                                            OriginalDefnError::ExpectBody,
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::clone`,
                            trai_path: TraitPath(`core::clone::Clone`),
                            ty_path: TypePath(`core::num::i32`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 10,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::clone`,
                                        trai_path: TraitPath(`core::clone::Clone`),
                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 10,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            35,
                                        ),
                                    },
                                    trai_expr: 46,
                                    for_token: TokenIdx(
                                        37,
                                    ),
                                    ty_expr: 47,
                                    body: ArenaIdxRange(
                                        3..4,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        35,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        37,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            39,
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i32`, `Extern`),
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
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::clone::Clone`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
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
                                                        36,
                                                    ),
                                                    ident: `Clone`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::clone::Clone`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                    ident: `i32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                                kind: Trait,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: SelfType,
                                                expr: 1,
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
                        impl_block_id: ImplBlockId::TypeAsTrait(
                            TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `clone`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeAsTraitItem(
                            TypeAsTraitItemDefn::Method(
                                TypeAsTraitMethodDefn {
                                    path: Some(
                                        TraitForTypeItemPath {
                                            parent_ty: TypePath(`core::num::i32`, `Extern`),
                                            trai: TraitPath(`core::clone::Clone`),
                                            ident: `clone`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    decl: TypeAsTraitMethodDecl {
                                        path: Some(
                                            TraitForTypeItemPath {
                                                parent_ty: TypePath(`core::num::i32`, `Extern`),
                                                trai: TraitPath(`core::clone::Clone`),
                                                ident: `clone`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `clone`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath {
                                                        parent_ty: TypePath(`core::num::i32`, `Extern`),
                                                        trai: TraitPath(`core::clone::Clone`),
                                                        ident: `clone`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::TraitForType(
                                                TraitForTypeImplBlock {
                                                    id: TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 10,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            35,
                                                        ),
                                                    },
                                                    trai_expr: 46,
                                                    for_token: TokenIdx(
                                                        37,
                                                    ),
                                                    ty_expr: 47,
                                                    body: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 3,
                                            ident: `clone`,
                                            associated_item_kind: TraitForTypeItem(
                                                MethodFn,
                                            ),
                                            accessibility: Visibility::PublicUnder(
                                                `core::clone`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 3,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::i32`, `Extern`),
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
                                                                                ModuleItemPath::Trait(
                                                                                    TraitPath(`core::clone::Clone`),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    Expr::EntityPath {
                                                                        entity_path_expr: 1,
                                                                        path: Some(
                                                                            EntityPath::ModuleItem(
                                                                                ModuleItemPath::Type(
                                                                                    TypePath(`core::num::i32`, `Extern`),
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
                                                                            36,
                                                                        ),
                                                                        ident: `Clone`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::clone::Clone`),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            38,
                                                                        ),
                                                                        ident: `i32`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::num::i32`, `Extern`),
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
                                                                    kind: Trait,
                                                                    expr: 0,
                                                                },
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 1,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::TypeAsTrait(
                                                                TraitForTypeImplBlockId {
                                                                    module_path: `core::clone`,
                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `clone`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::SelfType(
                                                            TokenIdx(
                                                                45,
                                                            ),
                                                        ),
                                                    ],
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
                                                roots: [
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        42,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            43,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    44,
                                                ),
                                            ),
                                        ),
                                        return_ty: Ok(
                                            OutputTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: Err(
                                            DeclExprError::Original(
                                                OriginalDeclExprError::ExpectEolColon(
                                                    TokenIdx(
                                                        46,
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
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::TypeAsTrait(
                                                                                TraitForTypeImplBlockId {
                                                                                    module_path: `core::clone`,
                                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
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
                                                                                        ModuleItemPath::Trait(
                                                                                            TraitPath(`core::clone::Clone`),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            Expr::EntityPath {
                                                                                entity_path_expr: 1,
                                                                                path: Some(
                                                                                    EntityPath::ModuleItem(
                                                                                        ModuleItemPath::Type(
                                                                                            TypePath(`core::num::i32`, `Extern`),
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
                                                                                    36,
                                                                                ),
                                                                                ident: `Clone`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Trait(
                                                                                        TraitPath(`core::clone::Clone`),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    38,
                                                                                ),
                                                                                ident: `i32`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`core::num::i32`, `Extern`),
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
                                                                            kind: Trait,
                                                                            expr: 0,
                                                                        },
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 1,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `clone`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::SelfType(
                                                                    TokenIdx(
                                                                        45,
                                                                    ),
                                                                ),
                                                            ],
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
                                                        roots: [
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::TypeAsTrait(
                                                            TraitForTypeImplBlockId {
                                                                module_path: `core::clone`,
                                                                trai_path: TraitPath(`core::clone::Clone`),
                                                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `clone`,
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
                                        DefnError::Original(
                                            OriginalDefnError::ExpectBody,
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::clone`,
                            trai_path: TraitPath(`core::clone::Clone`),
                            ty_path: TypePath(`core::num::i64`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 11,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::clone`,
                                        trai_path: TraitPath(`core::clone::Clone`),
                                        ty_path: TypePath(`core::num::i64`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 11,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            47,
                                        ),
                                    },
                                    trai_expr: 48,
                                    for_token: TokenIdx(
                                        49,
                                    ),
                                    ty_expr: 49,
                                    body: ArenaIdxRange(
                                        4..5,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        47,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        49,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            51,
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i64`, `Extern`),
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
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::clone::Clone`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i64`, `Extern`),
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
                                                        48,
                                                    ),
                                                    ident: `Clone`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::clone::Clone`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        50,
                                                    ),
                                                    ident: `i64`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i64`, `Extern`),
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
                                                kind: Trait,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: SelfType,
                                                expr: 1,
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
                        impl_block_id: ImplBlockId::TypeAsTrait(
                            TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::i64`, `Extern`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `clone`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeAsTraitItem(
                            TypeAsTraitItemDefn::Method(
                                TypeAsTraitMethodDefn {
                                    path: Some(
                                        TraitForTypeItemPath {
                                            parent_ty: TypePath(`core::num::i64`, `Extern`),
                                            trai: TraitPath(`core::clone::Clone`),
                                            ident: `clone`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    decl: TypeAsTraitMethodDecl {
                                        path: Some(
                                            TraitForTypeItemPath {
                                                parent_ty: TypePath(`core::num::i64`, `Extern`),
                                                trai: TraitPath(`core::clone::Clone`),
                                                ident: `clone`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i64`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `clone`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath {
                                                        parent_ty: TypePath(`core::num::i64`, `Extern`),
                                                        trai: TraitPath(`core::clone::Clone`),
                                                        ident: `clone`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::TraitForType(
                                                TraitForTypeImplBlock {
                                                    id: TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i64`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 11,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            47,
                                                        ),
                                                    },
                                                    trai_expr: 48,
                                                    for_token: TokenIdx(
                                                        49,
                                                    ),
                                                    ty_expr: 49,
                                                    body: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 4,
                                            ident: `clone`,
                                            associated_item_kind: TraitForTypeItem(
                                                MethodFn,
                                            ),
                                            accessibility: Visibility::PublicUnder(
                                                `core::clone`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 4,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::i64`, `Extern`),
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
                                                                                ModuleItemPath::Trait(
                                                                                    TraitPath(`core::clone::Clone`),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    Expr::EntityPath {
                                                                        entity_path_expr: 1,
                                                                        path: Some(
                                                                            EntityPath::ModuleItem(
                                                                                ModuleItemPath::Type(
                                                                                    TypePath(`core::num::i64`, `Extern`),
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
                                                                            48,
                                                                        ),
                                                                        ident: `Clone`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::clone::Clone`),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `i64`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::num::i64`, `Extern`),
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
                                                                    kind: Trait,
                                                                    expr: 0,
                                                                },
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 1,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::TypeAsTrait(
                                                                TraitForTypeImplBlockId {
                                                                    module_path: `core::clone`,
                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                    ty_path: TypePath(`core::num::i64`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `clone`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::SelfType(
                                                            TokenIdx(
                                                                57,
                                                            ),
                                                        ),
                                                    ],
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
                                                roots: [
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        54,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            55,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    56,
                                                ),
                                            ),
                                        ),
                                        return_ty: Ok(
                                            OutputTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: Err(
                                            DeclExprError::Original(
                                                OriginalDeclExprError::ExpectEolColon(
                                                    TokenIdx(
                                                        58,
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
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::TypeAsTrait(
                                                                                TraitForTypeImplBlockId {
                                                                                    module_path: `core::clone`,
                                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                                    ty_path: TypePath(`core::num::i64`, `Extern`),
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
                                                                                        ModuleItemPath::Trait(
                                                                                            TraitPath(`core::clone::Clone`),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            Expr::EntityPath {
                                                                                entity_path_expr: 1,
                                                                                path: Some(
                                                                                    EntityPath::ModuleItem(
                                                                                        ModuleItemPath::Type(
                                                                                            TypePath(`core::num::i64`, `Extern`),
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
                                                                                    48,
                                                                                ),
                                                                                ident: `Clone`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Trait(
                                                                                        TraitPath(`core::clone::Clone`),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `i64`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`core::num::i64`, `Extern`),
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
                                                                            kind: Trait,
                                                                            expr: 0,
                                                                        },
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 1,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::i64`, `Extern`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `clone`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::SelfType(
                                                                    TokenIdx(
                                                                        57,
                                                                    ),
                                                                ),
                                                            ],
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
                                                        roots: [
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::TypeAsTrait(
                                                            TraitForTypeImplBlockId {
                                                                module_path: `core::clone`,
                                                                trai_path: TraitPath(`core::clone::Clone`),
                                                                ty_path: TypePath(`core::num::i64`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `clone`,
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
                                        DefnError::Original(
                                            OriginalDefnError::ExpectBody,
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::clone`,
                            trai_path: TraitPath(`core::clone::Clone`),
                            ty_path: TypePath(`core::num::i128`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 12,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::clone`,
                                        trai_path: TraitPath(`core::clone::Clone`),
                                        ty_path: TypePath(`core::num::i128`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 12,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            59,
                                        ),
                                    },
                                    trai_expr: 50,
                                    for_token: TokenIdx(
                                        61,
                                    ),
                                    ty_expr: 51,
                                    body: ArenaIdxRange(
                                        5..6,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        59,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        61,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            63,
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i128`, `Extern`),
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
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::clone::Clone`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i128`, `Extern`),
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
                                                        60,
                                                    ),
                                                    ident: `Clone`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::clone::Clone`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                    ident: `i128`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i128`, `Extern`),
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
                                                kind: Trait,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: SelfType,
                                                expr: 1,
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
                        impl_block_id: ImplBlockId::TypeAsTrait(
                            TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::i128`, `Extern`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `clone`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeAsTraitItem(
                            TypeAsTraitItemDefn::Method(
                                TypeAsTraitMethodDefn {
                                    path: Some(
                                        TraitForTypeItemPath {
                                            parent_ty: TypePath(`core::num::i128`, `Extern`),
                                            trai: TraitPath(`core::clone::Clone`),
                                            ident: `clone`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    decl: TypeAsTraitMethodDecl {
                                        path: Some(
                                            TraitForTypeItemPath {
                                                parent_ty: TypePath(`core::num::i128`, `Extern`),
                                                trai: TraitPath(`core::clone::Clone`),
                                                ident: `clone`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i128`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `clone`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath {
                                                        parent_ty: TypePath(`core::num::i128`, `Extern`),
                                                        trai: TraitPath(`core::clone::Clone`),
                                                        ident: `clone`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::TraitForType(
                                                TraitForTypeImplBlock {
                                                    id: TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::i128`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 12,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            59,
                                                        ),
                                                    },
                                                    trai_expr: 50,
                                                    for_token: TokenIdx(
                                                        61,
                                                    ),
                                                    ty_expr: 51,
                                                    body: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 5,
                                            ident: `clone`,
                                            associated_item_kind: TraitForTypeItem(
                                                MethodFn,
                                            ),
                                            accessibility: Visibility::PublicUnder(
                                                `core::clone`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 5,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::i128`, `Extern`),
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
                                                                                ModuleItemPath::Trait(
                                                                                    TraitPath(`core::clone::Clone`),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    Expr::EntityPath {
                                                                        entity_path_expr: 1,
                                                                        path: Some(
                                                                            EntityPath::ModuleItem(
                                                                                ModuleItemPath::Type(
                                                                                    TypePath(`core::num::i128`, `Extern`),
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
                                                                            60,
                                                                        ),
                                                                        ident: `Clone`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::clone::Clone`),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            62,
                                                                        ),
                                                                        ident: `i128`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::num::i128`, `Extern`),
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
                                                                    kind: Trait,
                                                                    expr: 0,
                                                                },
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 1,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::TypeAsTrait(
                                                                TraitForTypeImplBlockId {
                                                                    module_path: `core::clone`,
                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                    ty_path: TypePath(`core::num::i128`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `clone`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::SelfType(
                                                            TokenIdx(
                                                                69,
                                                            ),
                                                        ),
                                                    ],
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
                                                roots: [
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        66,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            67,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    68,
                                                ),
                                            ),
                                        ),
                                        return_ty: Ok(
                                            OutputTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: Err(
                                            DeclExprError::Original(
                                                OriginalDeclExprError::ExpectEolColon(
                                                    TokenIdx(
                                                        70,
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
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::TypeAsTrait(
                                                                                TraitForTypeImplBlockId {
                                                                                    module_path: `core::clone`,
                                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                                    ty_path: TypePath(`core::num::i128`, `Extern`),
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
                                                                                        ModuleItemPath::Trait(
                                                                                            TraitPath(`core::clone::Clone`),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            Expr::EntityPath {
                                                                                entity_path_expr: 1,
                                                                                path: Some(
                                                                                    EntityPath::ModuleItem(
                                                                                        ModuleItemPath::Type(
                                                                                            TypePath(`core::num::i128`, `Extern`),
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
                                                                                    60,
                                                                                ),
                                                                                ident: `Clone`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Trait(
                                                                                        TraitPath(`core::clone::Clone`),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    62,
                                                                                ),
                                                                                ident: `i128`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`core::num::i128`, `Extern`),
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
                                                                            kind: Trait,
                                                                            expr: 0,
                                                                        },
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 1,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::i128`, `Extern`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `clone`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::SelfType(
                                                                    TokenIdx(
                                                                        69,
                                                                    ),
                                                                ),
                                                            ],
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
                                                        roots: [
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::TypeAsTrait(
                                                            TraitForTypeImplBlockId {
                                                                module_path: `core::clone`,
                                                                trai_path: TraitPath(`core::clone::Clone`),
                                                                ty_path: TypePath(`core::num::i128`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `clone`,
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
                                        DefnError::Original(
                                            OriginalDefnError::ExpectBody,
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::clone`,
                            trai_path: TraitPath(`core::clone::Clone`),
                            ty_path: TypePath(`core::num::isize`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 13,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::clone`,
                                        trai_path: TraitPath(`core::clone::Clone`),
                                        ty_path: TypePath(`core::num::isize`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 13,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            71,
                                        ),
                                    },
                                    trai_expr: 52,
                                    for_token: TokenIdx(
                                        73,
                                    ),
                                    ty_expr: 53,
                                    body: ArenaIdxRange(
                                        6..7,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        71,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        73,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            75,
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::isize`, `Extern`),
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
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::clone::Clone`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
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
                                                        72,
                                                    ),
                                                    ident: `Clone`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::clone::Clone`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        74,
                                                    ),
                                                    ident: `isize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
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
                                                kind: Trait,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: SelfType,
                                                expr: 1,
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
                        impl_block_id: ImplBlockId::TypeAsTrait(
                            TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::isize`, `Extern`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `clone`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeAsTraitItem(
                            TypeAsTraitItemDefn::Method(
                                TypeAsTraitMethodDefn {
                                    path: Some(
                                        TraitForTypeItemPath {
                                            parent_ty: TypePath(`core::num::isize`, `Extern`),
                                            trai: TraitPath(`core::clone::Clone`),
                                            ident: `clone`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    decl: TypeAsTraitMethodDecl {
                                        path: Some(
                                            TraitForTypeItemPath {
                                                parent_ty: TypePath(`core::num::isize`, `Extern`),
                                                trai: TraitPath(`core::clone::Clone`),
                                                ident: `clone`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::isize`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `clone`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath {
                                                        parent_ty: TypePath(`core::num::isize`, `Extern`),
                                                        trai: TraitPath(`core::clone::Clone`),
                                                        ident: `clone`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::TraitForType(
                                                TraitForTypeImplBlock {
                                                    id: TraitForTypeImplBlockId {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_path: TypePath(`core::num::isize`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 13,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            71,
                                                        ),
                                                    },
                                                    trai_expr: 52,
                                                    for_token: TokenIdx(
                                                        73,
                                                    ),
                                                    ty_expr: 53,
                                                    body: ArenaIdxRange(
                                                        6..7,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 6,
                                            ident: `clone`,
                                            associated_item_kind: TraitForTypeItem(
                                                MethodFn,
                                            ),
                                            accessibility: Visibility::PublicUnder(
                                                `core::clone`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 6,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::isize`, `Extern`),
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
                                                                                ModuleItemPath::Trait(
                                                                                    TraitPath(`core::clone::Clone`),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    Expr::EntityPath {
                                                                        entity_path_expr: 1,
                                                                        path: Some(
                                                                            EntityPath::ModuleItem(
                                                                                ModuleItemPath::Type(
                                                                                    TypePath(`core::num::isize`, `Extern`),
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
                                                                            72,
                                                                        ),
                                                                        ident: `Clone`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::clone::Clone`),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            74,
                                                                        ),
                                                                        ident: `isize`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::num::isize`, `Extern`),
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
                                                                    kind: Trait,
                                                                    expr: 0,
                                                                },
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 1,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::TypeAsTrait(
                                                                TraitForTypeImplBlockId {
                                                                    module_path: `core::clone`,
                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                    ty_path: TypePath(`core::num::isize`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `clone`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::SelfType(
                                                            TokenIdx(
                                                                81,
                                                            ),
                                                        ),
                                                    ],
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
                                                roots: [
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        78,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            79,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    80,
                                                ),
                                            ),
                                        ),
                                        return_ty: Ok(
                                            OutputTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: Err(
                                            DeclExprError::Original(
                                                OriginalDeclExprError::ExpectEolColon(
                                                    TokenIdx(
                                                        82,
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
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::TypeAsTrait(
                                                                                TraitForTypeImplBlockId {
                                                                                    module_path: `core::clone`,
                                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                                    ty_path: TypePath(`core::num::isize`, `Extern`),
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
                                                                                        ModuleItemPath::Trait(
                                                                                            TraitPath(`core::clone::Clone`),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            Expr::EntityPath {
                                                                                entity_path_expr: 1,
                                                                                path: Some(
                                                                                    EntityPath::ModuleItem(
                                                                                        ModuleItemPath::Type(
                                                                                            TypePath(`core::num::isize`, `Extern`),
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
                                                                                    72,
                                                                                ),
                                                                                ident: `Clone`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Trait(
                                                                                        TraitPath(`core::clone::Clone`),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    74,
                                                                                ),
                                                                                ident: `isize`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`core::num::isize`, `Extern`),
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
                                                                            kind: Trait,
                                                                            expr: 0,
                                                                        },
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 1,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::TypeAsTrait(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `core::clone`,
                                                                            trai_path: TraitPath(`core::clone::Clone`),
                                                                            ty_path: TypePath(`core::num::isize`, `Extern`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `clone`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::SelfType(
                                                                    TokenIdx(
                                                                        81,
                                                                    ),
                                                                ),
                                                            ],
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
                                                        roots: [
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::TypeAsTrait(
                                                            TraitForTypeImplBlockId {
                                                                module_path: `core::clone`,
                                                                trai_path: TraitPath(`core::clone::Clone`),
                                                                ty_path: TypePath(`core::num::isize`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `clone`,
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
                                        DefnError::Original(
                                            OriginalDefnError::ExpectBody,
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
Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::PropsStruct(
                    PropsStructTypeDefn {
                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        decl: PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            implicit_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `cc`,
                                        token_idx: TokenIdx(
                                            26,
                                        ),
                                    },
                                    colon: ColonToken(
                                        TokenIdx(
                                            27,
                                        ),
                                    ),
                                    ty_expr_idx: 1,
                                    initialization: None,
                                },
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `points`,
                                        token_idx: TokenIdx(
                                            31,
                                        ),
                                    },
                                    colon: ColonToken(
                                        TokenIdx(
                                            32,
                                        ),
                                    ),
                                    ty_expr_idx: 4,
                                    initialization: None,
                                },
                            ],
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Type(
                                                TypeNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    28,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::List {
                                                lbox_token_idx: TokenIdx(
                                                    33,
                                                ),
                                                items: [],
                                                rbox_token_idx: TokenIdx(
                                                    34,
                                                ),
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::ExplicitApplication {
                                                function: 2,
                                                argument: 3,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConnectedComponent`,
                                                        token_idx: TokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Point2d`,
                                                        token_idx: TokenIdx(
                                                            35,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: PropsStructFieldType {
                                                ident_token: IdentToken {
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 265,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        26,
                                                    ),
                                                },
                                            },
                                            expr_idx: 1,
                                        },
                                        ExprRoot {
                                            kind: PropsStructFieldType {
                                                ident_token: IdentToken {
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 236,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        31,
                                                    ),
                                                },
                                            },
                                            expr_idx: 4,
                                        },
                                    ],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Enum(
                    EnumTypeDefn {
                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        decl: EnumTypeDecl {
                            path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                            implicit_parameters: [],
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Type(
                                                TypeNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::PropsStruct(
                    PropsStructTypeDefn {
                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                        decl: PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                            implicit_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `prev1`,
                                        token_idx: TokenIdx(
                                            878,
                                        ),
                                    },
                                    colon: ColonToken(
                                        TokenIdx(
                                            879,
                                        ),
                                    ),
                                    ty_expr_idx: 0,
                                    initialization: None,
                                },
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `prev2`,
                                        token_idx: TokenIdx(
                                            882,
                                        ),
                                    },
                                    colon: ColonToken(
                                        TokenIdx(
                                            883,
                                        ),
                                    ),
                                    ty_expr_idx: 1,
                                    initialization: None,
                                },
                            ],
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Type(
                                                TypeNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `i32`,
                                                        token_idx: TokenIdx(
                                                            880,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `i32`,
                                                        token_idx: TokenIdx(
                                                            884,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: PropsStructFieldType {
                                                ident_token: IdentToken {
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 313,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        878,
                                                    ),
                                                },
                                            },
                                            expr_idx: 0,
                                        },
                                        ExprRoot {
                                            kind: PropsStructFieldType {
                                                ident_token: IdentToken {
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 314,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        882,
                                                    ),
                                                },
                                            },
                                            expr_idx: 1,
                                        },
                                    ],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ImplBlock(
            ImplBlockDecl::TraitForType(
                TraitForTypeImplBlockDecl {
                    path: TraitForTypeImplBlockPath {
                        module_path: `mnist_classifier::raw_contour`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        disambiguator: 0,
                    },
                    implicit_parameters: [],
                    trai_expr: TraitExpr {
                        expr: 0,
                    },
                    ty_expr: TypeExpr {
                        expr: 1,
                    },
                    expr_region: ExprRegion {
                        data: ExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                EntityNodePath::ImplBlock(
                                    ImplBlockNodePath::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockNodePath {
                                            path: TraitForTypeImplBlockPath {
                                                module_path: `mnist_classifier::raw_contour`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    Expr::PrincipalEntityPath {
                                        entity_path_expr: 0,
                                        opt_path: Some(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::visual::Visualize`),
                                                ),
                                            ),
                                        ),
                                    },
                                    Expr::PrincipalEntityPath {
                                        entity_path_expr: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            principal_entity_path_expr_arena: Arena {
                                data: [
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `Visualize`,
                                                token_idx: TokenIdx(
                                                    39,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Trait(
                                                TraitPath(`core::visual::Visualize`),
                                            ),
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `RawContour`,
                                                token_idx: TokenIdx(
                                                    41,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                            roots: [
                                ExprRoot {
                                    kind: Trait,
                                    expr_idx: 0,
                                },
                                ExprRoot {
                                    kind: SelfType,
                                    expr_idx: 1,
                                },
                            ],
                        },
                    },
                },
            ),
        ),
        Defn::AssociatedItem(
            AssociatedItemDefn::TraitForTypeItem(
                TraitForTypeItemDefn::MethodFn(
                    TraitForTypeMethodFnDefn {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `visualize`,
                            item_kind: MethodFn,
                        },
                        decl: TraitForTypeMethodFnDecl {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `mnist_classifier::raw_contour`,
                                    trai_path: TraitPath(`core::visual::Visualize`),
                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `visualize`,
                                item_kind: MethodFn,
                            },
                            implicit_parameters: [],
                            self_parameter: None,
                            explicit_parameters: [],
                            return_ty: Some(
                                ReturnTypeExpr {
                                    expr: 0,
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: Some(
                                        ExprRegion {
                                            data: ExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    EntityNodePath::ImplBlock(
                                                        ImplBlockNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockNodePath {
                                                                path: TraitForTypeImplBlockPath {
                                                                    module_path: `mnist_classifier::raw_contour`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
                                                                        TraitPath(`core::visual::Visualize`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_entity_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `Visualize`,
                                                                    token_idx: TokenIdx(
                                                                        39,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Trait(
                                                                    TraitPath(`core::visual::Visualize`),
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `RawContour`,
                                                                    token_idx: TokenIdx(
                                                                        41,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                roots: [
                                                    ExprRoot {
                                                        kind: Trait,
                                                        expr_idx: 0,
                                                    },
                                                    ExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        EntityNodePath::AssociatedItem(
                                            AssociatedItemNodePath::TraitForTypeItem(
                                                TraitForTypeItemNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TraitForTypeItemPath {
                                                            impl_block: TraitForTypeImplBlockPath {
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `visualize`,
                                                            item_kind: MethodFn,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::visual::Html`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Html`,
                                                        token_idx: TokenIdx(
                                                            48,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::visual::Html`, `Extern`),
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
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            3,
                        ),
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
                                                            EntityNodePath::ImplBlock(
                                                                ImplBlockNodePath::TraitForTypeImplBlock(
                                                                    TraitForTypeImplBlockNodePath {
                                                                        path: TraitForTypeImplBlockPath {
                                                                            module_path: `mnist_classifier::raw_contour`,
                                                                            trai_path: TraitPath(`core::visual::Visualize`),
                                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 0,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::visual::Visualize`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 1,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        principal_entity_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `Visualize`,
                                                                            token_idx: TokenIdx(
                                                                                39,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Trait(
                                                                            TraitPath(`core::visual::Visualize`),
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `RawContour`,
                                                                            token_idx: TokenIdx(
                                                                                41,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                        roots: [
                                                            ExprRoot {
                                                                kind: Trait,
                                                                expr_idx: 0,
                                                            },
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 1,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                EntityNodePath::AssociatedItem(
                                                    AssociatedItemNodePath::TraitForTypeItem(
                                                        TraitForTypeItemNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitForTypeItemPath {
                                                                    impl_block: TraitForTypeImplBlockPath {
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `visualize`,
                                                                    item_kind: MethodFn,
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::visual::Html`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Html`,
                                                                token_idx: TokenIdx(
                                                                    48,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::visual::Html`, `Extern`),
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
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::AssociatedItem(
                                        AssociatedItemNodePath::TraitForTypeItem(
                                            TraitForTypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath {
                                                        impl_block: TraitForTypeImplBlockPath {
                                                            module_path: `mnist_classifier::raw_contour`,
                                                            trai_path: TraitPath(`core::visual::Visualize`),
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `visualize`,
                                                        item_kind: MethodFn,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::SelfValue(
                                            TokenIdx(
                                                55,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                56,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    57,
                                                ),
                                            },
                                        },
                                        Expr::EmptyHtmlTag {
                                            empty_html_bra_idx: TokenIdx(
                                                50,
                                            ),
                                            function_ident: IdentToken {
                                                ident: `Contour`,
                                                token_idx: TokenIdx(
                                                    51,
                                                ),
                                            },
                                            arguments: [
                                                Expanded {
                                                    property_ident: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 236,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                    },
                                                    eq: EqToken(
                                                        TokenIdx(
                                                            53,
                                                        ),
                                                    ),
                                                    lcurl: LeftCurlyBraceToken(
                                                        TokenIdx(
                                                            54,
                                                        ),
                                                    ),
                                                    expr: 1,
                                                    rcurl: RightCurlyBraceToken(
                                                        TokenIdx(
                                                            58,
                                                        ),
                                                    ),
                                                },
                                            ],
                                            empty_html_ket: EmptyHtmlKetToken(
                                                TokenIdx(
                                                    59,
                                                ),
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 2,
                                        },
                                    ],
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
                                    allow_self_value: True,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: HtmlArgumentExpr,
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 2,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 3,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ImplBlock(
            ImplBlockDecl::Type(
                TypeImplBlockDecl {
                    path: TypeImplBlockPath {
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        disambiguator: 0,
                    },
                    implicit_parameters: [],
                    ty_expr: TypeExpr {
                        expr: 0,
                    },
                    expr_region: ExprRegion {
                        data: ExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                EntityNodePath::ImplBlock(
                                    ImplBlockNodePath::TypeImplBlock(
                                        TypeImplBlockNodePath {
                                            path: TypeImplBlockPath {
                                                module_path: `mnist_classifier::raw_contour`,
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    Expr::PrincipalEntityPath {
                                        entity_path_expr: 0,
                                        opt_path: Some(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            principal_entity_path_expr_arena: Arena {
                                data: [
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `RawContour`,
                                                token_idx: TokenIdx(
                                                    61,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                            roots: [
                                ExprRoot {
                                    kind: SelfType,
                                    expr_idx: 0,
                                },
                            ],
                        },
                    },
                },
            ),
        ),
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MemoizedField(
                    TypeMemoizedFieldDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `line_segment_sketch`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::raw_contour`,
                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `line_segment_sketch`,
                                item_kind: MemoizedField,
                            },
                            memo_ty: Some(
                                FormTypeExpr {
                                    expr: 0,
                                },
                            ),
                            expr: None,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: Some(
                                        ExprRegion {
                                            data: ExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    EntityNodePath::ImplBlock(
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::raw_contour`,
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_entity_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `RawContour`,
                                                                    token_idx: TokenIdx(
                                                                        61,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                roots: [
                                                    ExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        EntityNodePath::AssociatedItem(
                                            AssociatedItemNodePath::TypeItem(
                                                TypeItemNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `line_segment_sketch`,
                                                            item_kind: MemoizedField,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `LineSegmentSketch`,
                                                        token_idx: TokenIdx(
                                                            66,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: VarType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            5,
                        ),
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
                                                            EntityNodePath::ImplBlock(
                                                                ImplBlockNodePath::TypeImplBlock(
                                                                    TypeImplBlockNodePath {
                                                                        path: TypeImplBlockPath {
                                                                            module_path: `mnist_classifier::raw_contour`,
                                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 0,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        principal_entity_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `RawContour`,
                                                                            token_idx: TokenIdx(
                                                                                61,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                        roots: [
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                EntityNodePath::AssociatedItem(
                                                    AssociatedItemNodePath::TypeItem(
                                                        TypeItemNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypeItemPath {
                                                                    impl_block: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `line_segment_sketch`,
                                                                    item_kind: MemoizedField,
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `LineSegmentSketch`,
                                                                token_idx: TokenIdx(
                                                                    66,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: VarType,
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::AssociatedItem(
                                        AssociatedItemNodePath::TypeItem(
                                            TypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `mnist_classifier::raw_contour`,
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `line_segment_sketch`,
                                                        item_kind: MemoizedField,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::ScopeResolution {
                                            parent_expr_idx: 0,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    69,
                                                ),
                                            ),
                                            ident_token: IdentToken {
                                                ident: `new`,
                                                token_idx: TokenIdx(
                                                    70,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                72,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                74,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::FunctionApplicationOrCall {
                                            function: 1,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                71,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            73,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                75,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `LineSegmentSketch`,
                                                    token_idx: TokenIdx(
                                                        68,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 4,
                                        },
                                    ],
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
                                    allow_self_value: True,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 4,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 5,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MemoizedField(
                    TypeMemoizedFieldDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `bounding_box`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::raw_contour`,
                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `bounding_box`,
                                item_kind: MemoizedField,
                            },
                            memo_ty: Some(
                                FormTypeExpr {
                                    expr: 0,
                                },
                            ),
                            expr: None,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: Some(
                                        ExprRegion {
                                            data: ExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    EntityNodePath::ImplBlock(
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::raw_contour`,
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_entity_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `RawContour`,
                                                                    token_idx: TokenIdx(
                                                                        61,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                roots: [
                                                    ExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        EntityNodePath::AssociatedItem(
                                            AssociatedItemNodePath::TypeItem(
                                                TypeItemNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `bounding_box`,
                                                            item_kind: MemoizedField,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `BoundingBox`,
                                                        token_idx: TokenIdx(
                                                            79,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: VarType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            55,
                        ),
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
                                                            EntityNodePath::ImplBlock(
                                                                ImplBlockNodePath::TypeImplBlock(
                                                                    TypeImplBlockNodePath {
                                                                        path: TypeImplBlockPath {
                                                                            module_path: `mnist_classifier::raw_contour`,
                                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 0,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        principal_entity_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `RawContour`,
                                                                            token_idx: TokenIdx(
                                                                                61,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                        roots: [
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                EntityNodePath::AssociatedItem(
                                                    AssociatedItemNodePath::TypeItem(
                                                        TypeItemNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypeItemPath {
                                                                    impl_block: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `bounding_box`,
                                                                    item_kind: MemoizedField,
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `BoundingBox`,
                                                                token_idx: TokenIdx(
                                                                    79,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: VarType,
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::AssociatedItem(
                                        AssociatedItemNodePath::TypeItem(
                                            TypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `mnist_classifier::raw_contour`,
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `bounding_box`,
                                                        item_kind: MemoizedField,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::SelfValue(
                                            TokenIdx(
                                                84,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                85,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    86,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                88,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 1,
                                            lbox_token_idx: TokenIdx(
                                                87,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                89,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        94,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 272,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 4,
                                            dot_token_idx: TokenIdx(
                                                95,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    96,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 272,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                102,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    103,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        108,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 272,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 8,
                                            dot_token_idx: TokenIdx(
                                                109,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    110,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        115,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 272,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                116,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    117,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                121,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 12,
                                            dot_token_idx: TokenIdx(
                                                122,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    123,
                                                ),
                                            },
                                        },
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                119,
                                            ),
                                            ident: `i`,
                                            frame_var_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                14,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 13,
                                            dot_token_idx: TokenIdx(
                                                124,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    125,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                126,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                127,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 14,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                120,
                                            ),
                                            ropd: 15,
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                132,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 17,
                                            dot_token_idx: TokenIdx(
                                                133,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                136,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                14,
                                            ),
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 18,
                                            lbox_token_idx: TokenIdx(
                                                135,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 19,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                137,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `xmin`,
                                            token_idx: TokenIdx(
                                                140,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        144,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 277,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 22,
                                            dot_token_idx: TokenIdx(
                                                145,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    146,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `xmin`,
                                            token_idx: TokenIdx(
                                                138,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 21,
                                            dot_token_idx: TokenIdx(
                                                141,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `min`,
                                                token_idx: TokenIdx(
                                                    142,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                143,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 23,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                147,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 24,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                139,
                                            ),
                                            ropd: 25,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `xmax`,
                                            token_idx: TokenIdx(
                                                150,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        154,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 277,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 28,
                                            dot_token_idx: TokenIdx(
                                                155,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    156,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `xmax`,
                                            token_idx: TokenIdx(
                                                148,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 27,
                                            dot_token_idx: TokenIdx(
                                                151,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    152,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                153,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 29,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                157,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 30,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                149,
                                            ),
                                            ropd: 31,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `ymin`,
                                            token_idx: TokenIdx(
                                                160,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        164,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 277,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 34,
                                            dot_token_idx: TokenIdx(
                                                165,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    166,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `ymin`,
                                            token_idx: TokenIdx(
                                                158,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 33,
                                            dot_token_idx: TokenIdx(
                                                161,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `min`,
                                                token_idx: TokenIdx(
                                                    162,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                163,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 35,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                167,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 36,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                159,
                                            ),
                                            ropd: 37,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `ymax`,
                                            token_idx: TokenIdx(
                                                170,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        174,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 277,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 40,
                                            dot_token_idx: TokenIdx(
                                                175,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    176,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `ymax`,
                                            token_idx: TokenIdx(
                                                168,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 39,
                                            dot_token_idx: TokenIdx(
                                                171,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    172,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                173,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 41,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                177,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 42,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                169,
                                            ),
                                            ropd: 43,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `xmin`,
                                            token_idx: TokenIdx(
                                                183,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `xmax`,
                                            token_idx: TokenIdx(
                                                185,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 46,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                182,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 47,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            184,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 48,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                186,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `ymin`,
                                            token_idx: TokenIdx(
                                                190,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `ymax`,
                                            token_idx: TokenIdx(
                                                192,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 50,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                189,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 51,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            191,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 52,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                193,
                                            ),
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 45,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                180,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 49,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            187,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 53,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            194,
                                                        ),
                                                    ),
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                195,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                5..12,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `BoundingBox`,
                                                    token_idx: TokenIdx(
                                                        179,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `ClosedRange`,
                                                    token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `ClosedRange`,
                                                    token_idx: TokenIdx(
                                                        188,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    129,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                130,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        131,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 20,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 26,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 32,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 38,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 44,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    81,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                82,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        83,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 3,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    90,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        93,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 5,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    97,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 1,
                                                    variables: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        100,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 7,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    104,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 2,
                                                    variables: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        107,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 9,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    111,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 3,
                                                    variables: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        114,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 11,
                                        },
                                        Stmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    118,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    119,
                                                ),
                                                frame_var_expr_idx: 14,
                                                frame_var_ident: `i`,
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: None,
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            15,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            frame_var_symbol_idx: 4,
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            128,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    0..5,
                                                ),
                                            ),
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    178,
                                                ),
                                            },
                                            result: 54,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            91,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `xmin`,
                                                    token_idx: TokenIdx(
                                                        92,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            98,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `xmax`,
                                                    token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            105,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `ymin`,
                                                    token_idx: TokenIdx(
                                                        106,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            112,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `ymax`,
                                                    token_idx: TokenIdx(
                                                        113,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            Move,
                                            Move,
                                            Move,
                                            Move,
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                            PatternSymbol::Atom(
                                                1,
                                            ),
                                            PatternSymbol::Atom(
                                                2,
                                            ),
                                            PatternSymbol::Atom(
                                                3,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `xmin`,
                                                0,
                                            ),
                                        ],
                                        [
                                            (
                                                `xmax`,
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                `ymin`,
                                                2,
                                            ),
                                        ],
                                        [
                                            (
                                                `ymax`,
                                                3,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            Mut,
                                            Mut,
                                            Mut,
                                            Mut,
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    93,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            196,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `xmin`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    100,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            196,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `xmax`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    107,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            196,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `ymin`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    114,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            196,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `ymax`,
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    129,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            178,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable {
                                                    ident: `i`,
                                                    expr_idx: 14,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: True,
                                    pattern_ty_constraints: [
                                        (
                                            FrameVariable,
                                            ArenaIdxRange(
                                                4..5,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 3,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 7,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 9,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 11,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 20,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 26,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 32,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 38,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 44,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 54,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 55,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MemoizedField(
                    TypeMemoizedFieldDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `relative_bounding_box`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::raw_contour`,
                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `relative_bounding_box`,
                                item_kind: MemoizedField,
                            },
                            memo_ty: Some(
                                FormTypeExpr {
                                    expr: 0,
                                },
                            ),
                            expr: None,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: Some(
                                        ExprRegion {
                                            data: ExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    EntityNodePath::ImplBlock(
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::raw_contour`,
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_entity_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `RawContour`,
                                                                    token_idx: TokenIdx(
                                                                        61,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                roots: [
                                                    ExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        EntityNodePath::AssociatedItem(
                                            AssociatedItemNodePath::TypeItem(
                                                TypeItemNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `relative_bounding_box`,
                                                            item_kind: MemoizedField,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `RelativeBoundingBox`,
                                                        token_idx: TokenIdx(
                                                            199,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: VarType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            9,
                        ),
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
                                                            EntityNodePath::ImplBlock(
                                                                ImplBlockNodePath::TypeImplBlock(
                                                                    TypeImplBlockNodePath {
                                                                        path: TypeImplBlockPath {
                                                                            module_path: `mnist_classifier::raw_contour`,
                                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 0,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        principal_entity_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `RawContour`,
                                                                            token_idx: TokenIdx(
                                                                                61,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                        roots: [
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                EntityNodePath::AssociatedItem(
                                                    AssociatedItemNodePath::TypeItem(
                                                        TypeItemNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypeItemPath {
                                                                    impl_block: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `relative_bounding_box`,
                                                                    item_kind: MemoizedField,
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `RelativeBoundingBox`,
                                                                token_idx: TokenIdx(
                                                                    199,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: VarType,
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::AssociatedItem(
                                        AssociatedItemNodePath::TypeItem(
                                            TypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `mnist_classifier::raw_contour`,
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `relative_bounding_box`,
                                                        item_kind: MemoizedField,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::SelfValue(
                                            TokenIdx(
                                                201,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                202,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `cc`,
                                                token_idx: TokenIdx(
                                                    203,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                204,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `raw_contours`,
                                                token_idx: TokenIdx(
                                                    205,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                207,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 2,
                                            lbox_token_idx: TokenIdx(
                                                206,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                208,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 4,
                                            dot_token_idx: TokenIdx(
                                                209,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    210,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                214,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                215,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    216,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 5,
                                            dot_token_idx: TokenIdx(
                                                211,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    212,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                213,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 7,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                217,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 8,
                                        },
                                    ],
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
                                    allow_self_value: True,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 8,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 9,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MemoizedField(
                    TypeMemoizedFieldDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `contour_len`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::raw_contour`,
                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `contour_len`,
                                item_kind: MemoizedField,
                            },
                            memo_ty: Some(
                                FormTypeExpr {
                                    expr: 0,
                                },
                            ),
                            expr: None,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: Some(
                                        ExprRegion {
                                            data: ExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    EntityNodePath::ImplBlock(
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::raw_contour`,
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_entity_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `RawContour`,
                                                                    token_idx: TokenIdx(
                                                                        61,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                roots: [
                                                    ExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        EntityNodePath::AssociatedItem(
                                            AssociatedItemNodePath::TypeItem(
                                                TypeItemNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `contour_len`,
                                                            item_kind: MemoizedField,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            221,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
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
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: VarType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            65,
                        ),
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
                                                            EntityNodePath::ImplBlock(
                                                                ImplBlockNodePath::TypeImplBlock(
                                                                    TypeImplBlockNodePath {
                                                                        path: TypeImplBlockPath {
                                                                            module_path: `mnist_classifier::raw_contour`,
                                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 0,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        principal_entity_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `RawContour`,
                                                                            token_idx: TokenIdx(
                                                                                61,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                        roots: [
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                EntityNodePath::AssociatedItem(
                                                    AssociatedItemNodePath::TypeItem(
                                                        TypeItemNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypeItemPath {
                                                                    impl_block: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `contour_len`,
                                                                    item_kind: MemoizedField,
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    221,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: VarType,
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::AssociatedItem(
                                        AssociatedItemNodePath::TypeItem(
                                            TypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `mnist_classifier::raw_contour`,
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `contour_len`,
                                                        item_kind: MemoizedField,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::Literal(
                                            TokenIdx(
                                                227,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                229,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                231,
                                            ),
                                            ident: `i`,
                                            frame_var_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                2,
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                233,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                234,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    235,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 1,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                230,
                                            ),
                                            ropd: 2,
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 4,
                                            dot_token_idx: TokenIdx(
                                                236,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    237,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                238,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                239,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 5,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                232,
                                            ),
                                            ropd: 6,
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                244,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 8,
                                            dot_token_idx: TokenIdx(
                                                245,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    246,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                248,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                2,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                250,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 10,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                249,
                                            ),
                                            ropd: 11,
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 9,
                                            lbox_token_idx: TokenIdx(
                                                247,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 12,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                251,
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                255,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 14,
                                            dot_token_idx: TokenIdx(
                                                256,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    257,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                259,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                2,
                                            ),
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 15,
                                            lbox_token_idx: TokenIdx(
                                                258,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 16,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                260,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        264,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 55,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        268,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 282,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 18,
                                            dot_token_idx: TokenIdx(
                                                265,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    266,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 19,
                                            dot_token_idx: TokenIdx(
                                                269,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    270,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 20,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                267,
                                            ),
                                            ropd: 21,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                263,
                                            ),
                                            item: 22,
                                            rpar_token_idx: TokenIdx(
                                                271,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        278,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 55,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        282,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 282,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 24,
                                            dot_token_idx: TokenIdx(
                                                279,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    280,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 25,
                                            dot_token_idx: TokenIdx(
                                                283,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    284,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 26,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                281,
                                            ),
                                            ropd: 27,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                277,
                                            ),
                                            item: 28,
                                            rpar_token_idx: TokenIdx(
                                                285,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 23,
                                            dot_token_idx: TokenIdx(
                                                272,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    273,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                274,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                275,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 29,
                                            dot_token_idx: TokenIdx(
                                                286,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    287,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                288,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                289,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `contour_len`,
                                            token_idx: TokenIdx(
                                                261,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 30,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                276,
                                            ),
                                            ropd: 31,
                                        },
                                        Expr::Binary {
                                            lopd: 32,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                262,
                                            ),
                                            ropd: 33,
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                293,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 35,
                                            dot_token_idx: TokenIdx(
                                                294,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    295,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                297,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 37,
                                            dot_token_idx: TokenIdx(
                                                298,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    299,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 38,
                                            dot_token_idx: TokenIdx(
                                                300,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    301,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                302,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                303,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                305,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 39,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                304,
                                            ),
                                            ropd: 40,
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 36,
                                            lbox_token_idx: TokenIdx(
                                                296,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 41,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                306,
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                310,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 43,
                                            dot_token_idx: TokenIdx(
                                                311,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    312,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                314,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 44,
                                            lbox_token_idx: TokenIdx(
                                                313,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 45,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                315,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        319,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 55,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        323,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 282,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 47,
                                            dot_token_idx: TokenIdx(
                                                320,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    321,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 48,
                                            dot_token_idx: TokenIdx(
                                                324,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    325,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 49,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                322,
                                            ),
                                            ropd: 50,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                318,
                                            ),
                                            item: 51,
                                            rpar_token_idx: TokenIdx(
                                                326,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        333,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 55,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        337,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 282,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 53,
                                            dot_token_idx: TokenIdx(
                                                334,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    335,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 54,
                                            dot_token_idx: TokenIdx(
                                                338,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    339,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 55,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                336,
                                            ),
                                            ropd: 56,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                332,
                                            ),
                                            item: 57,
                                            rpar_token_idx: TokenIdx(
                                                340,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 52,
                                            dot_token_idx: TokenIdx(
                                                327,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    328,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                329,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                330,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 58,
                                            dot_token_idx: TokenIdx(
                                                341,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    342,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                343,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                344,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `contour_len`,
                                            token_idx: TokenIdx(
                                                316,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 59,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                331,
                                            ),
                                            ropd: 60,
                                        },
                                        Expr::Binary {
                                            lopd: 61,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                317,
                                            ),
                                            ropd: 62,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `contour_len`,
                                            token_idx: TokenIdx(
                                                346,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                3..9,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    241,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                242,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        243,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 13,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    252,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                253,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        254,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 17,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 34,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    223,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        226,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 0,
                                        },
                                        Stmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    228,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    231,
                                                ),
                                                frame_var_expr_idx: 2,
                                                frame_var_ident: `i`,
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            1,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            6,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            frame_var_symbol_idx: 1,
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            240,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    0..3,
                                                ),
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    290,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                291,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        292,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 42,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    307,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                308,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        309,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 46,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 63,
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    345,
                                                ),
                                            },
                                            result: 64,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            224,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `contour_len`,
                                                    token_idx: TokenIdx(
                                                        225,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            Move,
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `contour_len`,
                                                0,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            Mut,
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    226,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            347,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `contour_len`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    241,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            290,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable {
                                                    ident: `i`,
                                                    expr_idx: 2,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: True,
                                    pattern_ty_constraints: [
                                        (
                                            FrameVariable,
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 0,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 13,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 17,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 34,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 42,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 46,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 63,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 64,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 65,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)
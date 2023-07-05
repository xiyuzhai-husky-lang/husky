Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::PropsStruct(
                    PropsStructTypeDefn {
                        path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        decl: PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            implicit_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            36,
                                        ),
                                    },
                                    colon: ColonToken(
                                        TokenIdx(
                                            37,
                                        ),
                                    ),
                                    ty_expr_idx: 1,
                                    initialization: None,
                                },
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `strokes`,
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                    colon: ColonToken(
                                        TokenIdx(
                                            42,
                                        ),
                                    ),
                                    ty_expr_idx: 7,
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
                                                        path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                            Expr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    38,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::Err(
                                                ExprError::Original(
                                                    NoLeftOperandForBinaryOperator {
                                                        binary_token_idx: TokenIdx(
                                                            45,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            Expr::Err(
                                                ExprError::Original(
                                                    NoRightOperandForBinaryOperator {
                                                        punctuation: Closed(
                                                            RemEuclid,
                                                        ),
                                                        punctuation_token_idx: TokenIdx(
                                                            45,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            Expr::List {
                                                lbox_token_idx: TokenIdx(
                                                    44,
                                                ),
                                                items: [
                                                    CommaListItem {
                                                        expr_idx: 3,
                                                        comma_token_idx: None,
                                                    },
                                                ],
                                                rbox_token_idx: TokenIdx(
                                                    46,
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    43,
                                                ),
                                                opd: 4,
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::ExplicitApplication {
                                                function: 5,
                                                argument: 6,
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
                                                            39,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `LineSegmentStroke`,
                                                        token_idx: TokenIdx(
                                                            47,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                                value: 193,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        36,
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
                                                                value: 365,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        41,
                                                    ),
                                                },
                                            },
                                            expr_idx: 7,
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                    51,
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
                                                ident: `ConcaveComponent`,
                                                token_idx: TokenIdx(
                                                    53,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `visualize`,
                            item_kind: MethodFn,
                        },
                        decl: TraitForTypeMethodFnDecl {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    trai_path: TraitPath(`core::visual::Visualize`),
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        51,
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        53,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                            60,
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            trai_path: TraitPath(`core::visual::Visualize`),
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                51,
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                53,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    60,
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
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            trai_path: TraitPath(`core::visual::Visualize`),
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                62,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                63,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    64,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                65,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `visualize`,
                                                token_idx: TokenIdx(
                                                    66,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                67,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                68,
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                ident: `ConcaveComponent`,
                                                token_idx: TokenIdx(
                                                    70,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `norm`,
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
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `norm`,
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
                                                            75,
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
                            2,
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `norm`,
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
                                                                    75,
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
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `norm`,
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
                                                77,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                78,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `hausdorff_norm`,
                                                token_idx: TokenIdx(
                                                    79,
                                                ),
                                            },
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
                                            expr_idx: 1,
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
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 2,
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
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `rel_norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `rel_norm`,
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
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `rel_norm`,
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
                                                            83,
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
                            6,
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `rel_norm`,
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
                                                                    83,
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
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `rel_norm`,
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
                                                85,
                                            ),
                                        ),
                                        Expr::SelfValue(
                                            TokenIdx(
                                                89,
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                90,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    91,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                92,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                93,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                86,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    87,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 2,
                                            dot_token_idx: TokenIdx(
                                                94,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    95,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                96,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                97,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Closed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                88,
                                            ),
                                            ropd: 4,
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
                                            expr_idx: 5,
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
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 6,
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
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `hausdorff_norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `hausdorff_norm`,
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
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `hausdorff_norm`,
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
                                                            101,
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
                            35,
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `hausdorff_norm`,
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
                                                                    101,
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
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `hausdorff_norm`,
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
                                                107,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::SelfValue(
                                            TokenIdx(
                                                111,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                112,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    113,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 2,
                                            dot_token_idx: TokenIdx(
                                                114,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `first`,
                                                token_idx: TokenIdx(
                                                    115,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                116,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                117,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 3,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                118,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 4,
                                            dot_token_idx: TokenIdx(
                                                119,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    120,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                124,
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 6,
                                            dot_token_idx: TokenIdx(
                                                125,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `line_segment`,
                                                token_idx: TokenIdx(
                                                    126,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                127,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                128,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        132,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 405,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 8,
                                            dot_token_idx: TokenIdx(
                                                133,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                135,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                136,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 9,
                                            dot_token_idx: TokenIdx(
                                                137,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    138,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                139,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                140,
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                142,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 11,
                                            dot_token_idx: TokenIdx(
                                                143,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    144,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 12,
                                            dot_token_idx: TokenIdx(
                                                145,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    146,
                                                ),
                                            },
                                        },
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                148,
                                            ),
                                            ident: `i`,
                                            frame_var_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                14,
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                150,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 15,
                                            dot_token_idx: TokenIdx(
                                                151,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    152,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 13,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                147,
                                            ),
                                            ropd: 14,
                                        },
                                        Expr::Field {
                                            owner: 16,
                                            dot_token_idx: TokenIdx(
                                                153,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    154,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 17,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                149,
                                            ),
                                            ropd: 18,
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                159,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 20,
                                            dot_token_idx: TokenIdx(
                                                160,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    161,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                163,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                14,
                                            ),
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 21,
                                            lbox_token_idx: TokenIdx(
                                                162,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 22,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                164,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 23,
                                            dot_token_idx: TokenIdx(
                                                165,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    166,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        170,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 405,
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
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 25,
                                            dot_token_idx: TokenIdx(
                                                171,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `dist_to_point`,
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
                                                    expr_idx: 26,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                175,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 406,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `hausdorff_norm`,
                                            token_idx: TokenIdx(
                                                179,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 28,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                178,
                                            ),
                                            ropd: 29,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `hausdorff_norm`,
                                            token_idx: TokenIdx(
                                                181,
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
                                                        183,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 406,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 31,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ropd: 32,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `hausdorff_norm`,
                                            token_idx: TokenIdx(
                                                185,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                4..10,
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
                                            expr_idx: 33,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    156,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                157,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        158,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 24,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    167,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                168,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        169,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 27,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        176,
                                                    ),
                                                },
                                                condition: Ok(
                                                    30,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                180,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    103,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                                        106,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 0,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    108,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                109,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        110,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 5,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    121,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                122,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        123,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 7,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    129,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
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
                                            initial_value: 10,
                                        },
                                        Stmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    141,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    148,
                                                ),
                                                frame_var_expr_idx: 14,
                                                frame_var_ident: `i`,
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            13,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            18,
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
                                                            155,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    1..4,
                                                ),
                                            ),
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    184,
                                                ),
                                            },
                                            result: 34,
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
                                                            104,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `hausdorff_norm`,
                                                    token_idx: TokenIdx(
                                                        105,
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
                                                `hausdorff_norm`,
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
                                                    106,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            186,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `hausdorff_norm`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    156,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            184,
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
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 7,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 10,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 24,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 27,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 33,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 34,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 35,
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
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `angle_change`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `angle_change`,
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
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `angle_change`,
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
                                                            189,
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
                            32,
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `angle_change`,
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
                                                                    189,
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
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `angle_change`,
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
                                                195,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::SelfValue(
                                            TokenIdx(
                                                200,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                201,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    202,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                204,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                205,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    206,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 4,
                                            dot_token_idx: TokenIdx(
                                                207,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    208,
                                                ),
                                            },
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 2,
                                            lbox_token_idx: TokenIdx(
                                                203,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 5,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                209,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 6,
                                            dot_token_idx: TokenIdx(
                                                210,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    211,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                212,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                213,
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                215,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 8,
                                            dot_token_idx: TokenIdx(
                                                216,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    217,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 9,
                                            dot_token_idx: TokenIdx(
                                                218,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    219,
                                                ),
                                            },
                                        },
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                221,
                                            ),
                                            ident: `i`,
                                            frame_var_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                11,
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                223,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 12,
                                            dot_token_idx: TokenIdx(
                                                224,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    225,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 10,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                220,
                                            ),
                                            ropd: 11,
                                        },
                                        Expr::Field {
                                            owner: 13,
                                            dot_token_idx: TokenIdx(
                                                226,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    227,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 14,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                222,
                                            ),
                                            ropd: 15,
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                232,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 17,
                                            dot_token_idx: TokenIdx(
                                                233,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    234,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                236,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                11,
                                            ),
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 18,
                                            lbox_token_idx: TokenIdx(
                                                235,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 19,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                237,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 20,
                                            dot_token_idx: TokenIdx(
                                                238,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    239,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                240,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                241,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp0`,
                                            token_idx: TokenIdx(
                                                244,
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
                                                        248,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 378,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                250,
                                            ),
                                            Literal::Bool(
                                                True,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `angle_change`,
                                            token_idx: TokenIdx(
                                                242,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 22,
                                            dot_token_idx: TokenIdx(
                                                245,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_to`,
                                                token_idx: TokenIdx(
                                                    246,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                247,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 23,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            249,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 24,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                251,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 25,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                243,
                                            ),
                                            ropd: 26,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp0`,
                                            token_idx: TokenIdx(
                                                252,
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
                                                        254,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 378,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 28,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                253,
                                            ),
                                            ropd: 29,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `angle_change`,
                                            token_idx: TokenIdx(
                                                256,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                3..7,
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
                                                    229,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                230,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        231,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 21,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 27,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 30,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    191,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                                        194,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 0,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    196,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                                        199,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 7,
                                        },
                                        Stmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    214,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    221,
                                                ),
                                                frame_var_expr_idx: 11,
                                                frame_var_ident: `i`,
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            10,
                                                        ),
                                                        kind: LowerOpen,
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
                                            frame_var_symbol_idx: 2,
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            228,
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
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    255,
                                                ),
                                            },
                                            result: 31,
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
                                                            192,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `angle_change`,
                                                    token_idx: TokenIdx(
                                                        193,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            197,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `dp0`,
                                                    token_idx: TokenIdx(
                                                        198,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            Move,
                                            Move,
                                        ],
                                    },
                                    pattern_infos: [
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
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `angle_change`,
                                                0,
                                            ),
                                        ],
                                        [
                                            (
                                                `dp0`,
                                                1,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
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
                                                    194,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            257,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `angle_change`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    199,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            257,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `dp0`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    229,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            255,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable {
                                                    ident: `i`,
                                                    expr_idx: 11,
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
                                                2..3,
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
                                        expr_idx: 7,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 21,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 27,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 30,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 31,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 32,
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
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `bounding_box`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                            260,
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
                            61,
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    260,
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
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                265,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                266,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    267,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                268,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `first`,
                                                token_idx: TokenIdx(
                                                    269,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                270,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                271,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 2,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                272,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                273,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    274,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        279,
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
                                            owner: 5,
                                            dot_token_idx: TokenIdx(
                                                280,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    281,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        286,
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
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                287,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    288,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        293,
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
                                            owner: 9,
                                            dot_token_idx: TokenIdx(
                                                294,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    295,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        300,
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
                                            owner: 11,
                                            dot_token_idx: TokenIdx(
                                                301,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    302,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                304,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 13,
                                            dot_token_idx: TokenIdx(
                                                305,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    306,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 14,
                                            dot_token_idx: TokenIdx(
                                                307,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    308,
                                                ),
                                            },
                                        },
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                310,
                                            ),
                                            ident: `i`,
                                            frame_var_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                16,
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                312,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 17,
                                            dot_token_idx: TokenIdx(
                                                313,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    314,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 15,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                309,
                                            ),
                                            ropd: 16,
                                        },
                                        Expr::Field {
                                            owner: 18,
                                            dot_token_idx: TokenIdx(
                                                315,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    316,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 19,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                311,
                                            ),
                                            ropd: 20,
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                321,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 22,
                                            dot_token_idx: TokenIdx(
                                                322,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    323,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                325,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                16,
                                            ),
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 23,
                                            lbox_token_idx: TokenIdx(
                                                324,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 24,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                326,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 25,
                                            dot_token_idx: TokenIdx(
                                                327,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    328,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `xmin`,
                                            token_idx: TokenIdx(
                                                331,
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
                                                        335,
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
                                                336,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    337,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `xmin`,
                                            token_idx: TokenIdx(
                                                329,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 27,
                                            dot_token_idx: TokenIdx(
                                                332,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `min`,
                                                token_idx: TokenIdx(
                                                    333,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                334,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 29,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                338,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 30,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                330,
                                            ),
                                            ropd: 31,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `xmax`,
                                            token_idx: TokenIdx(
                                                341,
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
                                                        345,
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
                                                346,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    347,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `xmax`,
                                            token_idx: TokenIdx(
                                                339,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 33,
                                            dot_token_idx: TokenIdx(
                                                342,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    343,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                344,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 35,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                348,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 36,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                340,
                                            ),
                                            ropd: 37,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `ymin`,
                                            token_idx: TokenIdx(
                                                351,
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
                                                        355,
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
                                                356,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    357,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `ymin`,
                                            token_idx: TokenIdx(
                                                349,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 39,
                                            dot_token_idx: TokenIdx(
                                                352,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `min`,
                                                token_idx: TokenIdx(
                                                    353,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                354,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 41,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                358,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 42,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                350,
                                            ),
                                            ropd: 43,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `ymax`,
                                            token_idx: TokenIdx(
                                                361,
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
                                                        365,
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
                                            owner: 46,
                                            dot_token_idx: TokenIdx(
                                                366,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    367,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `ymax`,
                                            token_idx: TokenIdx(
                                                359,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 45,
                                            dot_token_idx: TokenIdx(
                                                362,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    363,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                364,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 47,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                368,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 48,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                360,
                                            ),
                                            ropd: 49,
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
                                                374,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `xmax`,
                                            token_idx: TokenIdx(
                                                376,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 52,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                373,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 53,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            375,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 54,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                377,
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
                                                381,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `ymax`,
                                            token_idx: TokenIdx(
                                                383,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 56,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                380,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 57,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            382,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 58,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                384,
                                            ),
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 51,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                371,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 55,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            378,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 59,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            385,
                                                        ),
                                                    ),
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                386,
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
                                                        370,
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
                                                        372,
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
                                                        379,
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
                                                    318,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                319,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        320,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 26,
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
                                        Stmt::Eval {
                                            expr_idx: 50,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    262,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                263,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        264,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 4,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    275,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                                        278,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 6,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    282,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                                        285,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 8,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    289,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                                        292,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 10,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    296,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                                        299,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 12,
                                        },
                                        Stmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    303,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    310,
                                                ),
                                                frame_var_expr_idx: 16,
                                                frame_var_ident: `i`,
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            15,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            20,
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
                                                            317,
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
                                                    369,
                                                ),
                                            },
                                            result: 60,
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
                                                            276,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `xmin`,
                                                    token_idx: TokenIdx(
                                                        277,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            283,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `xmax`,
                                                    token_idx: TokenIdx(
                                                        284,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            290,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `ymin`,
                                                    token_idx: TokenIdx(
                                                        291,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            297,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `ymax`,
                                                    token_idx: TokenIdx(
                                                        298,
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
                                                    278,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            387,
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
                                                    285,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            387,
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
                                                    292,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            387,
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
                                                    299,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            387,
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
                                                    318,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            369,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable {
                                                    ident: `i`,
                                                    expr_idx: 16,
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
                                        expr_idx: 4,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 6,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 8,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 10,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 12,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
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
                                        kind: EvalExpr,
                                        expr_idx: 50,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 60,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 61,
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
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `relative_bounding_box`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                            390,
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
                            6,
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    390,
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
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                392,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                393,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `line_segment_sketch`,
                                                token_idx: TokenIdx(
                                                    394,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                395,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    396,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                400,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                401,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    402,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 2,
                                            dot_token_idx: TokenIdx(
                                                397,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    398,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                399,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                403,
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
                                            expr_idx: 5,
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
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 6,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `line_segment`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `line_segment`,
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
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `line_segment`,
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                        ident: `LineSegment`,
                                                        token_idx: TokenIdx(
                                                            409,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                            14,
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `line_segment`,
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                ident: `LineSegment`,
                                                                token_idx: TokenIdx(
                                                                    409,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                        AssociatedItemNodePath::TypeItem(
                                            TypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `line_segment`,
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
                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                413,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                414,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    415,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 2,
                                            dot_token_idx: TokenIdx(
                                                416,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `first`,
                                                token_idx: TokenIdx(
                                                    417,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                418,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                419,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 3,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                420,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 4,
                                            dot_token_idx: TokenIdx(
                                                421,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    422,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 5,
                                            dot_token_idx: TokenIdx(
                                                423,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `clone`,
                                                token_idx: TokenIdx(
                                                    424,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                425,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                426,
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                428,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                429,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    430,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 8,
                                            dot_token_idx: TokenIdx(
                                                431,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `last`,
                                                token_idx: TokenIdx(
                                                    432,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                433,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                434,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 9,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                435,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                436,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    437,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 11,
                                            dot_token_idx: TokenIdx(
                                                438,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `clone`,
                                                token_idx: TokenIdx(
                                                    439,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                440,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                441,
                                            ),
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                412,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 6,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            427,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 12,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                442,
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
                                                    ident: `LineSegment`,
                                                    token_idx: TokenIdx(
                                                        411,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 13,
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
                                        expr_idx: 13,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 14,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `start`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `start`,
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
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `start`,
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
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        ident: `Point2d`,
                                                        token_idx: TokenIdx(
                                                            448,
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
                                            kind: ReturnType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            6,
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `start`,
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
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                ident: `Point2d`,
                                                                token_idx: TokenIdx(
                                                                    448,
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
                                                    kind: ReturnType,
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
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `start`,
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
                                                450,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                451,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    452,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                453,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `first`,
                                                token_idx: TokenIdx(
                                                    454,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                455,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                456,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 2,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                457,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                458,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    459,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 4,
                                            dot_token_idx: TokenIdx(
                                                460,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `clone`,
                                                token_idx: TokenIdx(
                                                    461,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                462,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                463,
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
                                            expr_idx: 5,
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
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 6,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `end`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `end`,
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
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `end`,
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
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        ident: `Point2d`,
                                                        token_idx: TokenIdx(
                                                            469,
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
                                            kind: ReturnType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            6,
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `end`,
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
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                ident: `Point2d`,
                                                                token_idx: TokenIdx(
                                                                    469,
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
                                                    kind: ReturnType,
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
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `end`,
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
                                                471,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                472,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    473,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                474,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `last`,
                                                token_idx: TokenIdx(
                                                    475,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                476,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                477,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 2,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                478,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                479,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    480,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 4,
                                            dot_token_idx: TokenIdx(
                                                481,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `clone`,
                                                token_idx: TokenIdx(
                                                    482,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                483,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                484,
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
                                            expr_idx: 5,
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
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 6,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `displacement`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `displacement`,
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
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `displacement`,
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
                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                        ident: `Vector2d`,
                                                        token_idx: TokenIdx(
                                                            490,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                ImplBlockNodePath::TypeImplBlock(
                                                                    TypeImplBlockNodePath {
                                                                        path: TypeImplBlockPath {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `displacement`,
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
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                ident: `Vector2d`,
                                                                token_idx: TokenIdx(
                                                                    490,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                        AssociatedItemNodePath::TypeItem(
                                            TypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `displacement`,
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
                                                492,
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                493,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `line_segment`,
                                                token_idx: TokenIdx(
                                                    494,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                495,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                496,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                497,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    498,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                499,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                500,
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
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `start_tangent`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `start_tangent`,
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
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `start_tangent`,
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
                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                        ident: `Vector2d`,
                                                        token_idx: TokenIdx(
                                                            506,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `start_tangent`,
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
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                ident: `Vector2d`,
                                                                token_idx: TokenIdx(
                                                                    506,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                        AssociatedItemNodePath::TypeItem(
                                            TypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `start_tangent`,
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
                                                508,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                509,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    510,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                511,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `first`,
                                                token_idx: TokenIdx(
                                                    512,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                513,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                514,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 2,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                515,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 3,
                                            dot_token_idx: TokenIdx(
                                                516,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    517,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                518,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                519,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `end_tangent`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `end_tangent`,
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
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    ident: `ConcaveComponent`,
                                                                    token_idx: TokenIdx(
                                                                        70,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `end_tangent`,
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
                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                        ident: `Vector2d`,
                                                        token_idx: TokenIdx(
                                                            525,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ident: `ConcaveComponent`,
                                                                            token_idx: TokenIdx(
                                                                                70,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `end_tangent`,
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
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                ident: `Vector2d`,
                                                                token_idx: TokenIdx(
                                                                    525,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                        AssociatedItemNodePath::TypeItem(
                                            TypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `end_tangent`,
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
                                                527,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                528,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    529,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                530,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `last`,
                                                token_idx: TokenIdx(
                                                    531,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                532,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                533,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 2,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                534,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 3,
                                            dot_token_idx: TokenIdx(
                                                535,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    536,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                537,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                538,
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
    ],
)
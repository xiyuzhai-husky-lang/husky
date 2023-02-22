Ok(
    DeclSheet {
        decls: [
            Ok(
                Decl::Type(
                    TypeDecl::RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            ast_idx: 22,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    12,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    9,
                                                ),
                                                items: ArenaIdxRange(
                                                    0..0,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    10,
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    11,
                                                ),
                                                opd: 1,
                                            },
                                            Expr::Application {
                                                function: 2,
                                                argument: 3,
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    17,
                                                ),
                                                items: ArenaIdxRange(
                                                    5..5,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    18,
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    19,
                                                ),
                                                opd: 5,
                                            },
                                            Expr::Application {
                                                function: 6,
                                                argument: 7,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    13,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    20,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: EntityPath::ModuleItem(
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
                                            kind: FieldType,
                                            expr: 4,
                                        },
                                        ExprRoot {
                                            kind: FieldType,
                                            expr: 8,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                            fields: [
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: `matches`,
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            8,
                                        ),
                                    },
                                    ty: 4,
                                },
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: `others`,
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            16,
                                        ),
                                    },
                                    ty: 8,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        14,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        21,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                        },
                    ),
                ),
            ),
            Ok(
                Decl::Form(
                    FormDecl::Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                            ast_idx: 24,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    151,
                                                ),
                                                items: ArenaIdxRange(
                                                    0..0,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    152,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Application {
                                                function: 0,
                                                argument: 1,
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    150,
                                                ),
                                                opd: 2,
                                            },
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    157,
                                                ),
                                                items: ArenaIdxRange(
                                                    4..4,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    158,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    161,
                                                ),
                                                opd: 5,
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 2,
                                                entity_path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Bracketed {
                                                lpar_token_idx: TokenIdx(
                                                    160,
                                                ),
                                                item: 6,
                                                rpar_token_idx: TokenIdx(
                                                    163,
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    165,
                                                ),
                                                opd: 7,
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 8,
                                                opr: Curry,
                                                opr_token_idx: TokenIdx(
                                                    164,
                                                ),
                                                ropd: 9,
                                            },
                                            Expr::ApplicationOrRitchieCall {
                                                function: 4,
                                                lpar_token_idx: TokenIdx(
                                                    159,
                                                ),
                                                argument: 10,
                                                rpar_token_idx: TokenIdx(
                                                    167,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 3,
                                                entity_path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                    153,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    162,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    166,
                                                ),
                                                ident: `f32`,
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    170,
                                                ),
                                                ident: `FermiMatchResult`,
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                            data: [
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `concave_components`,
                                                        token_idx: TokenIdx(
                                                            148,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `templates`,
                                                        token_idx: TokenIdx(
                                                            155,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                            Parameter,
                                        ],
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 318,
                                                            },
                                                        ),
                                                    ),
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 382,
                                                            },
                                                        ),
                                                    ),
                                                    1,
                                                ),
                                            ],
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
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    ident: `concave_components`,
                                                    access_start: TokenIdx(
                                                        149,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `templates`,
                                                    access_start: TokenIdx(
                                                        156,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 3,
                                            },
                                            RegularParameter {
                                                pattern: 1,
                                                ty: 11,
                                            },
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr: 12,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        147,
                                    ),
                                },
                                parameters: [
                                    RegularParameterDeclPattern {
                                        pattern: 0,
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                149,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                156,
                                            ),
                                        },
                                        ty: 11,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            154,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        168,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        169,
                                    ),
                                },
                            ),
                            return_ty: Ok(
                                OutputTypeExpr {
                                    expr: 12,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        171,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                Decl::ImplBlock(
                    ImplBlockDecl::TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 23,
                            impl_block: ImplBlock {
                                id: ImplBlockId {
                                    module_path: `mnist_classifier::fermi`,
                                    impl_block_kind: ImplBlockKind::Type {
                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    },
                                },
                                ast_idx: 23,
                                body: ArenaIdxRange(
                                    12..15,
                                ),
                                variant: ImplBlockVariant::Type {
                                    ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        25,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::ImplBlock(
                                            ImplBlockId {
                                                module_path: `mnist_classifier::fermi`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                ident: `FermiMatchResult`,
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        ident: `norm`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: `mnist_classifier::fermi`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            },
                                        },
                                        ident: `norm`,
                                    },
                                    path: Some(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ident: `norm`,
                                                ty_item_kind: Memo,
                                            },
                                        ),
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::fermi`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            },
                                        },
                                        ast_idx: 23,
                                        body: ArenaIdxRange(
                                            12..15,
                                        ),
                                        variant: ImplBlockVariant::Type {
                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        },
                                    },
                                    ast_idx: 12,
                                    ident: `norm`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: Accessibility::PublicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 12,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclExprPath::ImplBlock(
                                                            ImplBlockId {
                                                                module_path: `mnist_classifier::fermi`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                ident: `FermiMatchResult`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                        module_path: `mnist_classifier::fermi`,
                                                        impl_block_kind: ImplBlockKind::Type {
                                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        },
                                                    },
                                                    ident: `norm`,
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
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                        29,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
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
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            28,
                                        ),
                                    },
                                ),
                                return_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            30,
                                        ),
                                    },
                                ),
                            },
                        ),
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
                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        ident: `rel_norm`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: `mnist_classifier::fermi`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            },
                                        },
                                        ident: `rel_norm`,
                                    },
                                    path: Some(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ident: `rel_norm`,
                                                ty_item_kind: Memo,
                                            },
                                        ),
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::fermi`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            },
                                        },
                                        ast_idx: 23,
                                        body: ArenaIdxRange(
                                            12..15,
                                        ),
                                        variant: ImplBlockVariant::Type {
                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        },
                                    },
                                    ast_idx: 13,
                                    ident: `rel_norm`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: Accessibility::PublicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 13,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclExprPath::ImplBlock(
                                                            ImplBlockId {
                                                                module_path: `mnist_classifier::fermi`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                ident: `FermiMatchResult`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                        module_path: `mnist_classifier::fermi`,
                                                        impl_block_kind: ImplBlockKind::Type {
                                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        },
                                                    },
                                                    ident: `rel_norm`,
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
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                        67,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
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
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            66,
                                        ),
                                    },
                                ),
                                return_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            68,
                                        ),
                                    },
                                ),
                            },
                        ),
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
                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        ident: `angle_change_norm`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: `mnist_classifier::fermi`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            },
                                        },
                                        ident: `angle_change_norm`,
                                    },
                                    path: Some(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ident: `angle_change_norm`,
                                                ty_item_kind: Memo,
                                            },
                                        ),
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::fermi`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            },
                                        },
                                        ast_idx: 23,
                                        body: ArenaIdxRange(
                                            12..15,
                                        ),
                                        variant: ImplBlockVariant::Type {
                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        },
                                    },
                                    ast_idx: 14,
                                    ident: `angle_change_norm`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: Accessibility::PublicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 14,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclExprPath::ImplBlock(
                                                            ImplBlockId {
                                                                module_path: `mnist_classifier::fermi`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                ident: `FermiMatchResult`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                        module_path: `mnist_classifier::fermi`,
                                                        impl_block_kind: ImplBlockKind::Type {
                                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        },
                                                    },
                                                    ident: `angle_change_norm`,
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
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                        105,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
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
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            104,
                                        ),
                                    },
                                ),
                                return_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            106,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)
Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        ),
                    ),
                ),
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
                                            DeclRegionPath::Entity(
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
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        12,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::List {
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
                                                Expr::Prefix {
                                                    opr: Option,
                                                    opr_token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                    opd: 1,
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 2,
                                                    argument: 3,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
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
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        19,
                                                    ),
                                                    opd: 5,
                                                },
                                                Expr::ExplicitApplication {
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
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                lcurl: LeftCurlyBraceToken(
                                    TokenIdx(
                                        6,
                                    ),
                                ),
                                field_comma_list: (
                                    [
                                        RegularStructFieldPattern {
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    7,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    8,
                                                ),
                                            ),
                                            ty: 4,
                                        },
                                        RegularStructFieldPattern {
                                            ident_token: IdentToken {
                                                ident: `others`,
                                                token_idx: TokenIdx(
                                                    15,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    16,
                                                ),
                                            ),
                                            ty: 8,
                                        },
                                    ],
                                    [
                                        CommaToken(
                                            TokenIdx(
                                                14,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                21,
                                            ),
                                        ),
                                    ],
                                    Ok(
                                        (),
                                    ),
                                ),
                                rcurl: Ok(
                                    RightCurlyBraceToken(
                                        TokenIdx(
                                            22,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FormDecl::Fn(
                            FnDecl {
                                path: FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                ast_idx: 24,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::List {
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
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 0,
                                                    argument: 1,
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        150,
                                                    ),
                                                    opd: 2,
                                                },
                                                Expr::List {
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
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        161,
                                                    ),
                                                    opd: 5,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                Expr::Prefix {
                                                    opr: Option,
                                                    opr_token_idx: TokenIdx(
                                                        165,
                                                    ),
                                                    opd: 7,
                                                },
                                                Expr::Binary {
                                                    lopd: 8,
                                                    opr: Curry,
                                                    opr_token_idx: TokenIdx(
                                                        164,
                                                    ),
                                                    ropd: 9,
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 4,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        159,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        10..11,
                                                    ),
                                                    commas: [],
                                                    rpar_token_idx: TokenIdx(
                                                        167,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
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
                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                    PatternExpr::Ident {
                                                        ident_token: IdentToken {
                                                            ident: `concave_components`,
                                                            token_idx: TokenIdx(
                                                                148,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Ident {
                                                        ident_token: IdentToken {
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
                                                        `concave_components`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `templates`,
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
                                                        access_start: TokenIdx(
                                                            149,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            ident: `concave_components`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            156,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            ident: `templates`,
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
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                parameter_decl_list: Ok(
                                    ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                147,
                                            ),
                                        ),
                                        parameters: [
                                            ExplicitParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        149,
                                                    ),
                                                ),
                                                ty: 3,
                                            },
                                            ExplicitParameterDeclPattern {
                                                pattern: 1,
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        156,
                                                    ),
                                                ),
                                                ty: 11,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    154,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rpar: Ok(
                                            RightParenthesisToken(
                                                TokenIdx(
                                                    168,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken(
                                        TokenIdx(
                                            169,
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 12,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            171,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Impl(
                    ImplBlockId::Type(
                        TypeImplBlockId {
                            module: `mnist_classifier::fermi`,
                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Decl::Impl(
                        ImplDecl::Type(
                            TypeImplBlockDecl {
                                ast_idx: 23,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module: `mnist_classifier::fermi`,
                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            23,
                                        ),
                                    },
                                    ast_idx: 23,
                                    body: ArenaIdxRange(
                                        12..15,
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
                                ty: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            25,
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
                                                        module: `mnist_classifier::fermi`,
                                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module: `mnist_classifier::fermi`,
                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `norm`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ident: `norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module: `mnist_classifier::fermi`,
                                                    ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `norm`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    ident: `norm`,
                                                    ty_item_kind: Memo,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module: `mnist_classifier::fermi`,
                                                    ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        23,
                                                    ),
                                                },
                                                ast_idx: 23,
                                                body: ArenaIdxRange(
                                                    12..15,
                                                ),
                                            },
                                        ),
                                        ast_idx: 12,
                                        ident: `norm`,
                                        associated_item_kind: TypeItem(
                                            Memo,
                                        ),
                                        accessibility: Visibility::PublicUnder(
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
                                                            DeclRegionPath::Impl(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module: `mnist_classifier::fermi`,
                                                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module: `mnist_classifier::fermi`,
                                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                        CurryToken(
                                            TokenIdx(
                                                28,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                30,
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module: `mnist_classifier::fermi`,
                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `rel_norm`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ident: `rel_norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module: `mnist_classifier::fermi`,
                                                    ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `rel_norm`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    ident: `rel_norm`,
                                                    ty_item_kind: Memo,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module: `mnist_classifier::fermi`,
                                                    ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        23,
                                                    ),
                                                },
                                                ast_idx: 23,
                                                body: ArenaIdxRange(
                                                    12..15,
                                                ),
                                            },
                                        ),
                                        ast_idx: 13,
                                        ident: `rel_norm`,
                                        associated_item_kind: TypeItem(
                                            Memo,
                                        ),
                                        accessibility: Visibility::PublicUnder(
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
                                                            DeclRegionPath::Impl(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module: `mnist_classifier::fermi`,
                                                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module: `mnist_classifier::fermi`,
                                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `rel_norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                        CurryToken(
                                            TokenIdx(
                                                66,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                68,
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module: `mnist_classifier::fermi`,
                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `angle_change_norm`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ident: `angle_change_norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module: `mnist_classifier::fermi`,
                                                    ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `angle_change_norm`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    ident: `angle_change_norm`,
                                                    ty_item_kind: Memo,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module: `mnist_classifier::fermi`,
                                                    ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        23,
                                                    ),
                                                },
                                                ast_idx: 23,
                                                body: ArenaIdxRange(
                                                    12..15,
                                                ),
                                            },
                                        ),
                                        ast_idx: 14,
                                        ident: `angle_change_norm`,
                                        associated_item_kind: TypeItem(
                                            Memo,
                                        ),
                                        accessibility: Visibility::PublicUnder(
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
                                                            DeclRegionPath::Impl(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module: `mnist_classifier::fermi`,
                                                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module: `mnist_classifier::fermi`,
                                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `angle_change_norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                        CurryToken(
                                            TokenIdx(
                                                104,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                106,
                                            ),
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
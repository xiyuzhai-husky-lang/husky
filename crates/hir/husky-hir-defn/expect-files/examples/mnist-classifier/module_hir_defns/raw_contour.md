[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        fields: [
                            PropsStructFieldHirDecl {
                                ident: `cc`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: true,
                                    },
                                ),
                                initialization: None,
                            },
                            PropsStructFieldHirDecl {
                                ident: `points`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: false,
                                    },
                                ),
                                initialization: None,
                            },
                        ],
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `cc`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `points`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 305,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 263,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `row`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `j`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 2,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 3,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 1,
                                            opr: Shift(
                                                Shr,
                                            ),
                                            ropd: 4,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::R32(
                                                3,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 5,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 6,
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 7,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `row`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 305,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 263,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `row`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `j`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 1,
                                            opr: Shift(
                                                Shr,
                                            ),
                                            ropd: 2,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 3,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 4,
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 5,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `row`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 305,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 263,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `row`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `j`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 2,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 3,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 1,
                                            opr: Shift(
                                                Shr,
                                            ),
                                            ropd: 4,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 5,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 6,
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 7,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `row`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 3,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 309,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 310,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 263,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `row_above`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `row_below`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `j`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            25,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 1,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 17,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    2,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: StackPure {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    3,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: StackPure {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        3,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 5,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 17,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    6,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: StackPure {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        2,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    7,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: StackPure {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        3,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 301,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 300,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 302,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 303,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 301,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 300,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 303,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 300,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                16..19,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 12,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 13,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 6,
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 7,
                                                    stmts: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 8,
                                                    stmts: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ],
                                            match_target: 10,
                                        },
                                        Eval {
                                            expr_idx: 14,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 16,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 17,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 18,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 19,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 11,
                                                    stmts: ArenaIdxRange(
                                                        6..7,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 14,
                                                    stmts: ArenaIdxRange(
                                                        7..8,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 15,
                                                    stmts: ArenaIdxRange(
                                                        8..9,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 16,
                                                    stmts: ArenaIdxRange(
                                                        9..10,
                                                    ),
                                                },
                                            ],
                                            match_target: 15,
                                        },
                                        Eval {
                                            expr_idx: 21,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 22,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 23,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 20,
                                                    stmts: ArenaIdxRange(
                                                        11..12,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 21,
                                                    stmts: ArenaIdxRange(
                                                        12..13,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 22,
                                                    stmts: ArenaIdxRange(
                                                        13..14,
                                                    ),
                                                },
                                            ],
                                            match_target: 20,
                                        },
                                        Eval {
                                            expr_idx: 24,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 4,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 8,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 3,
                                                    stmts: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 9,
                                                    stmts: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 10,
                                                    stmts: ArenaIdxRange(
                                                        10..11,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 17,
                                                    stmts: ArenaIdxRange(
                                                        14..15,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 23,
                                                    stmts: ArenaIdxRange(
                                                        15..16,
                                                    ),
                                                },
                                            ],
                                            match_target: 9,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 311,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 312,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                3,
                                            ),
                                        ),
                                        OneOf {
                                            options: ArenaIdxRange(
                                                4..6,
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                2,
                                            ),
                                        ),
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                2,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                3,
                                            ),
                                        ),
                                        OneOf {
                                            options: ArenaIdxRange(
                                                12..14,
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                2,
                                            ),
                                        ),
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                3,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        OneOf {
                                            options: ArenaIdxRange(
                                                18..20,
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                2,
                                            ),
                                        ),
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `row_above`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `row_below`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `pixel_pair_above`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `pixel_pair_below`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 314,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 315,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `inward`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `outward`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            19,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 1,
                                            opr: As,
                                            ropd: 2,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 4,
                                            opr: As,
                                            ropd: 5,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 3,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 6,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 7,
                                            opr: As,
                                            ropd: 8,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                2,
                                            ),
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 9,
                                            ident: `last_bits`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::raw_bits::r32(0)::last_bits`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    10,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 13,
                                            opr: As,
                                            ropd: 14,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 16,
                                        },
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                4..6,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 15,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 17,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Const,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 18,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 11,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 5,
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 6,
                                                    stmts: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 7,
                                                    stmts: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ],
                                            match_target: 12,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 316,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                2,
                                            ),
                                        ),
                                        OneOf {
                                            options: ArenaIdxRange(
                                                2..5,
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                3,
                                            ),
                                        ),
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `inward`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `outward`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `raw_angle_change`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 3,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 4,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 309,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 310,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 263,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 318,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `row_above`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `row_below`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `j`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `inward_direction`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            35,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 1,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 17,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    2,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: StackPure {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    3,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: StackPure {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        3,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 5,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 17,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    6,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: StackPure {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        2,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    7,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: StackPure {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        3,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 302,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 301,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 303,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 302,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 301,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 303,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 301,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 300,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 300,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 302,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 303,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 272,
                                                                        },
                                                                    ),
                                                                ),
                                                                ident: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 302,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::Unreachable,
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                26..29,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 12,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 13,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 4,
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 7,
                                                    stmts: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 8,
                                                    stmts: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ],
                                            match_target: 10,
                                        },
                                        Eval {
                                            expr_idx: 15,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 16,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 18,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 19,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 20,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 13,
                                                    stmts: ArenaIdxRange(
                                                        7..8,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 14,
                                                    stmts: ArenaIdxRange(
                                                        8..9,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 15,
                                                    stmts: ArenaIdxRange(
                                                        9..10,
                                                    ),
                                                },
                                            ],
                                            match_target: 17,
                                        },
                                        Eval {
                                            expr_idx: 21,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 22,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 10,
                                                    stmts: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 11,
                                                    stmts: ArenaIdxRange(
                                                        6..7,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 12,
                                                    stmts: ArenaIdxRange(
                                                        10..11,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 16,
                                                    stmts: ArenaIdxRange(
                                                        11..12,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 17,
                                                    stmts: ArenaIdxRange(
                                                        12..13,
                                                    ),
                                                },
                                            ],
                                            match_target: 14,
                                        },
                                        Eval {
                                            expr_idx: 24,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 26,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 27,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 28,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 24,
                                                    stmts: ArenaIdxRange(
                                                        15..16,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 25,
                                                    stmts: ArenaIdxRange(
                                                        16..17,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 26,
                                                    stmts: ArenaIdxRange(
                                                        17..18,
                                                    ),
                                                },
                                            ],
                                            match_target: 25,
                                        },
                                        Eval {
                                            expr_idx: 29,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 22,
                                                    stmts: ArenaIdxRange(
                                                        14..15,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 23,
                                                    stmts: ArenaIdxRange(
                                                        18..19,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 27,
                                                    stmts: ArenaIdxRange(
                                                        19..20,
                                                    ),
                                                },
                                            ],
                                            match_target: 23,
                                        },
                                        Eval {
                                            expr_idx: 31,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 32,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 33,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 31,
                                                    stmts: ArenaIdxRange(
                                                        21..22,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 32,
                                                    stmts: ArenaIdxRange(
                                                        22..23,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 33,
                                                    stmts: ArenaIdxRange(
                                                        23..24,
                                                    ),
                                                },
                                            ],
                                            match_target: 30,
                                        },
                                        Eval {
                                            expr_idx: 34,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 4,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 8,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 3,
                                                    stmts: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 9,
                                                    stmts: ArenaIdxRange(
                                                        13..14,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 18,
                                                    stmts: ArenaIdxRange(
                                                        20..21,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 28,
                                                    stmts: ArenaIdxRange(
                                                        24..25,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 34,
                                                    stmts: ArenaIdxRange(
                                                        25..26,
                                                    ),
                                                },
                                            ],
                                            match_target: 9,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 311,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 312,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                2,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                3,
                                            ),
                                        ),
                                        OneOf {
                                            options: ArenaIdxRange(
                                                5..7,
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                2,
                                            ),
                                        ),
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 275,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 273,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                3,
                                            ),
                                        ),
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                2,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                2,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                3,
                                            ),
                                        ),
                                        OneOf {
                                            options: ArenaIdxRange(
                                                19..22,
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 274,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 276,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                3,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            R32(
                                                2,
                                            ),
                                        ),
                                        OneOf {
                                            options: ArenaIdxRange(
                                                29..31,
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `row_above`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `row_below`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `inward_direction`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `pixel_pair_above`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `pixel_pair_below`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::CaseVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        fields: [
                            PropsStructFieldHirDecl {
                                ident: `prev1`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                                initialization: None,
                            },
                            PropsStructFieldHirDecl {
                                ident: `prev2`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                                initialization: None,
                            },
                        ],
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `prev1`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `prev2`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: false,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 253,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `points`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            29,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 1,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                2,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 4,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 5,
                                        },
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 3,
                                            items: [
                                                6,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 9,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 10,
                                        },
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 8,
                                            items: [
                                                11,
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 14,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 16,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 15,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 17,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                TermF32Literal {
                                                    value: OrderedFloat(
                                                        2.0,
                                                    ),
                                                    text: "2.0f32",
                                                },
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 18,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 19,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 21,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 23,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 22,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 24,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                TermF32Literal {
                                                    value: OrderedFloat(
                                                        2.0,
                                                    ),
                                                    text: "2.0f32",
                                                },
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 25,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 26,
                                        },
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            function_hir_eager_expr_idx: 13,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 15,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    20,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 15,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    27,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..5,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 7,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 12,
                                        },
                                        Eval {
                                            expr_idx: 28,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 295,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 323,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 324,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `points`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `N`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `p0`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `p2`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                template_arguments: [],
                                                always_copyable: false,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 277,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `cc`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            308,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::NewList {
                                            items: [],
                                        },
                                        HirEagerExprData::AssociatedFn {
                                            associated_item_path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssociatedFunctionFn`),
                                            ),
                                        },
                                        HirEagerExprData::AssociatedFunctionFnCall {
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssociatedFunctionFn`),
                                            ),
                                            function_hir_eager_expr_idx: 2,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                29,
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 6,
                                            ident: `mask`,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 8,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 9,
                                        },
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 7,
                                            items: [
                                                10,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 12,
                                            ident: `mask`,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 13,
                                            items: [
                                                14,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 16,
                                            opr: Shift(
                                                Shl,
                                            ),
                                            ropd: 17,
                                        },
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 19,
                                            opr: Shift(
                                                Shl,
                                            ),
                                            ropd: 20,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 22,
                                            items: [
                                                23,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 25,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 26,
                                        },
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 27,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 28,
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 29,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 30,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 32,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 33,
                                        },
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 34,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 35,
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 36,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 37,
                                        },
                                        HirEagerExprData::Prefix {
                                            opr: BitNot,
                                            opd_hir_expr_idx: 38,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 31,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 39,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 24,
                                            opr: Assign,
                                            ropd: 40,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                29,
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 44,
                                            items: [
                                                45,
                                            ],
                                        },
                                        HirEagerExprData::NewList {
                                            items: [],
                                        },
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 49,
                                            items: [
                                                50,
                                            ],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 51,
                                            ident: `ctz`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::raw_bits::r32(0)::ctz`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 53,
                                            ident: `mask`,
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 55,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 56,
                                        },
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 54,
                                            items: [
                                                57,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 59,
                                            ident: `mask`,
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 60,
                                            items: [
                                                61,
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            13,
                                        ),
                                        HirEagerExprData::Variable(
                                            14,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 63,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 17,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    64,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        12,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 17,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    65,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        13,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    66,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        11,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Variable(
                                            15,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 74,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 76,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 78,
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            16,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 80,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 81,
                                        },
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Variable(
                                            17,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 83,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 84,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 82,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 85,
                                        },
                                        HirEagerExprData::Variable(
                                            15,
                                        ),
                                        HirEagerExprData::Variable(
                                            18,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 87,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 88,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 86,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 89,
                                        },
                                        HirEagerExprData::Prefix {
                                            opr: Not,
                                            opd_hir_expr_idx: 90,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            13,
                                        ),
                                        HirEagerExprData::Variable(
                                            14,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Variable(
                                            15,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 92,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 17,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    93,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        12,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 17,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    94,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        13,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    95,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        11,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 74,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    96,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        14,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            15,
                                        ),
                                        HirEagerExprData::Variable(
                                            25,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 98,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 74,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    99,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        14,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 74,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    100,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        24,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 102,
                                            items: [
                                                103,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 105,
                                            items: [
                                                106,
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 108,
                                            opr: Shift(
                                                Shl,
                                            ),
                                            ropd: 109,
                                        },
                                        HirEagerExprData::Prefix {
                                            opr: BitNot,
                                            opd_hir_expr_idx: 110,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 107,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 111,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 104,
                                            opr: Assign,
                                            ropd: 112,
                                        },
                                        HirEagerExprData::Variable(
                                            26,
                                        ),
                                        HirEagerExprData::Variable(
                                            19,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 116,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 115,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 117,
                                        },
                                        HirEagerExprData::Variable(
                                            20,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 120,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 119,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 121,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 118,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 122,
                                        },
                                        HirEagerExprData::Variable(
                                            24,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 124,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 125,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 123,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 126,
                                        },
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 129,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 128,
                                            opr: Comparison(
                                                Neq,
                                            ),
                                            ropd: 130,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 127,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 131,
                                        },
                                        HirEagerExprData::Variable(
                                            23,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 133,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 134,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 132,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 135,
                                        },
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 137,
                                            ident: `last`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            SelfPlace,
                                                        ),
                                                        HirTermSymbolResolution::SelfPlace(
                                                            MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        9,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 138,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 140,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 73,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    141,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        9,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 139,
                                            opr: Assign,
                                            ropd: 142,
                                        },
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::AssociatedFn {
                                            associated_item_path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::AssociatedFunctionFnCall {
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
                                            ),
                                            function_hir_eager_expr_idx: 145,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    146,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        10,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    147,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        11,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 144,
                                            ident: `push`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            SelfLifetime,
                                                        ),
                                                        HirTermSymbolResolution::SelfLifetime,
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 44,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    148,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            23,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 151,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 150,
                                            opr: Assign,
                                            ropd: 152,
                                        },
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 155,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 154,
                                            opr: Assign,
                                            ropd: 156,
                                        },
                                        HirEagerExprData::Variable(
                                            19,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 159,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 158,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 160,
                                        },
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 162,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 163,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 161,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 164,
                                        },
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 166,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 167,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 165,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 168,
                                        },
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 170,
                                            ident: `last`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            SelfPlace,
                                                        ),
                                                        HirTermSymbolResolution::SelfPlace(
                                                            MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        9,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 171,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::AssociatedFn {
                                            associated_item_path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::AssociatedFunctionFnCall {
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
                                            ),
                                            function_hir_eager_expr_idx: 173,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    174,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        10,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    175,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        11,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 172,
                                            opr: Assign,
                                            ropd: 176,
                                        },
                                        HirEagerExprData::Variable(
                                            23,
                                        ),
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 178,
                                            opr: Assign,
                                            ropd: 179,
                                        },
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Variable(
                                            24,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 181,
                                            opr: Assign,
                                            ropd: 182,
                                        },
                                        HirEagerExprData::Variable(
                                            19,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 185,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 184,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 186,
                                        },
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 188,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 189,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 187,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 190,
                                        },
                                        HirEagerExprData::Variable(
                                            24,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 192,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 193,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 191,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 194,
                                        },
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 196,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 197,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 195,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 198,
                                        },
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 200,
                                            ident: `last`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            SelfPlace,
                                                        ),
                                                        HirTermSymbolResolution::SelfPlace(
                                                            MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        9,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 201,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::AssociatedFn {
                                            associated_item_path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::AssociatedFunctionFnCall {
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
                                            ),
                                            function_hir_eager_expr_idx: 203,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    204,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        10,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    205,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        11,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 202,
                                            opr: Assign,
                                            ropd: 206,
                                        },
                                        HirEagerExprData::Variable(
                                            23,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 209,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 208,
                                            opr: Assign,
                                            ropd: 210,
                                        },
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 213,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 212,
                                            opr: Assign,
                                            ropd: 214,
                                        },
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::AssociatedFn {
                                            associated_item_path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::AssociatedFunctionFnCall {
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
                                            ),
                                            function_hir_eager_expr_idx: 217,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    218,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        10,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 6,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    219,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        11,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 216,
                                            ident: `push`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            SelfLifetime,
                                                        ),
                                                        HirTermSymbolResolution::SelfLifetime,
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 44,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    220,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            23,
                                        ),
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 222,
                                            opr: Assign,
                                            ropd: 223,
                                        },
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Variable(
                                            24,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 225,
                                            opr: Assign,
                                            ropd: 226,
                                        },
                                        HirEagerExprData::Variable(
                                            24,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 228,
                                            opr: Assign,
                                            ropd: 229,
                                        },
                                        HirEagerExprData::Variable(
                                            20,
                                        ),
                                        HirEagerExprData::Variable(
                                            19,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 231,
                                            opr: Assign,
                                            ropd: 232,
                                        },
                                        HirEagerExprData::Variable(
                                            19,
                                        ),
                                        HirEagerExprData::Variable(
                                            26,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 234,
                                            opr: Assign,
                                            ropd: 235,
                                        },
                                        HirEagerExprData::Variable(
                                            25,
                                        ),
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 239,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 240,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 238,
                                            opr: Assign,
                                            ropd: 241,
                                        },
                                        HirEagerExprData::Variable(
                                            14,
                                        ),
                                        HirEagerExprData::Variable(
                                            13,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 243,
                                            opr: Assign,
                                            ropd: 244,
                                        },
                                        HirEagerExprData::Variable(
                                            13,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 247,
                                            ident: `mask`,
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 249,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 250,
                                        },
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 248,
                                            items: [
                                                251,
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 246,
                                            opr: Assign,
                                            ropd: 252,
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 255,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 256,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 254,
                                            opr: Assign,
                                            ropd: 257,
                                        },
                                        HirEagerExprData::Variable(
                                            13,
                                        ),
                                        HirEagerExprData::Variable(
                                            14,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 259,
                                            opr: Assign,
                                            ropd: 260,
                                        },
                                        HirEagerExprData::Variable(
                                            14,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 263,
                                            ident: `mask`,
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 264,
                                            items: [
                                                265,
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 262,
                                            opr: Assign,
                                            ropd: 266,
                                        },
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 269,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 270,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 268,
                                            opr: Assign,
                                            ropd: 271,
                                        },
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 274,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 275,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 273,
                                            opr: Assign,
                                            ropd: 276,
                                        },
                                        HirEagerExprData::Variable(
                                            15,
                                        ),
                                        HirEagerExprData::Variable(
                                            25,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 278,
                                            opr: Assign,
                                            ropd: 279,
                                        },
                                        HirEagerExprData::Variable(
                                            24,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 282,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 281,
                                            opr: Comparison(
                                                Neq,
                                            ),
                                            ropd: 283,
                                        },
                                        HirEagerExprData::Variable(
                                            24,
                                        ),
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 285,
                                            opr: Incr,
                                        },
                                        HirEagerExprData::Variable(
                                            19,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 288,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 287,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 289,
                                        },
                                        HirEagerExprData::Variable(
                                            24,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 291,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 292,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 290,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 293,
                                        },
                                        HirEagerExprData::Variable(
                                            22,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 295,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 296,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 294,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 297,
                                        },
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 299,
                                            ident: `pop`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            SelfLifetime,
                                                        ),
                                                        HirTermSymbolResolution::SelfLifetime,
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            function_hir_eager_expr_idx: 302,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 42,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    303,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: StackPure {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 73,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    304,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        9,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 301,
                                            ident: `push`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            SelfLifetime,
                                                        ),
                                                        HirTermSymbolResolution::SelfLifetime,
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 34,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    305,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                59..64,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 11,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 15,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 18,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 21,
                                        },
                                        Eval {
                                            expr_idx: 41,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 143,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 149,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 153,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 157,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 177,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 180,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 183,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 207,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 211,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 215,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 221,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 224,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 227,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 136,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    6..10,
                                                ),
                                            },
                                            elif_branches: [
                                                HirEagerElifBranch {
                                                    condition: Other {
                                                        hir_eager_expr_idx: 169,
                                                        conversion: None,
                                                    },
                                                    stmts: ArenaIdxRange(
                                                        10..13,
                                                    ),
                                                },
                                                HirEagerElifBranch {
                                                    condition: Other {
                                                        hir_eager_expr_idx: 199,
                                                        conversion: None,
                                                    },
                                                    stmts: ArenaIdxRange(
                                                        13..16,
                                                    ),
                                                },
                                            ],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        16..19,
                                                    ),
                                                },
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 230,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 233,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 236,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 242,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 245,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 253,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 258,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 261,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 267,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 272,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 277,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 286,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 22,
                                                ty: None,
                                            },
                                            initial_value: 97,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 23,
                                                ty: None,
                                            },
                                            initial_value: 101,
                                        },
                                        Eval {
                                            expr_idx: 113,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 114,
                                                    conversion: IntToBool(
                                                        I32,
                                                    ),
                                                },
                                                stmts: ArenaIdxRange(
                                                    19..23,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 24,
                                                    stmts: ArenaIdxRange(
                                                        23..26,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 25,
                                                    stmts: ArenaIdxRange(
                                                        26..29,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 26,
                                                    stmts: ArenaIdxRange(
                                                        29..30,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 27,
                                                    stmts: ArenaIdxRange(
                                                        30..31,
                                                    ),
                                                },
                                            ],
                                            match_target: 237,
                                        },
                                        Eval {
                                            expr_idx: 280,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 284,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    31..32,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 300,
                                            coersion: None,
                                            discarded: true,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 7,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 73,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            initial_value: 47,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 8,
                                                ty: None,
                                            },
                                            initial_value: 48,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 9,
                                                ty: None,
                                            },
                                            initial_value: 52,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 10,
                                                ty: None,
                                            },
                                            initial_value: 58,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 11,
                                                ty: None,
                                            },
                                            initial_value: 62,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 12,
                                                ty: None,
                                            },
                                            initial_value: 67,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 13,
                                                ty: None,
                                            },
                                            initial_value: 68,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 14,
                                                ty: None,
                                            },
                                            initial_value: 69,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 15,
                                                ty: None,
                                            },
                                            initial_value: 70,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 16,
                                                ty: None,
                                            },
                                            initial_value: 71,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 17,
                                                ty: None,
                                            },
                                            initial_value: 72,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 18,
                                                ty: None,
                                            },
                                            initial_value: 73,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 19,
                                                ty: None,
                                            },
                                            initial_value: 75,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 20,
                                                ty: None,
                                            },
                                            initial_value: 77,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 21,
                                                ty: None,
                                            },
                                            initial_value: 79,
                                        },
                                        DoWhile {
                                            condition: Other {
                                                hir_eager_expr_idx: 91,
                                                conversion: None,
                                            },
                                            block: ArenaIdxRange(
                                                32..39,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 298,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    39..40,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 306,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        While {
                                            condition: Other {
                                                hir_eager_expr_idx: 46,
                                                conversion: IntToBool(
                                                    R32,
                                                ),
                                            },
                                            stmts: ArenaIdxRange(
                                                40..58,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 41,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            initial_value: 1,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 3,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 251,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            4,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            5,
                                                        ),
                                                        kind: UpperClosed,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..6,
                                            ),
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 120,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            42,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            43,
                                                        ),
                                                        kind: UpperClosed,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                58..59,
                                            ),
                                        },
                                        Return {
                                            result: 307,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 17,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 325,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 327,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 328,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 329,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 330,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 331,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 251,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 263,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 309,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 310,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 318,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 332,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 333,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 334,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 335,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 336,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 337,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 338,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 339,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 340,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 341,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 342,
                                                    },
                                                ),
                                            ),
                                        },
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 273,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 275,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 274,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 276,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `cc`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `result`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `boundary_unsearched`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LoopVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r_ur`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r_dr`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r_ul`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r_dl`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `k`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LoopVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `contour`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `row_above`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `row_below`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `inward_direction`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `i0`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `j0`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dir0`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `prev_angle_change1`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `prev_angle_change2`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `total_angle_change`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `prev_streak1`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `prev_streak2`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `current_streak`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `outward_direction`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `angle_change`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `mnist_classifier::raw_contour`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    trai: HirTrait {
                        trai_path: TraitPath(`core::visual::Visualize`),
                        template_arguments: [],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist_classifier::raw_contour`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::MethodFn(
                TraitForTypeMethodFnHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist_classifier::raw_contour`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `visualize`,
                                        item_kind: MethodFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::visual::Html`, `Extern`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TraitForTypeItem(
                                                        TraitForTypeItemPathData {
                                                            impl_block: TraitForTypeImplBlock {
                                                                data: TraitForTypeImplBlockPathData {
                                                                    module_path: `mnist_classifier::raw_contour`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `visualize`,
                                                            item_kind: MethodFn,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TraitForTypeItem(
                                            TraitForTypeItemPath(
                                                ItemPathId {
                                                    data: ItemPathData::AssociatedItem(
                                                        AssociatedItemPathData::TraitForTypeItem(
                                                            TraitForTypeItemPathData {
                                                                impl_block: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `visualize`,
                                                                item_kind: MethodFn,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::EmptyHtmlTag {
                                            function_ident: `Contour`,
                                            arguments: [
                                                HirEagerHtmlArgumentExpr {
                                                    property_ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 253,
                                                            },
                                                        ),
                                                    ),
                                                    expr: 2,
                                                },
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::SelfValue,
                                                data: HirEagerRuntimeSymbolData::SelfValue,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::raw_contour`,
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TypeImplBlock(
                                    TypeImplBlockPath(
                                        ItemPathId {
                                            data: ItemPathData::ImplBlock(
                                                ImplBlockPathData::TypeImplBlock(
                                                    TypeImplBlockPathData {
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`, `MemoizedField`),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`, `MemoizedField`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`, `MemoizedField`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            5,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::AssociatedFn {
                                            associated_item_path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`, `AssociatedFunctionFn`),
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                TermF32Literal {
                                                    value: OrderedFloat(
                                                        1.4,
                                                    ),
                                                    text: "1.4f32",
                                                },
                                            ),
                                        ),
                                        HirEagerExprData::AssociatedFunctionFnCall {
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`, `AssociatedFunctionFn`),
                                            ),
                                            function_hir_eager_expr_idx: 1,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    2,
                                                    PlaceToLeash,
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 15,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    3,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 4,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::SelfValue,
                                                data: HirEagerRuntimeSymbolData::SelfValue,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            54,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::USize(
                                                TermUSizeLiteral {
                                                    value: 0,
                                                },
                                            ),
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 2,
                                            items: [
                                                3,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 5,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 7,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 9,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 11,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 13,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 14,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 16,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 17,
                                            items: [
                                                18,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 22,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 21,
                                            ident: `min`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::num::f32(0)::min`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 15,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    23,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 20,
                                            opr: Assign,
                                            ropd: 24,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 28,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 27,
                                            ident: `max`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::num::f32(0)::max`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 15,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    29,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 26,
                                            opr: Assign,
                                            ropd: 30,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 34,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 33,
                                            ident: `min`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::num::f32(0)::min`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 15,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    35,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 32,
                                            opr: Assign,
                                            ropd: 36,
                                        },
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 40,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 39,
                                            ident: `max`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::num::f32(0)::max`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 15,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    41,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 38,
                                            opr: Assign,
                                            ropd: 42,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                            function_hir_eager_expr_idx: 45,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 15,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    46,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        2,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 15,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    47,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        3,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                            function_hir_eager_expr_idx: 49,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 15,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    50,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        4,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 15,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    51,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        5,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            function_hir_eager_expr_idx: 44,
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: None,
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 58,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    48,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Move,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 58,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    52,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                6..13,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 19,
                                        },
                                        Eval {
                                            expr_idx: 25,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 31,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 37,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 43,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 4,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 6,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 8,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 10,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 12,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 251,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: None,
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
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
                                            block: ArenaIdxRange(
                                                1..6,
                                            ),
                                        },
                                        Return {
                                            result: 53,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 284,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 287,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 288,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 289,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::SelfValue,
                                                data: HirEagerRuntimeSymbolData::SelfValue,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `start_point`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `xmin`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `xmax`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ymin`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ymax`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LoopVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `point`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            10,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `cc`,
                                        },
                                        HirEagerExprData::MemoizedField {
                                            owner_hir_expr_idx: 2,
                                            ident: `raw_contours`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`, `MemoizedField`),
                                            ),
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::USize(
                                                TermUSizeLiteral {
                                                    value: 0,
                                                },
                                            ),
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 3,
                                            items: [
                                                4,
                                            ],
                                        },
                                        HirEagerExprData::MemoizedField {
                                            owner_hir_expr_idx: 5,
                                            ident: `bounding_box`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MemoizedField {
                                            owner_hir_expr_idx: 7,
                                            ident: `bounding_box`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                                            ),
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 6,
                                            ident: `relative_bounding_box`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 60,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    8,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 9,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::SelfValue,
                                                data: HirEagerRuntimeSymbolData::SelfValue,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::contour_len`, `MemoizedField`),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::contour_len`, `MemoizedField`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::contour_len`, `MemoizedField`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            59,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::contour_len`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                TermF32Literal {
                                                    value: OrderedFloat(
                                                        0.0,
                                                    ),
                                                    text: "0.0f32",
                                                },
                                            ),
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 3,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 4,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 6,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 8,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 9,
                                        },
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 7,
                                            items: [
                                                10,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 12,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 13,
                                            items: [
                                                14,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 17,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 19,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 18,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 20,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 21,
                                            ident: `abs`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 23,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 25,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 24,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 26,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 27,
                                            ident: `abs`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 22,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 28,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 16,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 29,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 31,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 33,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 34,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 35,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 36,
                                        },
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 32,
                                            items: [
                                                37,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 39,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::USize(
                                                TermUSizeLiteral {
                                                    value: 0,
                                                },
                                            ),
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 40,
                                            items: [
                                                41,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 44,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 46,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 45,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 47,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 48,
                                            ident: `abs`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 50,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 52,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 51,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 53,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 54,
                                            ident: `abs`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 49,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 55,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 43,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 56,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                4..10,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 11,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 15,
                                        },
                                        Eval {
                                            expr_idx: 30,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 1,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 251,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            2,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            5,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 38,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 42,
                                        },
                                        Eval {
                                            expr_idx: 57,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Return {
                                            result: 58,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 243,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 45,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 150,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 45,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 150,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::SelfValue,
                                                data: HirEagerRuntimeSymbolData::SelfValue,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `contour_len`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LoopVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `a`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `b`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `a`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `b`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 142,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 143,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `start`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `end`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            19,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirTemplateSymbol::Type(
                                                            Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                                separator: Some(
                                                    1,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 4,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 6,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            ropd: 7,
                                        },
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 5,
                                            items: [
                                                8,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 10,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 12,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            ropd: 13,
                                        },
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 11,
                                            items: [
                                                14,
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 16,
                                            ident: `to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [
                                                Regular(
                                                    HirRitchieRegularParameter {
                                                        contract: Pure,
                                                        ty: PathLeading(
                                                            HirTypePathLeading(
                                                                Id {
                                                                    value: 44,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    17,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        3,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..5,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 3,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 9,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 15,
                                        },
                                        Eval {
                                            expr_idx: 18,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 295,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 296,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 297,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::SelfValue,
                                                data: HirEagerRuntimeSymbolData::SelfValue,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `start`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `end`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `N`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ct_start`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ct_end`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]
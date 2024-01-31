[
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: SynNodeRegionPath::Defn(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`LineSegmentSketch`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `strokes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Vec LineSegmentStroke`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec LineSegmentStroke`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec LineSegmentStroke`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Vec LineSegmentStroke`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: StackPure {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [
                                                        (
                                                            SymbolEtherealTerm(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 40,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
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
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`LineSegmentSketch`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        4,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `strokes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            16,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Vec LineSegmentStroke`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec LineSegmentStroke`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec LineSegmentStroke`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `index`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `L`,
                                    regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        6,
                                    ),
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ropd: SemaExprIdx(
                                        7,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        5,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                8,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: FluffyIndexSignature::Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`LineSegmentStroke`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`LineSegmentStroke`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentStroke`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`LineSegmentStroke`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: StackPure {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`LineSegmentSketch`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        11,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `strokes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            31,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Vec LineSegmentStroke`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec LineSegmentStroke`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec LineSegmentStroke`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `index`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        36,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        13,
                                    ),
                                    opr: Closed(
                                        Sub,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    ropd: SemaExprIdx(
                                        14,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    item: SemaExprIdx(
                                        15,
                                    ),
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `L`,
                                    regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        16,
                                    ),
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    ropd: SemaExprIdx(
                                        17,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    18,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                18,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        40,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: FluffyIndexSignature::Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`LineSegmentStroke`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`LineSegmentStroke`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    19,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentStroke`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        19,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            42,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`LineSegmentStroke`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: StackPure {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        44,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    20,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `previous_displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    21,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `current_displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    22,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        21,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        49,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `rotation_direction_to`,
                                        regional_token_idx: RegionalTokenIdx(
                                            50,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Vector2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FluffyRitchieParameter::Regular(
                                                        FluffyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: FluffyTermBase::Ethereal(
                                                                    EtherealTerm(`Vector2d`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: ImmutableStackOwned {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    3,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vector2d`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    22,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    23,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `is_rotation_counterclockwise_result`,
                                    regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    24,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        57,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    25,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        24,
                                    ),
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    ropd: SemaExprIdx(
                                        25,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    26,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        64,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 33,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    27,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        63,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        27,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    28,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        68,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`LineSegmentSketch`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    29,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        29,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        69,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `strokes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            70,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Vec LineSegmentStroke`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec LineSegmentStroke`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    30,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec LineSegmentStroke`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        73,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `index`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    31,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        75,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    32,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        31,
                                    ),
                                    opr: Closed(
                                        Sub,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        74,
                                    ),
                                    ropd: SemaExprIdx(
                                        32,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    33,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    item: SemaExprIdx(
                                        33,
                                    ),
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    34,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `L`,
                                    regional_token_idx: RegionalTokenIdx(
                                        78,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    35,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        34,
                                    ),
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        77,
                                    ),
                                    ropd: SemaExprIdx(
                                        35,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    36,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        30,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        71,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                36,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        79,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: FluffyIndexSignature::Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`LineSegmentStroke`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`LineSegmentStroke`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    37,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentStroke`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        37,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentStroke`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            81,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Leash CyclicSlice Point2d`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash CyclicSlice Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    38,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `previous_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        83,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash CyclicSlice Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    39,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        39,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        84,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `start`,
                                        regional_token_idx: RegionalTokenIdx(
                                            85,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`CyclicSlice Point2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            SymbolEtherealTerm(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 30,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
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
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        86,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        87,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    40,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `previous_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        91,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash CyclicSlice Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    41,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        41,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        92,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end`,
                                        regional_token_idx: RegionalTokenIdx(
                                            93,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`CyclicSlice Point2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            SymbolEtherealTerm(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 30,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
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
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        94,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    42,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        89,
                                    ),
                                    ident: `i1`,
                                    frame_var_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        41,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    7,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    43,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        100,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`LineSegmentSketch`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    44,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        44,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `contour`,
                                        regional_token_idx: RegionalTokenIdx(
                                            102,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Leash RawContour`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    45,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `previous_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        106,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash CyclicSlice Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    46,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        46,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        107,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `start`,
                                        regional_token_idx: RegionalTokenIdx(
                                            108,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`CyclicSlice Point2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            SymbolEtherealTerm(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 30,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
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
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        110,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    47,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `i1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        112,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        41,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    7,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    48,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        45,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        103,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            104,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`RawContour`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FluffyRitchieParameter::Regular(
                                                        FluffyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: FluffyTermBase::Ethereal(
                                                                    EtherealTerm(`i32`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                    FluffyRitchieParameter::Regular(
                                                        FluffyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: FluffyTermBase::Ethereal(
                                                                    EtherealTerm(`i32`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        105,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    47,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        111,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    48,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            7,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        113,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    49,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `previous_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        114,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    5,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    50,
                                    FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `previous_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        116,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    5,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    51,
                                    FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `current_displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        120,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    52,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        124,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    8,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    53,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        8,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        52,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        121,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `cross`,
                                        regional_token_idx: RegionalTokenIdx(
                                            122,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Vector2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FluffyRitchieParameter::Regular(
                                                        FluffyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: FluffyTermBase::Ethereal(
                                                                    EtherealTerm(`Vector2d`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: ImmutableStackOwned {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    2,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        123,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vector2d`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    53,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            8,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        125,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    54,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        51,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        117,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `max`,
                                        regional_token_idx: RegionalTokenIdx(
                                            118,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::max`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`f32`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FluffyRitchieParameter::Regular(
                                                        FluffyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: FluffyTermBase::Ethereal(
                                                                    EtherealTerm(`f32`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: MutableStackOwned {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    5,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        119,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`f32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    54,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        126,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    55,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        50,
                                    ),
                                    opr: Assign,
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        115,
                                    ),
                                    ropd: SemaExprIdx(
                                        55,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    56,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        132,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 34,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    57,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        131,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        57,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    58,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        136,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`LineSegmentSketch`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    59,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        59,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        137,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `strokes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            138,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Vec LineSegmentStroke`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec LineSegmentStroke`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    60,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec LineSegmentStroke`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        140,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `index`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    61,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `L`,
                                    regional_token_idx: RegionalTokenIdx(
                                        142,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    62,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        61,
                                    ),
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        141,
                                    ),
                                    ropd: SemaExprIdx(
                                        62,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    63,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        60,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        139,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                63,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        143,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: FluffyIndexSignature::Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`LineSegmentStroke`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`LineSegmentStroke`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    64,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentStroke`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        64,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentStroke`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        144,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            145,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Leash CyclicSlice Point2d`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash CyclicSlice Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    65,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `current_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        147,
                                    ),
                                    current_syn_symbol_idx: 10,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 9,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    10,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash CyclicSlice Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    66,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        66,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        148,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `start`,
                                        regional_token_idx: RegionalTokenIdx(
                                            149,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`CyclicSlice Point2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            SymbolEtherealTerm(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 30,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
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
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        150,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        151,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    67,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `current_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        155,
                                    ),
                                    current_syn_symbol_idx: 10,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 9,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    10,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash CyclicSlice Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    68,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        68,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        156,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end`,
                                        regional_token_idx: RegionalTokenIdx(
                                            157,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`CyclicSlice Point2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            SymbolEtherealTerm(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 30,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
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
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        158,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        159,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    69,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        153,
                                    ),
                                    ident: `i2`,
                                    frame_var_symbol_idx: 11,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        70,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    11,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    70,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        164,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`LineSegmentSketch`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    71,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        71,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        165,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `contour`,
                                        regional_token_idx: RegionalTokenIdx(
                                            166,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Leash RawContour`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    72,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `previous_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        170,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash CyclicSlice Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    73,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        73,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        171,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `start`,
                                        regional_token_idx: RegionalTokenIdx(
                                            172,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`CyclicSlice Point2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            SymbolEtherealTerm(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 30,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
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
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        173,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        174,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    74,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `i2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        176,
                                    ),
                                    current_syn_symbol_idx: 11,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        70,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    11,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    75,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        72,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        167,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            168,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`RawContour`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FluffyRitchieParameter::Regular(
                                                        FluffyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: FluffyTermBase::Ethereal(
                                                                    EtherealTerm(`i32`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                    FluffyRitchieParameter::Regular(
                                                        FluffyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: FluffyTermBase::Ethereal(
                                                                    EtherealTerm(`i32`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        169,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    74,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        175,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    75,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            11,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        177,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    76,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `current_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        178,
                                    ),
                                    current_syn_symbol_idx: 9,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    9,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    77,
                                    FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `current_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        180,
                                    ),
                                    current_syn_symbol_idx: 9,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    9,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    78,
                                    FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `current_displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        184,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    79,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        188,
                                    ),
                                    current_syn_symbol_idx: 12,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 10,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    12,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    80,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        12,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        79,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        185,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `cross`,
                                        regional_token_idx: RegionalTokenIdx(
                                            186,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Vector2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FluffyRitchieParameter::Regular(
                                                        FluffyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: FluffyTermBase::Ethereal(
                                                                    EtherealTerm(`Vector2d`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: ImmutableStackOwned {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    2,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        187,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vector2d`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    80,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            12,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        189,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    81,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        78,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        181,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `max`,
                                        regional_token_idx: RegionalTokenIdx(
                                            182,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::max`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`f32`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FluffyRitchieParameter::Regular(
                                                        FluffyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: FluffyTermBase::Ethereal(
                                                                    EtherealTerm(`f32`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: MutableStackOwned {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    9,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        183,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`f32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    81,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        190,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    82,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        77,
                                    ),
                                    opr: Assign,
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        179,
                                    ),
                                    ropd: SemaExprIdx(
                                        82,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    83,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `current_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        192,
                                    ),
                                    current_syn_symbol_idx: 9,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    9,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    84,
                                    FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `previous_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        194,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    5,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    85,
                                    FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        84,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        193,
                                    ),
                                    ropd: SemaExprIdx(
                                        85,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    86,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `is_rotation_counterclockwise_result`,
                                    regional_token_idx: RegionalTokenIdx(
                                        198,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    87,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        200,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    88,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        87,
                                    ),
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        199,
                                    ),
                                    ropd: SemaExprIdx(
                                        88,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    89,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            13..18,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`never`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    90,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`never`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            97,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 7,
                                        },
                                        variables: ArenaIdxRange(
                                            8..9,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            99,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        49,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        56,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            161,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 10,
                                        },
                                        variables: ArenaIdxRange(
                                            12..13,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            163,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        76,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        83,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            59,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 5,
                                        },
                                        variables: ArenaIdxRange(
                                            5..6,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            62,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        28,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            65,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 6,
                                        },
                                        variables: ArenaIdxRange(
                                            6..7,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            67,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        38,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            82,
                                        ),
                                    },
                                    particulars: SemaForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            89,
                                        ),
                                        for_between_loop_var_ident: `i1`,
                                        for_between_loop_var_expr_idx: SemaExprIdx(
                                            43,
                                        ),
                                        range: SemaForBetweenRange {
                                            initial_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    SemaExprIdx(
                                                        40,
                                                    ),
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    SemaExprIdx(
                                                        42,
                                                    ),
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    for_loop_var_symbol_idx: 7,
                                    eol_colon: EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                96,
                                            ),
                                        },
                                    ),
                                    block: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..3,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            127,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 8,
                                        },
                                        variables: ArenaIdxRange(
                                            9..10,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            130,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        58,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            133,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 9,
                                        },
                                        variables: ArenaIdxRange(
                                            10..11,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            135,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        65,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            146,
                                        ),
                                    },
                                    particulars: SemaForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            153,
                                        ),
                                        for_between_loop_var_ident: `i2`,
                                        for_between_loop_var_expr_idx: SemaExprIdx(
                                            70,
                                        ),
                                        range: SemaForBetweenRange {
                                            initial_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    SemaExprIdx(
                                                        67,
                                                    ),
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    SemaExprIdx(
                                                        69,
                                                    ),
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    for_loop_var_symbol_idx: 11,
                                    eol_colon: EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                160,
                                            ),
                                        },
                                    ),
                                    block: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            3..5,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            191,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        86,
                                    ),
                                    coersion_outcome: Some(
                                        ExpectCoersionOutcome {
                                            coersion: Trivial(
                                                TrivialFluffyCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`never`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            197,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        89,
                                    ),
                                    coersion_outcome: Some(
                                        ExpectCoersionOutcome {
                                            coersion: Trivial(
                                                TrivialFluffyCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`never`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            11,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 2,
                                        },
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            13,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            26,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 3,
                                        },
                                        variables: ArenaIdxRange(
                                            3..4,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            28,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        20,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            45,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 4,
                                        },
                                        variables: ArenaIdxRange(
                                            4..5,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            47,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        23,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::IfElse {
                                    if_branch: SemaIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                54,
                                            ),
                                        },
                                        condition: Other {
                                            sema_expr_idx: SemaExprIdx(
                                                26,
                                            ),
                                            conversion: None,
                                        },
                                        eol_colon_token: EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                58,
                                            ),
                                        },
                                        stmts: SemaStmtIdxRange(
                                            ArenaIdxRange(
                                                5..12,
                                            ),
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: Some(
                                        SemaElseBranch {
                                            else_regional_token: ElseRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    195,
                                                ),
                                            },
                                            eol_colon_regional_token: EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    196,
                                                ),
                                            },
                                            stmts: SemaStmtIdxRange(
                                                ArenaIdxRange(
                                                    12..13,
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`never`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    94,
                    (
                        SemaExprIdx(
                            90,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 123,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 123,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: Some(
                                Const,
                            ),
                            base: Hollow(
                                HollowTerm(
                                    0,
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: Some(
                                StackPure {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: Ethereal(
                                Application(
                                    ApplicationEtherealTerm(
                                        Id {
                                            value: 56,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: Some(
                                Const,
                            ),
                            base: Hollow(
                                HollowTerm(
                                    1,
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: Some(
                                StackPure {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: Ethereal(
                                Application(
                                    ApplicationEtherealTerm(
                                        Id {
                                            value: 56,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 123,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 123,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            ApplicationEtherealTerm(
                                                Id {
                                                    value: 56,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            ApplicationEtherealTerm(
                                                Id {
                                                    value: 56,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        14,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        25,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        27,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
                                            Id {
                                                value: 33,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        32,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        57,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
                                            Id {
                                                value: 33,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        88,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                StackPure {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`LineSegmentSketch`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                StackPure {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`i32`),
                            ),
                        },
                    ),
                ],
                current_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`i32`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Vector2d`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Vector2d`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`i32`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                MutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Hollow(
                                HollowTerm(
                                    0,
                                ),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Leash CyclicSlice Point2d`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`i32`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            8,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Vector2d`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                MutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Hollow(
                                HollowTerm(
                                    1,
                                ),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            10,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Leash CyclicSlice Point2d`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            11,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`i32`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Vector2d`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: [],
                    },
                    hollow_terms: HollowTerms {
                        entries: [
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        27,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        59,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 2,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec LineSegmentStroke`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec LineSegmentStroke`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                StackPure {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentStroke`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec LineSegmentStroke`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                StackPure {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentStroke`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 21,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Vector2d`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 22,
                                    src: ExpectationSource {
                                        syn_expr_idx: 22,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        2,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 23,
                                    src: ExpectationSource {
                                        syn_expr_idx: 23,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 24,
                                    src: ExpectationSource {
                                        syn_expr_idx: 24,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 25,
                                    src: ExpectationSource {
                                        syn_expr_idx: 25,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 26,
                                    src: ExpectationSource {
                                        syn_expr_idx: 26,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 27,
                                    src: ExpectationSource {
                                        syn_expr_idx: 27,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 28,
                                    src: ExpectationSource {
                                        syn_expr_idx: 28,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 29,
                                    src: ExpectationSource {
                                        syn_expr_idx: 29,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 30,
                                    src: ExpectationSource {
                                        syn_expr_idx: 30,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec LineSegmentStroke`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 31,
                                    src: ExpectationSource {
                                        syn_expr_idx: 31,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                StackPure {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 32,
                                    src: ExpectationSource {
                                        syn_expr_idx: 32,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 33,
                                    src: ExpectationSource {
                                        syn_expr_idx: 33,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 34,
                                    src: ExpectationSource {
                                        syn_expr_idx: 34,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 35,
                                    src: ExpectationSource {
                                        syn_expr_idx: 35,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 36,
                                    src: ExpectationSource {
                                        syn_expr_idx: 36,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 37,
                                    src: ExpectationSource {
                                        syn_expr_idx: 37,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentStroke`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 38,
                                    src: ExpectationSource {
                                        syn_expr_idx: 38,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 39,
                                    src: ExpectationSource {
                                        syn_expr_idx: 39,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::IntType(
                                    ExpectIntType,
                                ),
                                state: ExpectationState {
                                    idx: 40,
                                    src: ExpectationSource {
                                        syn_expr_idx: 40,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::IntType(
                                                ExpectIntTypeOutcome {
                                                    placeless_num_ty: FluffyTerm {
                                                        place: None,
                                                        base: Ethereal(
                                                            EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 123,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 41,
                                    src: ExpectationSource {
                                        syn_expr_idx: 42,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 42,
                                    src: ExpectationSource {
                                        syn_expr_idx: 44,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            7,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 43,
                                    src: ExpectationSource {
                                        syn_expr_idx: 41,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        7,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 44,
                                    src: ExpectationSource {
                                        syn_expr_idx: 46,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 45,
                                    src: ExpectationSource {
                                        syn_expr_idx: 47,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash RawContour`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 46,
                                    src: ExpectationSource {
                                        syn_expr_idx: 48,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 47,
                                    src: ExpectationSource {
                                        syn_expr_idx: 49,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 48,
                                    src: ExpectationSource {
                                        syn_expr_idx: 50,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        7,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 49,
                                    src: ExpectationSource {
                                        syn_expr_idx: 51,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 50,
                                    src: ExpectationSource {
                                        syn_expr_idx: 56,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 51,
                                    src: ExpectationSource {
                                        syn_expr_idx: 52,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 52,
                                    src: ExpectationSource {
                                        syn_expr_idx: 53,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Vector2d`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 53,
                                    src: ExpectationSource {
                                        syn_expr_idx: 54,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        8,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        8,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 54,
                                    src: ExpectationSource {
                                        syn_expr_idx: 55,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyDerived(
                                    ExpectAnyDerived,
                                ),
                                state: ExpectationState {
                                    idx: 55,
                                    src: ExpectationSource {
                                        syn_expr_idx: 57,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 56,
                                    src: ExpectationSource {
                                        syn_expr_idx: 58,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 57,
                                    src: ExpectationSource {
                                        syn_expr_idx: 59,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 58,
                                    src: ExpectationSource {
                                        syn_expr_idx: 60,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 59,
                                    src: ExpectationSource {
                                        syn_expr_idx: 61,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 60,
                                    src: ExpectationSource {
                                        syn_expr_idx: 62,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec LineSegmentStroke`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 61,
                                    src: ExpectationSource {
                                        syn_expr_idx: 63,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                StackPure {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 62,
                                    src: ExpectationSource {
                                        syn_expr_idx: 64,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 63,
                                    src: ExpectationSource {
                                        syn_expr_idx: 65,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 64,
                                    src: ExpectationSource {
                                        syn_expr_idx: 66,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentStroke`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 65,
                                    src: ExpectationSource {
                                        syn_expr_idx: 67,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 66,
                                    src: ExpectationSource {
                                        syn_expr_idx: 68,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::IntType(
                                    ExpectIntType,
                                ),
                                state: ExpectationState {
                                    idx: 67,
                                    src: ExpectationSource {
                                        syn_expr_idx: 69,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::IntType(
                                                ExpectIntTypeOutcome {
                                                    placeless_num_ty: FluffyTerm {
                                                        place: None,
                                                        base: Ethereal(
                                                            EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 123,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 68,
                                    src: ExpectationSource {
                                        syn_expr_idx: 71,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 69,
                                    src: ExpectationSource {
                                        syn_expr_idx: 73,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            11,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 70,
                                    src: ExpectationSource {
                                        syn_expr_idx: 70,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        11,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 71,
                                    src: ExpectationSource {
                                        syn_expr_idx: 75,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`LineSegmentSketch`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 72,
                                    src: ExpectationSource {
                                        syn_expr_idx: 76,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash RawContour`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 73,
                                    src: ExpectationSource {
                                        syn_expr_idx: 77,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 74,
                                    src: ExpectationSource {
                                        syn_expr_idx: 78,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 75,
                                    src: ExpectationSource {
                                        syn_expr_idx: 79,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        11,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 76,
                                    src: ExpectationSource {
                                        syn_expr_idx: 80,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 77,
                                    src: ExpectationSource {
                                        syn_expr_idx: 85,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 78,
                                    src: ExpectationSource {
                                        syn_expr_idx: 81,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 79,
                                    src: ExpectationSource {
                                        syn_expr_idx: 82,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Vector2d`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 80,
                                    src: ExpectationSource {
                                        syn_expr_idx: 83,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        12,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        12,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 81,
                                    src: ExpectationSource {
                                        syn_expr_idx: 84,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyDerived(
                                    ExpectAnyDerived,
                                ),
                                state: ExpectationState {
                                    idx: 82,
                                    src: ExpectationSource {
                                        syn_expr_idx: 86,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 83,
                                    src: ExpectationSource {
                                        syn_expr_idx: 87,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 84,
                                    src: ExpectationSource {
                                        syn_expr_idx: 88,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                MutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            9,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 85,
                                    src: ExpectationSource {
                                        syn_expr_idx: 89,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        5,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`bool`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 86,
                                    src: ExpectationSource {
                                        syn_expr_idx: 90,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 87,
                                    src: ExpectationSource {
                                        syn_expr_idx: 91,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 88,
                                    src: ExpectationSource {
                                        syn_expr_idx: 92,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`bool`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 89,
                                    src: ExpectationSource {
                                        syn_expr_idx: 93,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`bool`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 90,
                                    src: ExpectationSource {
                                        syn_expr_idx: 94,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`never`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Never,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                EtherealTerm(`bool`),
            ),
            self_ty: None,
        },
    },
]
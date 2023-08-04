[
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [
            PatternExprTypeInfo {
                ty: Ok(
                    Application(
                        EtherealTermApplication(
                            Id {
                                value: 47,
                            },
                        ),
                    ),
                ),
            },
        ],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
                None,
                None,
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::List(
                            ListExprDisambiguation::ListFunctor,
                        ),
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Explicit,
                                    variance: Covariant,
                                    parameter_variable: None,
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`Type`),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm::Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::List(
                            ListExprDisambiguation::NewList,
                        ),
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`List`),
                                    argument: EtherealTerm(`ConcaveComponent`),
                                    shift: 0,
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Place(
                                        StackPure {
                                            location: StackLocationIdx(
                                                LocalSymbolIdx(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ),
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 92,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 100,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`List`),
                                    argument: EtherealTerm(`LineSegmentStroke`),
                                    shift: 0,
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    5,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FluffyTermError(
                            FluffyTermError::Todo,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    6,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    1,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    7,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::PrefixOperandTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Ritchie(
                                EtherealTermRitchie {
                                    ritchie_kind: FnType,
                                    parameter_contracted_tys: [
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: None,
                                                ty: EtherealTerm(`LineSegmentSketch`),
                                            },
                                        ),
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: None,
                                                ty: EtherealTerm(`i32`),
                                            },
                                        ),
                                    ],
                                    return_ty: EtherealTerm(`bool`),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    9,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    10,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    11,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    8,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    12,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    13,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SuffixOperandTypeNotInferred,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    14,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Ritchie(
                                EtherealTermRitchie {
                                    ritchie_kind: FnType,
                                    parameter_contracted_tys: [
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: None,
                                                ty: EtherealTerm(`LineSegmentSketch`),
                                            },
                                        ),
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: None,
                                                ty: EtherealTerm(`i32`),
                                            },
                                        ),
                                    ],
                                    return_ty: EtherealTerm(`bool`),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    16,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    17,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    18,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    15,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    19,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    20,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SuffixOperandTypeNotInferred,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    2,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    21,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    22,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    1,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    23,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Ritchie(
                                EtherealTermRitchie {
                                    ritchie_kind: FnType,
                                    parameter_contracted_tys: [
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: Move,
                                                ty: EtherealTerm(`Leash LineSegmentSketch`),
                                            },
                                        ),
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: Move,
                                                ty: EtherealTerm(`CyclicSliceLeashed LineSegmentStroke`),
                                            },
                                        ),
                                    ],
                                    return_ty: EtherealTerm(`ConcaveComponent`),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    24,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    25,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    26,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Place(
                                        StackPure {
                                            location: StackLocationIdx(
                                                LocalSymbolIdx(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ),
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 92,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 100,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`List`),
                                    argument: EtherealTerm(`LineSegmentStroke`),
                                    shift: 0,
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    27,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Original(
                        OriginalExprTypeError::NoMethodForType {
                            self_expr_ty: FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`List`),
                                    argument: EtherealTerm(`LineSegmentStroke`),
                                    shift: 0,
                                },
                            ),
                            ident_token: IdentToken {
                                ident: `cyclic_slice`,
                                token_idx: TokenIdx(
                                    643,
                                ),
                            },
                        },
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    28,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Place(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                LocalSymbolIdx(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [
                                            Regular(
                                                FluffyTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 93,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ],
                                        return_ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    29,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    30,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    3,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    31,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    32,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    1,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    33,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::never`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    34,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [
            Ok(
                FluffyTerm::EntityPath(
                    TermEntityPath::TypeOntology(
                        TypePath(`core::list::List`, `Extern`),
                    ),
                ),
            ),
            Ok(
                FluffyTerm::EntityPath(
                    TermEntityPath::TypeOntology(
                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    ),
                ),
            ),
            Ok(
                FluffyTerm::Application(
                    EtherealTermApplication {
                        function: EtherealTerm(`List`),
                        argument: EtherealTerm(`ConcaveComponent`),
                        shift: 0,
                    },
                ),
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ],
            },
            current_symbol_map: ArenaMap {
                data: [
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    1,
                                ),
                            ),
                        ),
                    ),
                    None,
                    None,
                    None,
                    None,
                ],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::mem::Leash`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Indirection(
                                    Leash,
                                ),
                            ),
                            arguments: [
                                FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Leash LineSegmentSketch`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        0,
                                    ),
                                ),
                            },
                        },
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::list::List`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::List,
                            ),
                            arguments: [
                                FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`List ConcaveComponent`),
                            ),
                            place: MutableStackOwned {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        1,
                                    ),
                                ),
                            },
                        },
                    ],
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    7,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    8,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    41,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    59,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Covariant,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::Category(
                                                    TermCategory {
                                                        universe: TermUniverse(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Covariant,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm::Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    return_ty: FluffyTerm::Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`List`),
                                            argument: EtherealTerm(`ConcaveComponent`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`List`),
                                        argument: EtherealTerm(`ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`List`),
                                        argument: EtherealTerm(`LineSegmentStroke`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 16,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EtherealTerm(`LineSegmentSketch`),
                                                },
                                            ),
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EtherealTerm(`i32`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`bool`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 92,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 44,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 17,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 18,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 26,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 35,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 31,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EtherealTerm(`LineSegmentSketch`),
                                                },
                                            ),
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EtherealTerm(`i32`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`bool`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 92,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 44,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 32,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 34,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 19,
                                src: ExpectationSource {
                                    expr_idx: 36,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 37,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 41,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        2,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 22,
                                src: ExpectationSource {
                                    expr_idx: 44,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 45,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        1,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 24,
                                src: ExpectationSource {
                                    expr_idx: 46,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: Move,
                                                    ty: EtherealTerm(`Leash LineSegmentSketch`),
                                                },
                                            ),
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: Move,
                                                    ty: EtherealTerm(`CyclicSliceLeashed LineSegmentStroke`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`ConcaveComponent`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: Application(
                                                                    EtherealTermApplication(
                                                                        Id {
                                                                            value: 73,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: Application(
                                                                    EtherealTermApplication(
                                                                        Id {
                                                                            value: 74,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`Leash`),
                                            argument: EtherealTerm(`LineSegmentSketch`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 47,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 26,
                                src: ExpectationSource {
                                    expr_idx: 48,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 27,
                                src: ExpectationSource {
                                    expr_idx: 49,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`List`),
                                        argument: EtherealTerm(`LineSegmentStroke`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 28,
                                src: ExpectationSource {
                                    expr_idx: 53,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 29,
                                src: ExpectationSource {
                                    expr_idx: 54,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 30,
                                src: ExpectationSource {
                                    expr_idx: 57,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 31,
                                src: ExpectationSource {
                                    expr_idx: 59,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        3,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 62,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`List`),
                                            argument: EtherealTerm(`ConcaveComponent`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 63,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        1,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`List`),
                                            argument: EtherealTerm(`ConcaveComponent`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 34,
                                src: ExpectationSource {
                                    expr_idx: 64,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::never`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Never,
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
            EtherealTerm(`List ConcaveComponent`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    trai_path: TraitPath(`core::visual::Visualize`),
                                    ty_sketch: Path(
                                        TypePath(
                                            Id {
                                                value: 93,
                                            },
                                        ),
                                    ),
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BlockTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`Html`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BlockTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BlockTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::UnableToInferSuffixOperandType,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::never`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    0,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 30,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 33,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 35,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::never`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Never,
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
            EtherealTerm(`f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::never`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                    None,
                    None,
                    None,
                ],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                    None,
                    None,
                    None,
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    0,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 24,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 27,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 30,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 32,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::never`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Never,
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
            EtherealTerm(`f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::UnableToInferSuffixOperandType,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Ritchie(
                                EtherealTermRitchie {
                                    ritchie_kind: FnType,
                                    parameter_contracted_tys: [
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: Move,
                                                ty: EtherealTerm(`ClosedRange`),
                                            },
                                        ),
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: Move,
                                                ty: EtherealTerm(`ClosedRange`),
                                            },
                                        ),
                                    ],
                                    return_ty: EtherealTerm(`BoundingBox`),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Ritchie(
                                EtherealTermRitchie {
                                    ritchie_kind: FnType,
                                    parameter_contracted_tys: [
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: Move,
                                                ty: EtherealTerm(`f32`),
                                            },
                                        ),
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: Move,
                                                ty: EtherealTerm(`f32`),
                                            },
                                        ),
                                    ],
                                    return_ty: EtherealTerm(`ClosedRange`),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    5,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    6,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Ritchie(
                                EtherealTermRitchie {
                                    ritchie_kind: FnType,
                                    parameter_contracted_tys: [
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: Move,
                                                ty: EtherealTerm(`f32`),
                                            },
                                        ),
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: Move,
                                                ty: EtherealTerm(`f32`),
                                            },
                                        ),
                                    ],
                                    return_ty: EtherealTerm(`ClosedRange`),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    7,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    8,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    9,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::never`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    10,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 32,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 38,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 44,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 50,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 51,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: Move,
                                                    ty: EtherealTerm(`ClosedRange`),
                                                },
                                            ),
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: Move,
                                                    ty: EtherealTerm(`ClosedRange`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`BoundingBox`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 88,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 88,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 52,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: Move,
                                                    ty: EtherealTerm(`f32`),
                                                },
                                            ),
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: Move,
                                                    ty: EtherealTerm(`f32`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`ClosedRange`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 60,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 60,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 55,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 56,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: Move,
                                                    ty: EtherealTerm(`f32`),
                                                },
                                            ),
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: Move,
                                                    ty: EtherealTerm(`f32`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`ClosedRange`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 60,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 60,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 59,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 60,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 61,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::never`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Never,
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
            EtherealTerm(`BoundingBox`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BlockTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`RelativeBoundingBox`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Ritchie(
                                EtherealTermRitchie {
                                    ritchie_kind: FnType,
                                    parameter_contracted_tys: [
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: Move,
                                                ty: EtherealTerm(`Point2d`),
                                            },
                                        ),
                                        EtherealTermRitchieParameter::Regular(
                                            EtherealTermRitchieRegularParameter {
                                                contract: Move,
                                                ty: EtherealTerm(`Point2d`),
                                            },
                                        ),
                                    ],
                                    return_ty: EtherealTerm(`LineSegment`),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::UnableToInferSuffixOperandType,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::UnableToInferSuffixOperandType,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: Move,
                                                    ty: EtherealTerm(`Point2d`),
                                                },
                                            ),
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: Move,
                                                    ty: EtherealTerm(`Point2d`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`LineSegment`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 85,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 85,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
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
            EtherealTerm(`LineSegment`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::UnableToInferSuffixOperandType,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BlockTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`Point2d`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::UnableToInferSuffixOperandType,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BlockTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`Point2d`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BlockTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`Vector2d`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::UnableToInferSuffixOperandType,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BlockTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`Vector2d`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::SelfTypeNotInferredForSelfValue,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::UnableToInferSuffixOperandType,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BlockTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`Vector2d`),
        ),
        self_ty: None,
    },
]
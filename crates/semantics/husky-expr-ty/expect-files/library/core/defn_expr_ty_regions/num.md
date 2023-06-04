[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::i8`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::i8)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::i8`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 14,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 14,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 14,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::i8)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::i16`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::i16)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::i16`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 16,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 16,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 16,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::i16)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::i32`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::i32)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::i32`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 16,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 16,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 16,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::i32)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::i64`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::i64)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::i64`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 17,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 17,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 17,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::i64)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::i128`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::i128)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::i128`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 18,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::i128)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::isize`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::isize)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::isize`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 19,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 19,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 19,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::isize)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::u8`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::u8)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::u8`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 20,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 20,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 20,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::u8)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::u16`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::u16)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::u16`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 21,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 21,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 21,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::u16)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::u32`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::u32)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::u32`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 22,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 22,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 22,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::u32)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::u64`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::u64)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::u64`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 23,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 23,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::u64)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::u128`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::u128)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::u128`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 24,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 24,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 24,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::u128)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::usize`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::usize)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::usize`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 25,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 25,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::usize)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::f32)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `sqrt`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::f32)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 26,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 26,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 26,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::f32)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::num`,
                            ty_path: TypePath(`core::num::f64`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `abs`,
                },
            ),
        ),
        expr_ty_infos: [],
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
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
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
            EtherealTerm(`TypeOntology(core::num::f64)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `core::num`,
                            trai_path: TraitPath(`core::ops::Add`),
                            ty_path: TypePath(`core::num::f64`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 27,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
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
            EtherealTerm(`TypeOntology(core::num::f64)`),
        ),
        self_ty: None,
    },
]
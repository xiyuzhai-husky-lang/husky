[
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntitySynNodePath::ModuleItem(
                ModuleItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`malamute::narrow_down`, `Gn`),
                            disambiguator: 0,
                        },
                    },
                ),
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
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    1,
                                ),
                            ),
                        ),
                    ),
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    2,
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
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 6,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 7,
                                },
                            ),
                        ),
                    ),
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [
                        SolidTermData::SymbolAtPlace {
                            term: EtherealTermSymbol {
                                ty: EtherealTerm(`Type`),
                                idx: DeclarativeTermSymbolIndex(
                                    Type {
                                        attrs: DeclarativeTemplateSymbolAttrs {
                                            phantom: false,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                            },
                            place: Const,
                        },
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::num::f32`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Num(
                                    Float(
                                        F32,
                                    ),
                                ),
                            ),
                            arguments: [],
                            base_ty_term: Some(
                                EtherealTerm(`f32`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        2,
                                    ),
                                ),
                            },
                        },
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::num::i32`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Num(
                                    Int(
                                        I32,
                                    ),
                                ),
                            ),
                            arguments: [],
                            base_ty_term: Some(
                                EtherealTerm(`i32`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        3,
                                    ),
                                ),
                            },
                        },
                    ],
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
            EtherealTerm(`OneVsAllResult t a`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntitySynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `malamute`,
                                    trai_path: TraitPath(`core::ops::Unveil`),
                                    ty_sketch: Path(
                                        TypePath(
                                            Id {
                                                value: 104,
                                            },
                                        ),
                                    ),
                                    disambiguator: 0,
                                },
                                ident: `Output`,
                                item_kind: AssociatedType,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
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
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
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
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 6,
                                },
                            ),
                        ),
                    ),
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [
                        SolidTermData::SymbolAtPlace {
                            term: EtherealTermSymbol {
                                ty: EtherealTerm(`Type`),
                                idx: DeclarativeTermSymbolIndex(
                                    Type {
                                        attrs: DeclarativeTemplateSymbolAttrs {
                                            phantom: false,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                            },
                            place: Const,
                        },
                    ],
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
        return_ty: None,
        self_ty: None,
    },
]
```rust
[
    (
        ItemPath(`core::result::Result`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Enum(
                        EnumDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 10,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 11,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                            ),
                                            argument: DecTerm::SymbolicVariable(
                                                DecSymbolicVariable {
                                                    toolchain: Toolchain {
                                                        data: ToolchainData::Local {
                                                            library_path: "../../../library",
                                                        },
                                                    },
                                                    ty: Ok(
                                                        Category(
                                                            Sort {
                                                                universe: Universe(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    index: DecSymbolicVariableIndex(
                                                        Type {
                                                            attrs: DeclarativeTemplateVariableAttrs {
                                                                class: Mono,
                                                            },
                                                            variance: None,
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    argument: DecTerm::SymbolicVariable(
                                        DecSymbolicVariable {
                                            toolchain: Toolchain {
                                                data: ToolchainData::Local {
                                                    library_path: "../../../library",
                                                },
                                            },
                                            ty: Ok(
                                                Category(
                                                    Sort {
                                                        universe: Universe(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                Type {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 1,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::result::Result as core::ops::Unveil(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 11,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 12,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::Application(
                                            DecApplication {
                                                function: DecTerm::EntityPath(
                                                    DecItemPath::Trait(
                                                        TraitPath(`core::ops::Unveil`),
                                                    ),
                                                ),
                                                argument: DecTerm::EntityPath(
                                                    DecItemPath::Type(
                                                        TypePath(`core::result::Result`, `Enum`),
                                                    ),
                                                ),
                                            },
                                        ),
                                        argument: DecTerm::SymbolicVariable(
                                            DecSymbolicVariable {
                                                toolchain: Toolchain {
                                                    data: ToolchainData::Local {
                                                        library_path: "../../../library",
                                                    },
                                                },
                                                ty: Ok(
                                                    Category(
                                                        Sort {
                                                            universe: Universe(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                index: DecSymbolicVariableIndex(
                                                    Type {
                                                        attrs: DeclarativeTemplateVariableAttrs {
                                                            class: Mono,
                                                        },
                                                        variance: None,
                                                        disambiguator: 1,
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                ),
                                argument: DecTerm::SymbolicVariable(
                                    DecSymbolicVariable {
                                        toolchain: Toolchain {
                                            data: ToolchainData::Local {
                                                library_path: "../../../library",
                                            },
                                        },
                                        ty: Ok(
                                            Category(
                                                Sort {
                                                    universe: Universe(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                        index: DecSymbolicVariableIndex(
                                            Type {
                                                attrs: DeclarativeTemplateVariableAttrs {
                                                    class: Mono,
                                                },
                                                variance: None,
                                                disambiguator: 3,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                            ),
                                            argument: DecTerm::SymbolicVariable(
                                                DecSymbolicVariable {
                                                    toolchain: Toolchain {
                                                        data: ToolchainData::Local {
                                                            library_path: "../../../library",
                                                        },
                                                    },
                                                    ty: Ok(
                                                        Category(
                                                            Sort {
                                                                universe: Universe(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    index: DecSymbolicVariableIndex(
                                                        Type {
                                                            attrs: DeclarativeTemplateVariableAttrs {
                                                                class: Mono,
                                                            },
                                                            variance: None,
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    argument: DecTerm::SymbolicVariable(
                                        DecSymbolicVariable {
                                            toolchain: Toolchain {
                                                data: ToolchainData::Local {
                                                    library_path: "../../../library",
                                                },
                                            },
                                            ty: Ok(
                                                Category(
                                                    Sort {
                                                        universe: Universe(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                Type {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 2,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::result::Result as core::ops::Unveil(0)>::Continue`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::AssocType(
                        TraitForTypeAssocTypeDecTemplate {
                            path: TraitForTypeItemPath(
                                `<core::result::Result as core::ops::Unveil(0)>::Continue`,
                                TraitItemKind::AssocType,
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            ty_term: DecTerm::SymbolicVariable(
                                DecSymbolicVariable {
                                    toolchain: Toolchain {
                                        data: ToolchainData::Local {
                                            library_path: "../../../library",
                                        },
                                    },
                                    ty: Ok(
                                        Category(
                                            Sort {
                                                universe: Universe(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                    index: DecSymbolicVariableIndex(
                                        Type {
                                            attrs: DeclarativeTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 3,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::result::Result as core::ops::Unveil(0)>::unveil`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::AssocRitchie(
                        TraitForTypeAssocRitchieDecTemplate {
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                            ),
                                            argument: DecTerm::SymbolicVariable(
                                                DecSymbolicVariable {
                                                    toolchain: Toolchain {
                                                        data: ToolchainData::Local {
                                                            library_path: "../../../library",
                                                        },
                                                    },
                                                    ty: Ok(
                                                        Category(
                                                            Sort {
                                                                universe: Universe(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    index: DecSymbolicVariableIndex(
                                                        Type {
                                                            attrs: DeclarativeTemplateVariableAttrs {
                                                                class: Mono,
                                                            },
                                                            variance: None,
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    argument: DecTerm::SymbolicVariable(
                                        DecSymbolicVariable {
                                            toolchain: Toolchain {
                                                data: ToolchainData::Local {
                                                    library_path: "../../../library",
                                                },
                                            },
                                            ty: Ok(
                                                Category(
                                                    Sort {
                                                        universe: Universe(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                Type {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 2,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::Application(
                                                        DecApplication {
                                                            function: DecTerm::EntityPath(
                                                                DecItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                            argument: DecTerm::SymbolicVariable(
                                                                DecSymbolicVariable {
                                                                    toolchain: Toolchain {
                                                                        data: ToolchainData::Local {
                                                                            library_path: "../../../library",
                                                                        },
                                                                    },
                                                                    ty: Ok(
                                                                        Category(
                                                                            Sort {
                                                                                universe: Universe(
                                                                                    1,
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    index: DecSymbolicVariableIndex(
                                                                        Type {
                                                                            attrs: DeclarativeTemplateVariableAttrs {
                                                                                class: Mono,
                                                                            },
                                                                            variance: None,
                                                                            disambiguator: 1,
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    argument: DecTerm::SymbolicVariable(
                                                        DecSymbolicVariable {
                                                            toolchain: Toolchain {
                                                                data: ToolchainData::Local {
                                                                    library_path: "../../../library",
                                                                },
                                                            },
                                                            ty: Ok(
                                                                Category(
                                                                    Sort {
                                                                        universe: Universe(
                                                                            1,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            index: DecSymbolicVariableIndex(
                                                                Type {
                                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 3,
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                            ),
                                            argument: DecTerm::SymbolicVariable(
                                                DecSymbolicVariable {
                                                    toolchain: Toolchain {
                                                        data: ToolchainData::Local {
                                                            library_path: "../../../library",
                                                        },
                                                    },
                                                    ty: Ok(
                                                        Category(
                                                            Sort {
                                                                universe: Universe(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    index: DecSymbolicVariableIndex(
                                                        Type {
                                                            attrs: DeclarativeTemplateVariableAttrs {
                                                                class: Mono,
                                                            },
                                                            variance: None,
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    argument: DecTerm::SymbolicVariable(
                                        DecSymbolicVariable {
                                            toolchain: Toolchain {
                                                data: ToolchainData::Local {
                                                    library_path: "../../../library",
                                                },
                                            },
                                            ty: Ok(
                                                Category(
                                                    Sort {
                                                        universe: Universe(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                Type {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 2,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```
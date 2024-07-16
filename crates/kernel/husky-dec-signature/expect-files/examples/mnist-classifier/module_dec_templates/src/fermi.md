```rust
[
    (
        ItemPath(`mnist_classifier::fermi::FermiMatchResult`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `matches`,
                                    ty: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::List(
                                                DecList {
                                                    toolchain: Toolchain {
                                                        data: ToolchainData::Local {
                                                            library_path: "../../../library",
                                                        },
                                                    },
                                                    items: [],
                                                },
                                            ),
                                            argument: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`core::option::Option`, `Enum`),
                                                        ),
                                                    ),
                                                    argument: DecTerm::Application(
                                                        DecApplication {
                                                            function: DecTerm::LeashOrBitNot(
                                                                Toolchain {
                                                                    data: ToolchainData::Local {
                                                                        library_path: "../../../library",
                                                                    },
                                                                },
                                                            ),
                                                            argument: DecTerm::EntityPath(
                                                                DecItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `others`,
                                    ty: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::List(
                                                DecList {
                                                    toolchain: Toolchain {
                                                        data: ToolchainData::Local {
                                                            library_path: "../../../library",
                                                        },
                                                    },
                                                    items: [],
                                                },
                                            ),
                                            argument: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::List(
                                                        DecList {
                                                            toolchain: Toolchain {
                                                                data: ToolchainData::Local {
                                                                    library_path: "../../../library",
                                                                },
                                                            },
                                                            items: [],
                                                        },
                                                    ),
                                                    argument: DecTerm::Application(
                                                        DecApplication {
                                                            function: DecTerm::EntityPath(
                                                                DecItemPath::Type(
                                                                    TypePath(`core::option::Option`, `Enum`),
                                                                ),
                                                            ),
                                                            argument: DecTerm::Application(
                                                                DecApplication {
                                                                    function: DecTerm::LeashOrBitNot(
                                                                        Toolchain {
                                                                            data: ToolchainData::Local {
                                                                                library_path: "../../../library",
                                                                            },
                                                                        },
                                                                    ),
                                                                    argument: DecTerm::EntityPath(
                                                                        DecItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::List(
                                                        DecList {
                                                            toolchain: Toolchain {
                                                                data: ToolchainData::Local {
                                                                    library_path: "../../../library",
                                                                },
                                                            },
                                                            items: [],
                                                        },
                                                    ),
                                                    argument: DecTerm::Application(
                                                        DecApplication {
                                                            function: DecTerm::LeashOrBitNot(
                                                                Toolchain {
                                                                    data: ToolchainData::Local {
                                                                        library_path: "../../../library",
                                                                    },
                                                                },
                                                            ),
                                                            argument: DecTerm::EntityPath(
                                                                DecItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::fermi::fermi_match`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::Application(
                                                        DecApplication {
                                                            function: DecTerm::List(
                                                                DecList {
                                                                    toolchain: Toolchain {
                                                                        data: ToolchainData::Local {
                                                                            library_path: "../../../library",
                                                                        },
                                                                    },
                                                                    items: [],
                                                                },
                                                            ),
                                                            argument: DecTerm::EntityPath(
                                                                DecItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::ApplicationOrRitchieCall(
                                                DecApplicationOrRitchieCall {
                                                    function: DecTerm::List(
                                                        DecList {
                                                            toolchain: Toolchain {
                                                                data: ToolchainData::Local {
                                                                    library_path: "../../../library",
                                                                },
                                                            },
                                                            items: [],
                                                        },
                                                    ),
                                                    template_arguments: [],
                                                    items: [
                                                        DecTerm::Ritchie(
                                                            DecRitchie {
                                                                ritchie_kind: RitchieKind::Type(
                                                                    RitchieTypeKind::Item(
                                                                        RitchieItemKind::Fn,
                                                                    ),
                                                                ),
                                                                params: [
                                                                    DeclarativeRitchieParameter::Simple(
                                                                        DeclarativeRitchieSimpleParameter {
                                                                            contract: Pure,
                                                                            ty: DecTerm::Application(
                                                                                DecApplication {
                                                                                    function: DecTerm::LeashOrBitNot(
                                                                                        Toolchain {
                                                                                            data: ToolchainData::Local {
                                                                                                library_path: "../../../library",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    argument: DecTerm::EntityPath(
                                                                                        DecItemPath::Type(
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        },
                                                                    ),
                                                                ],
                                                                return_ty: DecTerm::Application(
                                                                    DecApplication {
                                                                        function: DecTerm::EntityPath(
                                                                            DecItemPath::Type(
                                                                                TypePath(`core::option::Option`, `Enum`),
                                                                            ),
                                                                        ),
                                                                        argument: DecTerm::EntityPath(
                                                                            DecItemPath::Type(
                                                                                TypePath(`core::num::f32`, `Extern`),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                    extra_comma: false,
                                                },
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::fermi::FermiMatchResult(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::fermi::FermiMatchResult(0)::norm`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```
```rust
[
    (
        ItemPath(`core::vec::Vec`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
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
                            ],
                        },
                        ty: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::vec::Vec`, `Extern`),
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
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::ilen`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::ilen`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
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
                                    ],
                                },
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::vec::Vec`, `Extern`),
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
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::push`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::push`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
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
                                    ],
                                },
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::vec::Vec`, `Extern`),
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
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::BorrowMut,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: DecTerm::SymbolicVariable(
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
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::first`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::first`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
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
                                    ],
                                },
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::vec::Vec`, `Extern`),
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
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::At,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::option::Option`, `Enum`),
                                                ),
                                            ),
                                            argument: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`core::mem::At`, `Extern`),
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
                                                                EntityPath(
                                                                    Type(
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
                                                            index: DecSymbolicVariableIndex(
                                                                SelfPlace,
                                                            ),
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
                                                    disambiguator: 0,
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
        ItemPath(`core::vec::Vec(0)::last`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::last`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
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
                                    ],
                                },
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::vec::Vec`, `Extern`),
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
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::At,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::option::Option`, `Enum`),
                                                ),
                                            ),
                                            argument: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`core::mem::At`, `Extern`),
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
                                                                EntityPath(
                                                                    Type(
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
                                                            index: DecSymbolicVariableIndex(
                                                                SelfPlace,
                                                            ),
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
                                                    disambiguator: 0,
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
        ItemPath(`core::vec::Vec(0)::pop`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::pop`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
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
                                    ],
                                },
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::vec::Vec`, `Extern`),
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
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::BorrowMut,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::option::Option`, `Enum`),
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
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::collect_leashes`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::collect_leashes`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
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
                                    ],
                                },
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::vec::Vec`, `Extern`),
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
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Leash,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::Application(
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
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::cyclic_slice_leashed`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
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
                                    ],
                                },
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::vec::Vec`, `Extern`),
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
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Leash,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::Application(
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
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::pop_with_largest_opt_f32`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::pop_with_largest_opt_f32`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
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
                                    ],
                                },
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::vec::Vec`, `Extern`),
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
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::BorrowMut,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::EntityPath(
                                            DecItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
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
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::Ritchie(
                                                DecRitchie {
                                                    ritchie_kind: RitchieKind::Type(
                                                        RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                    params: [
                                                        DeclarativeRitchieParameter::Simple(
                                                            DeclarativeRitchieSimpleParameter {
                                                                contract: Contract::Pure,
                                                                ty: DecTerm::SymbolicVariable(
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
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::option::Option`, `Enum`),
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
                        },
                    ),
                ),
            ),
        ),
    ),
]
```
```rust
[
    (
        ItemPath(`core::slice::Slice`),
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
        ItemPath(`core::slice::CyclicSlice`),
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
        ItemPath(`core::slice::Slice(0)`),
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
                                function: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`core::slice::Slice`, `Extern`),
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
        ItemPath(`core::slice::Slice(0)::len`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::slice::Slice(0)::len`,
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
                                        function: DecTerm::ItemPath(
                                            DecItemPath::Type(
                                                TypePath(`core::slice::Slice`, `Extern`),
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
                                    function: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`core::slice::Slice`, `Extern`),
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
                                        function: DecTerm::ItemPath(
                                            DecItemPath::Type(
                                                TypePath(`core::slice::Slice`, `Extern`),
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
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::usize`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::slice::Slice(0)::swap`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::slice::Slice(0)::swap`,
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
                                        function: DecTerm::ItemPath(
                                            DecItemPath::Type(
                                                TypePath(`core::slice::Slice`, `Extern`),
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
                                    function: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`core::slice::Slice`, `Extern`),
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
                                        function: DecTerm::ItemPath(
                                            DecItemPath::Type(
                                                TypePath(`core::slice::Slice`, `Extern`),
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
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::ItemPath(
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
        ItemPath(`core::slice::CyclicSlice as core::ops::IntIndex(0)`),
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
                            ],
                        },
                        trai: DecTerm::ItemPath(
                            DecItemPath::Trait(
                                TraitPath(`core::ops::IntIndex`),
                            ),
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::ItemPath(
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
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::slice::CyclicSlice as core::ops::IntIndex(0)>::Output`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::AssocType(
                        TraitForTypeAssocTypeDecTemplate {
                            path: TraitForTypeItemPath(
                                `<core::slice::CyclicSlice as core::ops::IntIndex(0)>::Output`,
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
                                            disambiguator: 0,
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
        ItemPath(`core::slice::CyclicSlice(0)`),
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
                                function: DecTerm::ItemPath(
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
            ),
        ),
    ),
    (
        ItemPath(`core::slice::CyclicSlice(0)::ilen`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::slice::CyclicSlice(0)::ilen`,
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
                                        function: DecTerm::ItemPath(
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
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::ItemPath(
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
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::ItemPath(
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
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::ItemPath(
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
        ItemPath(`core::slice::CyclicSlice(0)::start`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::slice::CyclicSlice(0)::start`,
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
                                        function: DecTerm::ItemPath(
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
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::ItemPath(
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
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::ItemPath(
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
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::ItemPath(
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
        ItemPath(`core::slice::CyclicSlice(0)::end`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::slice::CyclicSlice(0)::end`,
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
                                        function: DecTerm::ItemPath(
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
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::ItemPath(
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
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::ItemPath(
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
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::ItemPath(
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
        ItemPath(`core::slice::CyclicSlice(0)::first`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::slice::CyclicSlice(0)::first`,
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
                                        function: DecTerm::ItemPath(
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
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::ItemPath(
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
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::ItemPath(
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
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::ItemPath(
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
        ItemPath(`core::slice::CyclicSlice(0)::last`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::slice::CyclicSlice(0)::last`,
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
                                        function: DecTerm::ItemPath(
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
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::ItemPath(
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
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::Application(
                                    DecApplication {
                                        function: DecTerm::ItemPath(
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
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::ItemPath(
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
]
```
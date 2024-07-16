```rust
[
    (
        ItemPath(`malamute::Class`),
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
                                                value: 3,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`malamute::Class`, `Enum`),
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
        ItemPath(`malamute::OneVsAll`),
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
                                                value: 4,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 5,
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
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                                class: Phan,
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
                                                SymbolicVariable(
                                                    DecSymbolicVariable(
                                                        Id {
                                                            value: 4,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                ConstOther {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Phan,
                                                    },
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
        ItemPath(`malamute::OneVsAllResult`),
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
                                                value: 4,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 5,
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
                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                                                class: Phan,
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
                                                SymbolicVariable(
                                                    DecSymbolicVariable(
                                                        Id {
                                                            value: 4,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                ConstOther {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Phan,
                                                    },
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
        ItemPath(`malamute::narrow_down`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Gn,
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Variadic(
                                        DeclarativeRitchieVariadicParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Keyed(
                                        DeclarativeRitchieKeyedParameter {
                                            key: `skip`,
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                            has_default: true,
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
                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                                SymbolicVariable(
                                                    DecSymbolicVariable(
                                                        Id {
                                                            value: 3,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                ConstOther {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Poly,
                                                    },
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
        ItemPath(`malamute::OneVsAll as core::default::Default(0)`),
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
                                            value: 4,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: DecTerm::EntityPath(
                            DecItemPath::Trait(
                                TraitPath(`core::default::Default`),
                            ),
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                                class: Phan,
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
                                                SymbolicVariable(
                                                    DecSymbolicVariable(
                                                        Id {
                                                            value: 4,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                ConstOther {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Phan,
                                                    },
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
        ItemPath(`<malamute::OneVsAll as core::default::Default(0)>::default`),
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
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                                class: Phan,
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
                                                SymbolicVariable(
                                                    DecSymbolicVariable(
                                                        Id {
                                                            value: 4,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                ConstOther {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Phan,
                                                    },
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
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                                class: Phan,
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
                                                SymbolicVariable(
                                                    DecSymbolicVariable(
                                                        Id {
                                                            value: 4,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                ConstOther {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Phan,
                                                    },
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
        ItemPath(`malamute::Class as core::ops::Unveil(0)`),
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
                                            value: 3,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 6,
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
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
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
                                            SymbolicVariable(
                                                DecSymbolicVariable(
                                                    Id {
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                        index: DecSymbolicVariableIndex(
                                            ConstOther {
                                                attrs: DeclarativeTemplateVariableAttrs {
                                                    class: Poly,
                                                },
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`malamute::Class`, `Enum`),
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
        ItemPath(`<malamute::Class as core::ops::Unveil(0)>::Output`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::AssocType(
                        TraitForTypeAssocTypeDecTemplate {
                            path: TraitForTypeItemPath(
                                `<malamute::Class as core::ops::Unveil(0)>::Output`,
                                TraitItemKind::AssocType,
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            ty_term: DecTerm::EntityPath(
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
        ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::AssocRitchie(
                        TraitForTypeAssocRitchieDecTemplate {
                            self_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`malamute::Class`, `Enum`),
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
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                                SymbolicVariable(
                                                                    DecSymbolicVariable(
                                                                        Id {
                                                                            value: 3,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            index: DecSymbolicVariableIndex(
                                                                ConstOther {
                                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                                        class: Poly,
                                                                    },
                                                                    disambiguator: 0,
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
                            return_ty: DecTerm::ApplicationOrRitchieCall(
                                DecApplicationOrRitchieCall {
                                    function: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::ops::ControlFlow`, `Enum`),
                                                ),
                                            ),
                                            argument: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`malamute::Class`, `Enum`),
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
                                    template_arguments: [],
                                    items: [],
                                    extra_comma: false,
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
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
                                            value: 4,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 5,
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
                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                                            class: Phan,
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
                                            SymbolicVariable(
                                                DecSymbolicVariable(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                        index: DecSymbolicVariableIndex(
                                            ConstOther {
                                                attrs: DeclarativeTemplateVariableAttrs {
                                                    class: Phan,
                                                },
                                                disambiguator: 0,
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
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                                class: Phan,
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
                                                SymbolicVariable(
                                                    DecSymbolicVariable(
                                                        Id {
                                                            value: 4,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                ConstOther {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Phan,
                                                    },
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
        ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::Output`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::AssocType(
                        TraitForTypeAssocTypeDecTemplate {
                            path: TraitForTypeItemPath(
                                `<malamute::OneVsAll as core::ops::Unveil(0)>::Output`,
                                TraitItemKind::AssocType,
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            ty_term: DecTerm::EntityPath(
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
        ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
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
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                                class: Phan,
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
                                                SymbolicVariable(
                                                    DecSymbolicVariable(
                                                        Id {
                                                            value: 4,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            index: DecSymbolicVariableIndex(
                                                ConstOther {
                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                        class: Phan,
                                                    },
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
                                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                                                                class: Phan,
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
                                                                SymbolicVariable(
                                                                    DecSymbolicVariable(
                                                                        Id {
                                                                            value: 4,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            index: DecSymbolicVariableIndex(
                                                                ConstOther {
                                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                                        class: Phan,
                                                                    },
                                                                    disambiguator: 0,
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
                            return_ty: DecTerm::ApplicationOrRitchieCall(
                                DecApplicationOrRitchieCall {
                                    function: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::ops::ControlFlow`, `Enum`),
                                                ),
                                            ),
                                            argument: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::Application(
                                                        DecApplication {
                                                            function: DecTerm::EntityPath(
                                                                DecItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                                                class: Phan,
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
                                                                SymbolicVariable(
                                                                    DecSymbolicVariable(
                                                                        Id {
                                                                            value: 4,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            index: DecSymbolicVariableIndex(
                                                                ConstOther {
                                                                    attrs: DeclarativeTemplateVariableAttrs {
                                                                        class: Phan,
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    template_arguments: [],
                                    items: [],
                                    extra_comma: false,
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
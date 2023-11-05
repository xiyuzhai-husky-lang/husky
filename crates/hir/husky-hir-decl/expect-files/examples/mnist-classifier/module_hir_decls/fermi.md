[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            PropsStruct(
                PropsStructTypeHirDecl(
                    Id {
                        value: 4,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirParenateParameters(
                        [
                            Ordinary,
                            Ordinary,
                        ],
                    ),
                },
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `mnist_classifier::fermi`,
                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 45,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MemoizedField(
                TypeMemoizedFieldHirDecl(
                    Id {
                        value: 9,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MemoizedField(
                TypeMemoizedFieldHirDecl(
                    Id {
                        value: 10,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MemoizedField(
                TypeMemoizedFieldHirDecl(
                    Id {
                        value: 11,
                    },
                ),
            ),
        ),
    ),
]
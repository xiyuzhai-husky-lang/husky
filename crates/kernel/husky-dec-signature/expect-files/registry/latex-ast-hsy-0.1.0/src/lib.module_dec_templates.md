```rust
[
    (
        ItemPath(`latex_ast_hsy::LxAstId`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`latex_ast_hsy::AST`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::StaticVar(
                        MajorStaticVarDecTemplate {
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`latex_ast_hsy::LxAstId`, `Extern`),
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
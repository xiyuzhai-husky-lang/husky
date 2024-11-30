```rust
[
    (
        ItemPath(`latex_ast_hsy::LxAstId`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`latex_ast_hsy::LxAstId`, `Extern`),
                            template_parameters: EthTemplateParameters {
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
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::StaticVar(
                        MajorStaticVarEthTemplate {
                            path: MajorFormPath(`latex_ast_hsy::AST`, `StaticVar`),
                            return_ty: EthTerm(`LxAstId`),
                            expr_ty: EthTerm(`Leash LxAstId`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```
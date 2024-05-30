```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                            return_ty: EthTerm(`OneVsAll MnistLabel Five`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```
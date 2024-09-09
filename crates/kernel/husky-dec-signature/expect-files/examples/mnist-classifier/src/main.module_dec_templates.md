```rust
[
    (
        ItemPath(`mnist_classifier::connected_component`),
        Ok(
            ItemDecTemplate::Submodule,
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour`),
        Ok(
            ItemDecTemplate::Submodule,
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d`),
        Ok(
            ItemDecTemplate::Submodule,
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch`),
        Ok(
            ItemDecTemplate::Submodule,
        ),
    ),
    (
        ItemPath(`mnist_classifier::fermi`),
        Ok(
            ItemDecTemplate::Submodule,
        ),
    ),
    (
        ItemPath(`mnist_classifier::digits`),
        Ok(
            ItemDecTemplate::Submodule,
        ),
    ),
    (
        ItemPath(`mnist_classifier::major`),
        Ok(
            ItemDecTemplate::Submodule,
        ),
    ),
    (
        ItemPath(`mnist_classifier::main`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`malamute::Class`, `Enum`),
                                        ),
                                    ),
                                    argument: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                        ),
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
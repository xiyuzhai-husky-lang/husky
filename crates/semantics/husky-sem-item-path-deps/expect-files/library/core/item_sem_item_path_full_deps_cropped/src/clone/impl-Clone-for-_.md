```rust
Some(
    [
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(`#derive _ as core::clone::Clone(0)`),
            ),
        ),
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::clone::Clone`),
            ),
        ),
    ],
)
```
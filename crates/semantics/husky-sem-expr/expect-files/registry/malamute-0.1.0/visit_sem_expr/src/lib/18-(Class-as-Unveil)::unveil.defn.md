```rust
Some(
    [
        "one_vs_all",
        "core::ops::ControlFlow::Break",
        "Class::Known",
        "label",
        "Class::Known(label)",
        "core::ops::ControlFlow::Break(Class::Known(label))",
        "core::ops::ControlFlow::Break(Class::Known(label))",
        "core::ops::ControlFlow::Continue",
        "()",
        "core::ops::ControlFlow::Continue(())",
        "core::ops::ControlFlow::Continue(())",
        "match one_vs_all with\n        | OneVsAll::Yes => core::ops::ControlFlow::Break(Class::Known(label))\n        | OneVsAll::No => core::ops::ControlFlow::Continue(())",
        "match one_vs_all with\n        | OneVsAll::Yes => core::ops::ControlFlow::Break(Class::Known(label))\n        | OneVsAll::No => core::ops::ControlFlow::Continue(())",
    ],
)
```
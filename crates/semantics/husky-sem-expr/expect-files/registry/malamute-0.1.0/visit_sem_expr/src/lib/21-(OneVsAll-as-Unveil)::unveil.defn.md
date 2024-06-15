```rust
Some(
    [
        "one_vs_all_result",
        "core::ops::ControlFlow::Break",
        "OneVsAll::Yes",
        "core::ops::ControlFlow::Break(OneVsAll::Yes)",
        "core::ops::ControlFlow::Break(OneVsAll::Yes)",
        "core::ops::ControlFlow::Break",
        "OneVsAll::No",
        "core::ops::ControlFlow::Break(OneVsAll::No)",
        "core::ops::ControlFlow::Break(OneVsAll::No)",
        "core::ops::ControlFlow::Continue",
        "()",
        "core::ops::ControlFlow::Continue(())",
        "core::ops::ControlFlow::Continue(())",
        "match one_vs_all_result with\n        | OneVsAllResult::ConfidentYes => core::ops::ControlFlow::Break(OneVsAll::Yes)\n        | OneVsAllResult::ConfidentNo => core::ops::ControlFlow::Break(OneVsAll::No)\n        | OneVsAllResult::Unconfident => core::ops::ControlFlow::Continue(())",
        "match one_vs_all_result with\n        | OneVsAllResult::ConfidentYes => core::ops::ControlFlow::Break(OneVsAll::Yes)\n        | OneVsAllResult::ConfidentNo => core::ops::ControlFlow::Break(OneVsAll::No)\n        | OneVsAllResult::Unconfident => core::ops::ControlFlow::Continue(())",
    ],
)
```
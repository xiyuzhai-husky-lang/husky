## `Class` decl

```rust
Some(
    [],
)
```

## `Class` defn

```rust
None
```

## ``Class`::#derive` decl

```rust
Some(
    [
        "Debug",
        "Clone",
        "Copy",
    ],
)
```

## ``Class`::#derive` defn

```rust
None
```

## `Class::Known` decl

```rust
Some(
    [
        "Label",
    ],
)
```

## `Class::Known` defn

```rust
None
```

## `Class::Unknown` decl

```rust
Some(
    [],
)
```

## `Class::Unknown` defn

```rust
None
```

## `OneVsAll` decl

```rust
Some(
    [
        "Label",
    ],
)
```

## `OneVsAll` defn

```rust
None
```

## ``OneVsAll`::#derive` decl

```rust
Some(
    [
        "Debug",
        "Clone",
        "Copy",
    ],
)
```

## ``OneVsAll`::#derive` defn

```rust
None
```

## `OneVsAll::Yes` decl

```rust
Some(
    [],
)
```

## `OneVsAll::Yes` defn

```rust
None
```

## `OneVsAll::No` decl

```rust
Some(
    [],
)
```

## `OneVsAll::No` defn

```rust
None
```

## `OneVsAllResult` decl

```rust
Some(
    [
        "Label",
    ],
)
```

## `OneVsAllResult` defn

```rust
None
```

## ``OneVsAllResult`::#derive` decl

```rust
Some(
    [
        "Debug",
        "Clone",
        "Copy",
    ],
)
```

## ``OneVsAllResult`::#derive` defn

```rust
None
```

## `OneVsAllResult::ConfidentYes` decl

```rust
Some(
    [],
)
```

## `OneVsAllResult::ConfidentYes` defn

```rust
None
```

## `OneVsAllResult::ConfidentNo` decl

```rust
Some(
    [],
)
```

## `OneVsAllResult::ConfidentNo` defn

```rust
None
```

## `OneVsAllResult::Unconfident` decl

```rust
Some(
    [],
)
```

## `OneVsAllResult::Unconfident` defn

```rust
None
```

## `narrow_down` decl

```rust
Some(
    [
        "Label",
        "f32",
        "i32",
        "5",
        "OneVsAllResult",
        "Label",
        "OneVsAllResult Label",
        "label",
        "OneVsAllResult Label label",
    ],
)
```

## `narrow_down` defn

```rust
None
```

## ``narrow_down`::#dep` decl

```rust
Some(
    [
        "Task",
    ],
)
```

## ``narrow_down`::#dep` defn

```rust
None
```

## `impl Default for OneVsAll` decl

```rust
Some(
    [
        "Label",
        "Default",
        "OneVsAll",
        "Label",
        "OneVsAll Label",
        "label",
        "OneVsAll Label label",
    ],
)
```

## `impl Default for OneVsAll` defn

```rust
None
```

## `(OneVsAll as Default)::default` decl

```rust
Some(
    [
        "Self",
    ],
)
```

## `(OneVsAll as Default)::default` defn

```rust
Some(
    [
        "OneVsAll::No",
        "OneVsAll::No",
        "OneVsAll::No",
    ],
)
```

## `impl Unveil for Class` decl

```rust
Some(
    [
        "Label",
        "core::ops::Unveil",
        "OneVsAll",
        "core::ops::Unveil OneVsAll",
        "Label",
        "core::ops::Unveil OneVsAll Label",
        "label",
        "core::ops::Unveil OneVsAll Label label",
        "Class",
        "Label",
        "Class Label",
    ],
)
```

## `impl Unveil for Class` defn

```rust
None
```

## `(Class as Unveil)::Output` decl

```rust
Some(
    [
        "()",
    ],
)
```

## `(Class as Unveil)::Output` defn

```rust
None
```

## `(Class as Unveil)::unveil` decl

```rust
Some(
    [
        "OneVsAll",
        "Label",
        "OneVsAll Label",
        "label",
        "OneVsAll Label label",
        "core::ops::ControlFlow",
        "Self",
        "core::ops::ControlFlow Self",
        "()",
        "core::ops::ControlFlow Self ()",
    ],
)
```

## `(Class as Unveil)::unveil` defn

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

## `impl Unveil for OneVsAll` decl

```rust
Some(
    [
        "Label",
        "core::ops::Unveil",
        "OneVsAllResult",
        "core::ops::Unveil OneVsAllResult",
        "Label",
        "core::ops::Unveil OneVsAllResult Label",
        "LABEL",
        "core::ops::Unveil OneVsAllResult Label LABEL",
        "OneVsAll",
        "Label",
        "OneVsAll Label",
        "LABEL",
        "OneVsAll Label LABEL",
    ],
)
```

## `impl Unveil for OneVsAll` defn

```rust
None
```

## `(OneVsAll as Unveil)::Output` decl

```rust
Some(
    [
        "()",
    ],
)
```

## `(OneVsAll as Unveil)::Output` defn

```rust
None
```

## `(OneVsAll as Unveil)::unveil` decl

```rust
Some(
    [
        "OneVsAllResult",
        "Label",
        "OneVsAllResult Label",
        "LABEL",
        "OneVsAllResult Label LABEL",
        "core::ops::ControlFlow",
        "Self",
        "core::ops::ControlFlow Self",
        "()",
        "core::ops::ControlFlow Self ()",
    ],
)
```

## `(OneVsAll as Unveil)::unveil` defn

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

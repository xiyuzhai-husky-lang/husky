## `Class`

```rust
[
    ItemPath(`malamute::Class`),
]
```

## ``Class`::#derive`

```rust
[
    ItemPath(`malamute::Class::#derive(0)`),
    ItemPath(`core::fmt::Debug`),
    ItemPath(`core::clone::Clone`),
    ItemPath(`core::marker::Copy`),
]
```

## `Class::Known`

```rust
[
    ItemPath(`malamute::Class::Known`),
]
```

## `Class::Unknown`

```rust
[
    ItemPath(`malamute::Class::Unknown`),
]
```

## `OneVsAll`

```rust
[
    ItemPath(`malamute::OneVsAll`),
]
```

## ``OneVsAll`::#derive`

```rust
[
    ItemPath(`malamute::OneVsAll::#derive(0)`),
    ItemPath(`core::fmt::Debug`),
    ItemPath(`core::clone::Clone`),
    ItemPath(`core::marker::Copy`),
]
```

## `OneVsAll::Yes`

```rust
[
    ItemPath(`malamute::OneVsAll::Yes`),
]
```

## `OneVsAll::No`

```rust
[
    ItemPath(`malamute::OneVsAll::No`),
]
```

## `OneVsAllResult`

```rust
[
    ItemPath(`malamute::OneVsAllResult`),
]
```

## ``OneVsAllResult`::#derive`

```rust
[
    ItemPath(`malamute::OneVsAllResult::#derive(0)`),
    ItemPath(`core::fmt::Debug`),
    ItemPath(`core::clone::Clone`),
    ItemPath(`core::marker::Copy`),
]
```

## `OneVsAllResult::ConfidentYes`

```rust
[
    ItemPath(`malamute::OneVsAllResult::ConfidentYes`),
]
```

## `OneVsAllResult::ConfidentNo`

```rust
[
    ItemPath(`malamute::OneVsAllResult::ConfidentNo`),
]
```

## `OneVsAllResult::Unconfident`

```rust
[
    ItemPath(`malamute::OneVsAllResult::Unconfident`),
]
```

## `narrow_down`

```rust
[
    ItemPath(`malamute::narrow_down`),
    ItemPath(`core::num::f32`),
    ItemPath(`core::num::i32`),
    ItemPath(`malamute::OneVsAllResult`),
]
```

## ``narrow_down`::#dep`

```rust
[
    ItemPath(`malamute::narrow_down::#dep(0)`),
    ItemPath(`core::task::Task`),
]
```

## `impl Default for OneVsAll`

```rust
[
    ItemPath(`malamute::OneVsAll as core::default::Default(0)`),
    ItemPath(`core::default::Default`),
    ItemPath(`malamute::OneVsAll`),
]
```

## `(OneVsAll as Default)::default`

```rust
[
    ItemPath(`<malamute::OneVsAll as core::default::Default(0)>::default`),
    ItemPath(`malamute::OneVsAll::No`),
]
```

## `impl Unveil for Class`

```rust
[
    ItemPath(`malamute::Class as core::ops::Unveil(0)`),
    ItemPath(`core::ops::Unveil`),
    ItemPath(`malamute::OneVsAll`),
    ItemPath(`malamute::Class`),
]
```

## `(Class as Unveil)::Output`

```rust
[
    ItemPath(`<malamute::Class as core::ops::Unveil(0)>::Output`),
]
```

## `(Class as Unveil)::unveil`

```rust
[
    ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
    ItemPath(`malamute::OneVsAll`),
    ItemPath(`core::ops::ControlFlow`),
    ItemPath(`core::ops::ControlFlow::Break`),
    ItemPath(`malamute::Class::Known`),
    ItemPath(`core::ops::ControlFlow::Continue`),
]
```

## `impl Unveil for OneVsAll`

```rust
[
    ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
    ItemPath(`core::ops::Unveil`),
    ItemPath(`malamute::OneVsAllResult`),
    ItemPath(`malamute::OneVsAll`),
]
```

## `(OneVsAll as Unveil)::Output`

```rust
[
    ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::Output`),
]
```

## `(OneVsAll as Unveil)::unveil`

```rust
[
    ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
    ItemPath(`malamute::OneVsAllResult`),
    ItemPath(`core::ops::ControlFlow`),
    ItemPath(`core::ops::ControlFlow::Break`),
    ItemPath(`malamute::OneVsAll::Yes`),
    ItemPath(`malamute::OneVsAll::No`),
    ItemPath(`core::ops::ControlFlow::Continue`),
]
```

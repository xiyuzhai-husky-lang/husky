## `Clone`

```rust
SemItemPathDepsCyclceGroupItd {
    cycle_group: CycleGroup {
        nodes: [
            ItemPath(`core::clone::Clone`),
        ],
    },
}
```

## `Clone::clone`

```rust
SemItemPathDepsCyclceGroupItd {
    cycle_group: CycleGroup {
        nodes: [
            ItemPath(`core::clone::Clone::clone`),
        ],
    },
}
```

## `impl Clone for _`

```rust
SemItemPathDepsCyclceGroupItd {
    cycle_group: CycleGroup {
        nodes: [
            ItemPath(`#derive _ as core::clone::Clone(0)`),
        ],
    },
}
```

## `(_ as Clone)::clone`

```rust
SemItemPathDepsCyclceGroupItd {
    cycle_group: CycleGroup {
        nodes: [
            ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
        ],
    },
}
```

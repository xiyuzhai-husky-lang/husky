## `FermiMatchResult` decl

```rust
Some(
    [
        "[",
        "ConcaveComponent",
        "~ConcaveComponent",
        "?~ConcaveComponent",
        "[]?~ConcaveComponent",
        "[",
        "ConcaveComponent",
        "~ConcaveComponent",
        "[]~ConcaveComponent",
    ],
)
```

## `FermiMatchResult` defn

```rust
None
```

## `fermi_match` decl

```rust
Some(
    [
        "[",
        "ConcaveComponent",
        "[]ConcaveComponent",
        "~[]ConcaveComponent",
        "[",
        "ConcaveComponent",
        "~ConcaveComponent",
        "f32",
        "?f32",
        "fn (~ConcaveComponent) -> ?f32",
        "[](fn (~ConcaveComponent) -> ?f32",
        "FermiMatchResult",
    ],
)
```

## `fermi_match` defn

```rust
Some(
    [
        "concave_components",
        "concave_components.collect_leashes()",
        "let mut others = concave_components.collect_leashes()",
        "[]",
        "let mut matches: []?~ConcaveComponent = []",
        "templates",
        "templates.ilen()",
        "i",
        "templates",
        "i",
        "templates[i]",
        "let template = templates[i]",
        "matches",
        "others",
        "template",
        "others.pop_with_largest_opt_f32(template)",
        "matches.push(others.pop_with_largest_opt_f32(template))",
        "matches.push(others.pop_with_largest_opt_f32(template))",
        "for i < templates.ilen():\n        let template = templates[i]\n        matches.push(others.pop_with_largest_opt_f32(template))",
        "FermiMatchResult",
        "matches",
        "others",
        "FermiMatchResult(matches, others)",
        "return FermiMatchResult(matches, others)",
        "let mut others = concave_components.collect_leashes()\n    let mut matches: []?~ConcaveComponent = []\n    // todo: change this to `for template in templates` after introducing `for ... in` loop\n    for i < templates.ilen():\n        let template = templates[i]\n        matches.push(others.pop_with_largest_opt_f32(template))\n    return FermiMatchResult(matches, others)",
    ],
)
```

## `impl FermiMatchResult` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `impl FermiMatchResult` defn

```rust
None
```

## `FermiMatchResult::norm` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `FermiMatchResult::norm` defn

```rust
Some(
    [
        "0.0",
        "let mut norm: f32 = 0.0",
        "self",
        "self.others",
        "self.others.ilen()",
        "i",
        "norm",
        "norm",
        "self",
        "self.others",
        "i",
        "self.others[i]",
        "self.others[i].norm",
        "norm.max(self.others[i].norm)",
        "norm = norm.max(self.others[i].norm)",
        "norm = norm.max(self.others[i].norm)",
        "for i < self.others.ilen():\n            norm = norm.max(self.others[i].norm)",
        "norm",
        "return norm",
        "let mut norm: f32 = 0.0\n        for i < self.others.ilen():\n            norm = norm.max(self.others[i].norm)\n        return norm",
    ],
)
```

## `FermiMatchResult::rel_norm` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `FermiMatchResult::rel_norm` defn

```rust
Some(
    [
        "0.0",
        "let mut norm: f32 = 0.0",
        "self",
        "self.others",
        "self.others.ilen()",
        "i",
        "norm",
        "norm",
        "self",
        "self.others",
        "i",
        "self.others[i]",
        "self.others[i].rel_norm",
        "norm.max(self.others[i].rel_norm)",
        "norm = norm.max(self.others[i].rel_norm)",
        "norm = norm.max(self.others[i].rel_norm)",
        "for i < self.others.ilen():\n            norm = norm.max(self.others[i].rel_norm)",
        "norm",
        "return norm",
        "let mut norm: f32 = 0.0\n        for i < self.others.ilen():\n            norm = norm.max(self.others[i].rel_norm)\n        return norm",
    ],
)
```

## `FermiMatchResult::angle_change_norm` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `FermiMatchResult::angle_change_norm` defn

```rust
Some(
    [
        "0.0",
        "let mut norm: f32 = 0.0",
        "self",
        "self.others",
        "self.others.ilen()",
        "i",
        "norm",
        "norm",
        "self",
        "self.others",
        "i",
        "self.others[i]",
        "self.others[i].angle_change",
        "self.others[i].angle_change.abs()",
        "norm.max(self.others[i].angle_change.abs())",
        "norm = norm.max(self.others[i].angle_change.abs())",
        "norm = norm.max(self.others[i].angle_change.abs())",
        "for i < self.others.ilen():\n            norm = norm.max(self.others[i].angle_change.abs())",
        "norm",
        "return norm",
        "let mut norm: f32 = 0.0\n        for i < self.others.ilen():\n            norm = norm.max(self.others[i].angle_change.abs())\n        return norm",
    ],
)
```

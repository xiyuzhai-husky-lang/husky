```rust
Some(
    [
        "let mut others = concave_components.collect_leashes()",
        "let mut matches: []?~ConcaveComponent = []",
        "for i < templates.ilen():\n        let template = templates[i]\n        matches.push(others.pop_with_largest_opt_f32(template))",
        "return FermiMatchResult(matches, others)",
        "let mut others = concave_components.collect_leashes()\n    let mut matches: []?~ConcaveComponent = []\n    // todo: change this to `for template in templates` after introducing `for ... in` loop\n    for i < templates.ilen():\n        let template = templates[i]\n        matches.push(others.pop_with_largest_opt_f32(template))\n    return FermiMatchResult(matches, others)",
    ],
)
```
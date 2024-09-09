## `ConnectedComponentDistribution` decl

```rust
Some(
    [
        "i32",
        "i32",
        "i32",
        "i32",
    ],
)
```

## `ConnectedComponentDistribution` defn

```rust
None
```

## `EffHoles` decl

```rust
Some(
    [
        "[",
        "RawContour",
        "~RawContour",
        "?~RawContour",
        "[]?~RawContour",
    ],
)
```

## `EffHoles` defn

```rust
None
```

## `hole_tmpl` decl

```rust
Some(
    [
        "RawContour",
        "~RawContour",
        "f32",
        "?f32",
    ],
)
```

## `hole_tmpl` defn

```rust
Some(
    [
        "ct",
        "ct.contour_len",
        "let len = ct.contour_len",
        "len",
        "4.0",
        "len > 4.0",
        "len > 4.0",
        "require len > 4.0",
        "len",
        "0.0",
        "len + 0.0",
        "len + 0.0",
        "let len = ct.contour_len\n    require len > 4.0\n    // ad hoc\n    len + 0.0",
    ],
)
```

## `ConnectedComponent` decl

```rust
Some(
    [
        "BinaryImage28",
    ],
)
```

## `ConnectedComponent` defn

```rust
None
```

## `horizontal_extend` decl

```rust
Some(
    [
        "r32",
        "r32",
        "r32",
    ],
)
```

## `horizontal_extend` defn

```rust
Some(
    [
        "a",
        "x",
        "x",
        "1",
        "x << 1",
        "(x << 1)",
        "x | (x << 1)",
        "x",
        "1",
        "x >> 1",
        "(x >> 1)",
        "x | (x << 1) | (x >> 1)",
        "(x | (x << 1) | (x >> 1))",
        "a & (x | (x << 1) | (x >> 1))",
        "let mut y = a & (x | (x << 1) | (x >> 1))",
        "a",
        "y",
        "y",
        "1",
        "y << 1",
        "(y << 1)",
        "y | (y << 1)",
        "y",
        "1",
        "y >> 1",
        "(y >> 1)",
        "y | (y << 1) | (y >> 1)",
        "(y | (y << 1) | (y >> 1))",
        "a & (y | (y << 1) | (y >> 1))",
        "let mut z = a & (y | (y << 1) | (y >> 1))",
        "z",
        "y",
        "z != y",
        "z != y",
        "y",
        "z",
        "y = z",
        "y = z",
        "z",
        "a",
        "y",
        "y",
        "1",
        "y << 1",
        "(y << 1)",
        "y | (y << 1)",
        "y",
        "1",
        "y >> 1",
        "(y >> 1)",
        "y | (y << 1) | (y >> 1)",
        "(y | (y << 1) | (y >> 1))",
        "a & (y | (y << 1) | (y >> 1))",
        "z = a & (y | (y << 1) | (y >> 1))",
        "z = a & (y | (y << 1) | (y >> 1))",
        "while z != y:\n        y = z\n        z = a & (y | (y << 1) | (y >> 1))",
        "y",
        "return y",
        "let mut y = a & (x | (x << 1) | (x >> 1))\n    let mut z = a & (y | (y << 1) | (y >> 1))\n    while z != y:\n        y = z\n        z = a & (y | (y << 1) | (y >> 1))\n    return y",
    ],
)
```

## `find_connected_components` decl

```rust
Some(
    [
        "BinaryImage28",
        "[",
        "ConnectedComponent",
        "[]ConnectedComponent",
    ],
)
```

## `find_connected_components` defn

```rust
Some(
    [
        "[]",
        "let mut result: []ConnectedComponent = []",
        "img",
        "img.clone()",
        "let mut unsearched = img.clone()",
        "30",
        "j",
        "unsearched",
        "j",
        "unsearched[j]",
        "unsearched[j]",
        "unsearched",
        "j",
        "unsearched[j]",
        "let a = unsearched[j]",
        "a",
        "a.ctz()",
        "let shift = a.ctz()",
        "BinaryImage28::new_zeros",
        "BinaryImage28::new_zeros()",
        "let mut mask = BinaryImage28::new_zeros()",
        "mask",
        "j",
        "mask[j]",
        "horizontal_extend",
        "a",
        "1r32",
        "shift",
        "1r32 << shift",
        "horizontal_extend(a, 1r32 << shift)",
        "mask[j] = horizontal_extend(a, 1r32 << shift)",
        "mask[j] = horizontal_extend(a, 1r32 << shift)",
        "false",
        "let mut flag = false",
        "flag",
        "!flag",
        "!flag",
        "flag",
        "true",
        "flag = true",
        "flag = true",
        "j",
        "let mut i = j",
        "30",
        "1",
        "30 - 1",
        "mask",
        "i",
        "1",
        "i + 1",
        "mask[i + 1]",
        "let old_row = mask[i + 1]",
        "old_row",
        "horizontal_extend",
        "img",
        "i",
        "1",
        "i + 1",
        "img[i + 1]",
        "mask",
        "i",
        "mask[i]",
        "horizontal_extend(img[i + 1], mask[i])",
        "old_row | horizontal_extend(img[i + 1], mask[i])",
        "let new_row = old_row | horizontal_extend(img[i + 1], mask[i])",
        "new_row",
        "!new_row",
        "!new_row",
        "break",
        "if !new_row:\n                        break",
        "old_row",
        "new_row",
        "old_row != new_row",
        "old_row != new_row",
        "flag",
        "false",
        "flag = false",
        "flag = false",
        "mask",
        "i",
        "1",
        "i + 1",
        "mask[i + 1]",
        "new_row",
        "mask[i + 1] = new_row",
        "mask[i + 1] = new_row",
        "if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row",
        "forext i < 30 - 1:\n                    let old_row = mask[i + 1]\n                    let new_row = old_row | horizontal_extend(img[i + 1], mask[i])\n                    if !new_row:\n                        break\n                    if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row",
        "j",
        "mask",
        "i",
        "mask[i]",
        "let old_row = mask[i]",
        "old_row",
        "horizontal_extend",
        "img",
        "i",
        "img[i]",
        "mask",
        "i",
        "1",
        "i + 1",
        "mask[i + 1]",
        "horizontal_extend(img[i], mask[i + 1])",
        "old_row | horizontal_extend(img[i], mask[i + 1])",
        "let new_row = old_row | horizontal_extend(img[i], mask[i + 1])",
        "old_row",
        "new_row",
        "old_row != new_row",
        "old_row != new_row",
        "flag",
        "false",
        "flag = false",
        "flag = false",
        "mask",
        "i",
        "mask[i]",
        "new_row",
        "mask[i] = new_row",
        "mask[i] = new_row",
        "if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row",
        "forext i >= j:\n                    let old_row = mask[i]\n                    let new_row = old_row | horizontal_extend(img[i], mask[i + 1])\n                    if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row",
        "while !flag:\n                flag = true\n                let mut i = j\n                forext i < 30 - 1:\n                    let old_row = mask[i + 1]\n                    let new_row = old_row | horizontal_extend(img[i + 1], mask[i])\n                    if !new_row:\n                        break\n                    if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row\n                forext i >= j:\n                    let old_row = mask[i]\n                    let new_row = old_row | horizontal_extend(img[i], mask[i + 1])\n                    if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row",
        "j",
        "30",
        "k",
        "unsearched",
        "k",
        "unsearched[k]",
        "mask",
        "k",
        "mask[k]",
        "~mask[k]",
        "(~mask[k])",
        "unsearched[k] &= (~mask[k])",
        "unsearched[k] &= (~mask[k])",
        "for j <= k < 30:\n                unsearched[k] &= (~mask[k])",
        "result",
        "ConnectedComponent",
        "mask",
        "ConnectedComponent(mask)",
        "result.push(ConnectedComponent(mask))",
        "result.push(ConnectedComponent(mask))",
        "while unsearched[j]:\n            let a = unsearched[j]\n            let shift = a.ctz()\n            let mut mask = BinaryImage28::new_zeros()\n            mask[j] = horizontal_extend(a, 1r32 << shift)\n            let mut flag = false\n            while !flag:\n                flag = true\n                let mut i = j\n                forext i < 30 - 1:\n                    let old_row = mask[i + 1]\n                    let new_row = old_row | horizontal_extend(img[i + 1], mask[i])\n                    if !new_row:\n                        break\n                    if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row\n                forext i >= j:\n                    let old_row = mask[i]\n                    let new_row = old_row | horizontal_extend(img[i], mask[i + 1])\n                    if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row\n            for j <= k < 30:\n                unsearched[k] &= (~mask[k])\n            result.push(ConnectedComponent(mask))",
        "for j < 30:\n        while unsearched[j]:\n            let a = unsearched[j]\n            let shift = a.ctz()\n            let mut mask = BinaryImage28::new_zeros()\n            mask[j] = horizontal_extend(a, 1r32 << shift)\n            let mut flag = false\n            while !flag:\n                flag = true\n                let mut i = j\n                forext i < 30 - 1:\n                    let old_row = mask[i + 1]\n                    let new_row = old_row | horizontal_extend(img[i + 1], mask[i])\n                    if !new_row:\n                        break\n                    if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row\n                forext i >= j:\n                    let old_row = mask[i]\n                    let new_row = old_row | horizontal_extend(img[i], mask[i + 1])\n                    if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row\n            for j <= k < 30:\n                unsearched[k] &= (~mask[k])\n            result.push(ConnectedComponent(mask))",
        "result",
        "return result",
        "let mut result: []ConnectedComponent = []\n    let mut unsearched = img.clone()\n    for j < 30:\n        while unsearched[j]:\n            let a = unsearched[j]\n            let shift = a.ctz()\n            let mut mask = BinaryImage28::new_zeros()\n            mask[j] = horizontal_extend(a, 1r32 << shift)\n            let mut flag = false\n            while !flag:\n                flag = true\n                let mut i = j\n                forext i < 30 - 1:\n                    let old_row = mask[i + 1]\n                    let new_row = old_row | horizontal_extend(img[i + 1], mask[i])\n                    if !new_row:\n                        break\n                    if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row\n                forext i >= j:\n                    let old_row = mask[i]\n                    let new_row = old_row | horizontal_extend(img[i], mask[i + 1])\n                    if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row\n            for j <= k < 30:\n                unsearched[k] &= (~mask[k])\n            result.push(ConnectedComponent(mask))\n    return result",
    ],
)
```

## `impl Visualize for ConnectedComponent` decl

```rust
Some(
    [
        "Visualize",
        "ConnectedComponent",
    ],
)
```

## `impl Visualize for ConnectedComponent` defn

```rust
None
```

## `(ConnectedComponent as Visualize)::visualize` decl

```rust
Some(
    [
        "Visual",
    ],
)
```

## `(ConnectedComponent as Visualize)::visualize` defn

```rust
Some(
    [
        "self",
        "self.mask",
        "self.mask.visualize()",
        "self.mask.visualize()",
        "self.mask.visualize()",
    ],
)
```

## `impl ConnectedComponent` decl

```rust
Some(
    [
        "ConnectedComponent",
    ],
)
```

## `impl ConnectedComponent` defn

```rust
None
```

## `ConnectedComponent::raw_contours` decl

```rust
Some(
    [
        "[",
        "RawContour",
        "[]RawContour",
    ],
)
```

## `ConnectedComponent::raw_contours` defn

```rust
Some(
    [
        "find_raw_contours",
        "self",
        "find_raw_contours(self)",
        "find_raw_contours(self)",
        "find_raw_contours(self)",
    ],
)
```

## `ConnectedComponent::eff_holes` decl

```rust
Some(
    [
        "EffHoles",
    ],
)
```

## `ConnectedComponent::eff_holes` defn

```rust
Some(
    [
        "self",
        "self.raw_contours",
        "self.raw_contours.collect_leashes()",
        "let mut raw_contours = self.raw_contours.collect_leashes()",
        "[]",
        "let mut matches: []?~RawContour = []",
        "raw_contours",
        "hole_tmpl",
        "raw_contours.pop_with_largest_opt_f32(hole_tmpl)",
        "raw_contours.pop_with_largest_opt_f32(hole_tmpl)",
        "matches",
        "raw_contours",
        "hole_tmpl",
        "raw_contours.pop_with_largest_opt_f32(hole_tmpl)",
        "matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl))",
        "matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl))",
        "matches",
        "raw_contours",
        "hole_tmpl",
        "raw_contours.pop_with_largest_opt_f32(hole_tmpl)",
        "matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl))",
        "matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl))",
        "EffHoles",
        "matches",
        "EffHoles(matches)",
        "return EffHoles(matches)",
        "let mut raw_contours = self.raw_contours.collect_leashes()\n        let mut matches: []?~RawContour = []\n        // ad hoc, should replace self with pop_first\n        raw_contours.pop_with_largest_opt_f32(hole_tmpl);\n        matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl))\n        matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl))\n        return EffHoles(matches)",
    ],
)
```

## `ConnectedComponent::max_hole_ilen` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `ConnectedComponent::max_hole_ilen` defn

```rust
Some(
    [
        "0",
        "let mut max_hole_ilen = 0",
        "self",
        "self.raw_contours",
        "let raw_contours = self.raw_contours",
        "0",
        "raw_contours",
        "raw_contours.ilen()",
        "i",
        "raw_contours",
        "i",
        "raw_contours[i]",
        "raw_contours[i].points",
        "raw_contours[i].points.ilen()",
        "let hole_ilen = raw_contours[i].points.ilen()",
        "max_hole_ilen",
        "hole_ilen",
        "max_hole_ilen < hole_ilen",
        "max_hole_ilen < hole_ilen",
        "max_hole_ilen",
        "hole_ilen",
        "max_hole_ilen = hole_ilen",
        "max_hole_ilen = hole_ilen",
        "if max_hole_ilen < hole_ilen:\n                max_hole_ilen = hole_ilen",
        "for 0 < i < raw_contours.ilen():\n            let hole_ilen = raw_contours[i].points.ilen()\n            if max_hole_ilen < hole_ilen:\n                max_hole_ilen = hole_ilen",
        "max_hole_ilen",
        "f32",
        "max_hole_ilen as f32",
        "return max_hole_ilen as f32",
        "let mut max_hole_ilen = 0\n        let raw_contours = self.raw_contours\n        for 0 < i < raw_contours.ilen():\n            let hole_ilen = raw_contours[i].points.ilen()\n            if max_hole_ilen < hole_ilen:\n                max_hole_ilen = hole_ilen\n        return max_hole_ilen as f32",
    ],
)
```

## `ConnectedComponent::max_row_span` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `ConnectedComponent::max_row_span` defn

```rust
Some(
    [
        "0",
        "let mut max_row: i32 = 0",
        "0",
        "29",
        "i",
        "max_row",
        "max_row",
        "self",
        "self.mask",
        "i",
        "self.mask[i]",
        "self.mask[i].span()",
        "max_row.max(self.mask[i].span())",
        "max_row = max_row.max(self.mask[i].span())",
        "max_row = max_row.max(self.mask[i].span())",
        "for 0 < i < 29:\n            max_row = max_row.max(self.mask[i].span())",
        "max_row",
        "f32",
        "max_row as f32",
        "return max_row as f32",
        "let mut max_row: i32 = 0\n        for 0 < i < 29:\n            max_row = max_row.max(self.mask[i].span())\n        return max_row as f32",
    ],
)
```

## `ConnectedComponent::row_span_sum` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `ConnectedComponent::row_span_sum` defn

```rust
Some(
    [
        "0",
        "let mut row_span_sum = 0",
        "0",
        "29",
        "i",
        "row_span_sum",
        "self",
        "self.mask",
        "i",
        "self.mask[i]",
        "self.mask[i].span()",
        "row_span_sum += self.mask[i].span()",
        "row_span_sum += self.mask[i].span()",
        "for 0 < i < 29:\n            row_span_sum += self.mask[i].span()",
        "row_span_sum",
        "f32",
        "row_span_sum as f32",
        "return row_span_sum as f32",
        "let mut row_span_sum = 0\n        for 0 < i < 29:\n            row_span_sum += self.mask[i].span()\n        return row_span_sum as f32",
    ],
)
```

## `ConnectedComponent::distribution` decl

```rust
Some(
    [
        "ConnectedComponentDistribution",
    ],
)
```

## `ConnectedComponent::distribution` defn

```rust
Some(
    [
        "1",
        "let mut row_start = 1",
        "29",
        "self",
        "self.mask",
        "row_start",
        "self.mask[row_start]",
        "self.mask[row_start]",
        "break",
        "if self.mask[row_start]:\n                break",
        "forext row_start < 29:\n            if self.mask[row_start]:\n                break",
        "row_start",
        "1",
        "row_start + 1",
        "let mut row_end = row_start + 1",
        "29",
        "self",
        "self.mask",
        "row_end",
        "self.mask[row_end]",
        "!self.mask[row_end]",
        "!self.mask[row_end]",
        "break",
        "if !self.mask[row_end]:\n                break",
        "forext row_end < 29:\n            if !self.mask[row_end]:\n                break",
        "row_end",
        "row_start",
        "row_end - row_start",
        "let height = row_end - row_start",
        "height",
        "2",
        "height / 2",
        "let half_height = height / 2",
        "0",
        "let mut upper_mass = 0",
        "row_start",
        "row_start",
        "half_height",
        "row_start + half_height",
        "i1",
        "upper_mass",
        "self",
        "self.mask",
        "i1",
        "self.mask[i1]",
        "self.mask[i1].co()",
        "upper_mass += self.mask[i1].co()",
        "upper_mass += self.mask[i1].co()",
        "for row_start <= i1 < row_start + half_height:\n            upper_mass += self.mask[i1].co()",
        "0",
        "let mut lower_mass = 0",
        "row_end",
        "row_end",
        "half_height",
        "row_end - half_height",
        "i2",
        "lower_mass",
        "self",
        "self.mask",
        "i2",
        "self.mask[i2]",
        "self.mask[i2].co()",
        "lower_mass += self.mask[i2].co()",
        "lower_mass += self.mask[i2].co()",
        "for row_end > i2 >= row_end - half_height:\n            lower_mass += self.mask[i2].co()",
        "ConnectedComponentDistribution",
        "row_start",
        "row_end",
        "upper_mass",
        "lower_mass",
        "ConnectedComponentDistribution(\n            row_start,\n            row_end,\n            upper_mass,\n            lower_mass,\n        )",
        "return ConnectedComponentDistribution(\n            row_start,\n            row_end,\n            upper_mass,\n            lower_mass,\n        )",
        "let mut row_start = 1\n        forext row_start < 29:\n            if self.mask[row_start]:\n                break\n        let mut row_end = row_start + 1\n        forext row_end < 29:\n            if !self.mask[row_end]:\n                break\n        let height = row_end - row_start\n        let half_height = height / 2\n        let mut upper_mass = 0\n        for row_start <= i1 < row_start + half_height:\n            upper_mass += self.mask[i1].co()\n        let mut lower_mass = 0\n        for row_end > i2 >= row_end - half_height:\n            lower_mass += self.mask[i2].co()\n        return ConnectedComponentDistribution(\n            row_start,\n            row_end,\n            upper_mass,\n            lower_mass,\n        )",
    ],
)
```

## `ConnectedComponent::upper_mass` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `ConnectedComponent::upper_mass` defn

```rust
Some(
    [
        "self",
        "self.distribution",
        "self.distribution.upper_mass",
        "f32",
        "self.distribution.upper_mass as f32",
        "self.distribution.upper_mass as f32",
        "self.distribution.upper_mass as f32",
    ],
)
```

## `ConnectedComponent::lower_mass` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `ConnectedComponent::lower_mass` defn

```rust
Some(
    [
        "self",
        "self.distribution",
        "self.distribution.lower_mass",
        "f32",
        "self.distribution.lower_mass as f32",
        "self.distribution.lower_mass as f32",
        "self.distribution.lower_mass as f32",
    ],
)
```

## `ConnectedComponent::top_k_row_span_sum` decl

```rust
Some(
    [
        "i32",
        "f32",
    ],
)
```

## `ConnectedComponent::top_k_row_span_sum` defn

```rust
Some(
    [
        "0",
        "let mut top_k_row_span_sum = 0",
        "k",
        "0",
        "k > 0",
        "k > 0",
        "assert k > 0",
        "1",
        "let mut i = 1",
        "29",
        "self",
        "self.mask",
        "i",
        "self.mask[i]",
        "self.mask[i]",
        "break",
        "if self.mask[i]:\n                break",
        "forext i < 29:\n            if self.mask[i]:\n                break",
        "i",
        "i",
        "k",
        "i + k",
        "j",
        "top_k_row_span_sum",
        "self",
        "self.mask",
        "j",
        "self.mask[j]",
        "self.mask[j].span()",
        "top_k_row_span_sum += self.mask[j].span()",
        "top_k_row_span_sum += self.mask[j].span()",
        "for i <= j < i + k:\n            top_k_row_span_sum += self.mask[j].span()",
        "top_k_row_span_sum",
        "f32",
        "top_k_row_span_sum as f32",
        "return top_k_row_span_sum as f32",
        "let mut top_k_row_span_sum = 0\n        assert k > 0\n        let mut i = 1\n        forext i < 29:\n            if self.mask[i]:\n                break\n        for i <= j < i + k:\n            top_k_row_span_sum += self.mask[j].span()\n        return top_k_row_span_sum as f32",
    ],
)
```

## `ConnectedComponent::top_k_row_right_mass_sum` decl

```rust
Some(
    [
        "i32",
        "f32",
    ],
)
```

## `ConnectedComponent::top_k_row_right_mass_sum` defn

```rust
Some(
    [
        "0",
        "let mut top_k_row_span_sum = 0",
        "k",
        "0",
        "k > 0",
        "k > 0",
        "assert k > 0",
        "1",
        "let mut i = 1",
        "29",
        "self",
        "self.mask",
        "i",
        "self.mask[i]",
        "self.mask[i]",
        "break",
        "if self.mask[i]:\n                break",
        "forext i < 29:\n            if self.mask[i]:\n                break",
        "i",
        "i",
        "k",
        "i + k",
        "j",
        "top_k_row_span_sum",
        "self",
        "self.mask",
        "j",
        "self.mask[j]",
        "self.mask[j].right_mass()",
        "top_k_row_span_sum += self.mask[j].right_mass()",
        "top_k_row_span_sum += self.mask[j].right_mass()",
        "for i <= j < i + k:\n            top_k_row_span_sum += self.mask[j].right_mass()",
        "top_k_row_span_sum",
        "f32",
        "top_k_row_span_sum as f32",
        "return top_k_row_span_sum as f32",
        "let mut top_k_row_span_sum = 0\n        assert k > 0\n        let mut i = 1\n        forext i < 29:\n            if self.mask[i]:\n                break\n        for i <= j < i + k:\n            top_k_row_span_sum += self.mask[j].right_mass()\n        return top_k_row_span_sum as f32",
    ],
)
```

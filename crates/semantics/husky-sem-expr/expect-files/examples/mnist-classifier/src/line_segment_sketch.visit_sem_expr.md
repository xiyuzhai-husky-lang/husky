## `concave_component` decl

```rust
None
```

## `concave_component` defn

```rust
None
```

## `convex_component` decl

```rust
None
```

## `convex_component` defn

```rust
None
```

## `convexity` decl

```rust
None
```

## `convexity` defn

```rust
None
```

## `line_segment` decl

```rust
None
```

## `line_segment` defn

```rust
None
```

## `LineSegmentStroke` decl

```rust
Some(
    [
        "CyclicSlice",
        "Point2d",
        "CyclicSlice Point2d",
        "~CyclicSlice Point2d",
        "Point2d",
        "points",
        "points.first()",
        "points.first()!",
        "points.first()!.clone()",
        "Point2d",
        "points",
        "points.last()",
        "points.last()!",
        "points.last()!.clone()",
    ],
)
```

## `LineSegmentStroke` defn

```rust
None
```

## `LineSegmentSketch` decl

```rust
Some(
    [
        "RawContour",
        "~RawContour",
        "[",
        "LineSegmentStroke",
        "[]LineSegmentStroke",
    ],
)
```

## `LineSegmentSketch` defn

```rust
None
```

## `go_right` decl

```rust
Some(
    [
        "Vector2d",
        "f32",
        "Vector2d",
    ],
)
```

## `go_right` defn

```rust
Some(
    [
        "u",
        "u.x",
        "u",
        "u.x",
        "u.x*u.x",
        "u",
        "u.y",
        "u",
        "u.y",
        "u.y*u.y",
        "u.x*u.x+u.y*u.y",
        "(u.x*u.x+u.y*u.y)",
        "(u.x*u.x+u.y*u.y).sqrt()",
        "let L = (u.x*u.x+u.y*u.y).sqrt()",
        "L",
        "r",
        "L > r",
        "L > r",
        "assert L > r",
        "r",
        "L",
        "r*L",
        "L",
        "L",
        "L*L",
        "r",
        "r",
        "r*r",
        "L*L-r*r",
        "(L*L-r*r)",
        "(L*L-r*r).sqrt()",
        "r*L/(L*L-r*r).sqrt()",
        "let dr = r*L/(L*L-r*r).sqrt()",
        "dr",
        "u",
        "u.y",
        "dr*u.y",
        "L",
        "dr*u.y/L",
        "let dx = dr*u.y/L",
        "dr",
        "-dr",
        "u",
        "u.x",
        "-dr*u.x",
        "L",
        "-dr*u.x/L",
        "let dy = -dr*u.x/L",
        "Vector2d",
        "u",
        "u.x",
        "dx",
        "u.x+dx",
        "u",
        "u.y",
        "dy",
        "u.y+dy",
        "Vector2d(u.x+dx, u.y+dy)",
        "Vector2d(u.x+dx, u.y+dy)",
        "let L = (u.x*u.x+u.y*u.y).sqrt()\n    assert L > r\n    let dr = r*L/(L*L-r*r).sqrt()\n    let dx = dr*u.y/L\n    let dy = -dr*u.x/L\n    Vector2d(u.x+dx, u.y+dy)",
    ],
)
```

## `go_left` decl

```rust
Some(
    [
        "Vector2d",
        "f32",
        "Vector2d",
    ],
)
```

## `go_left` defn

```rust
Some(
    [
        "u",
        "u.x",
        "u",
        "u.x",
        "u.x*u.x",
        "u",
        "u.y",
        "u",
        "u.y",
        "u.y*u.y",
        "u.x*u.x+u.y*u.y",
        "(u.x*u.x+u.y*u.y)",
        "(u.x*u.x+u.y*u.y).sqrt()",
        "let L = (u.x*u.x+u.y*u.y).sqrt()",
        "L",
        "r",
        "L > r",
        "L > r",
        "assert L > r",
        "r",
        "L",
        "r*L",
        "L",
        "L",
        "L*L",
        "r",
        "r",
        "r*r",
        "L*L-r*r",
        "(L*L-r*r)",
        "(L*L-r*r).sqrt()",
        "r*L/(L*L-r*r).sqrt()",
        "let dr = r*L/(L*L-r*r).sqrt()",
        "dr",
        "-dr",
        "u",
        "u.y",
        "-dr*u.y",
        "L",
        "-dr*u.y/L",
        "let dx = -dr*u.y/L",
        "dr",
        "u",
        "u.x",
        "dr*u.x",
        "L",
        "dr*u.x/L",
        "let dy = dr*u.x/L",
        "Vector2d",
        "u",
        "u.x",
        "dx",
        "u.x+dx",
        "u",
        "u.y",
        "dy",
        "u.y+dy",
        "Vector2d(u.x+dx, u.y+dy)",
        "Vector2d(u.x+dx, u.y+dy)",
        "let L = (u.x*u.x+u.y*u.y).sqrt()\n    assert L > r\n    let dr = r*L/(L*L-r*r).sqrt()\n    let dx = -dr*u.y/L\n    let dy = dr*u.x/L\n    Vector2d(u.x+dx, u.y+dy)",
    ],
)
```

## `extend_end` decl

```rust
Some(
    [
        "RawContour",
        "~RawContour",
        "i32",
        "f32",
        "i32",
    ],
)
```

## `extend_end` defn

```rust
Some(
    [
        "start",
        "let mut end = start",
        "ct",
        "start",
        "end",
        "1",
        "end + 1",
        "ct.displacement(start, end + 1)",
        "let mut dp = ct.displacement(start, end + 1)",
        "ct",
        "ct.points",
        "ct.points.ilen()",
        "let N = ct.points.ilen()",
        "start",
        "N",
        "start + N",
        "let max_end = start + N",
        "end",
        "max_end",
        "end <= max_end",
        "dp",
        "dp.norm()",
        "r",
        "dp.norm() < r",
        "end <= max_end and dp.norm() < r",
        "end <= max_end and dp.norm() < r",
        "end",
        "end++",
        "end++",
        "dp",
        "ct",
        "start",
        "end",
        "1",
        "end + 1",
        "ct.displacement(start, end + 1)",
        "dp = ct.displacement(start, end + 1)",
        "dp = ct.displacement(start, end + 1)",
        "while end <= max_end and dp.norm() < r:\n        end++\n        dp = ct.displacement(start, end + 1)",
        "dp",
        "dp.norm()",
        "r",
        "dp.norm() < r",
        "dp.norm() < r",
        "end",
        "return end",
        "if dp.norm() < r:\n        return end",
        "go_right",
        "dp",
        "r",
        "go_right(dp, r)",
        "let mut right_bound = go_right(dp, r)",
        "go_left",
        "dp",
        "r",
        "go_left(dp, r)",
        "let mut left_bound = go_left(dp, r)",
        "0.0",
        "let mut r_max = 0.0",
        "end",
        "max_end",
        "end <= max_end",
        "right_bound",
        "dp",
        "right_bound.rotation_direction_to(dp)",
        "0",
        "right_bound.rotation_direction_to(dp) >= 0",
        "end <= max_end\n            and right_bound.rotation_direction_to(dp) >= 0",
        "dp",
        "left_bound",
        "dp.rotation_direction_to(left_bound)",
        "0",
        "dp.rotation_direction_to(left_bound) >= 0",
        "end <= max_end\n            and right_bound.rotation_direction_to(dp) >= 0 \n            and dp.rotation_direction_to(left_bound) >= 0",
        "end <= max_end\n            and right_bound.rotation_direction_to(dp) >= 0 \n            and dp.rotation_direction_to(left_bound) >= 0",
        "dp",
        "dp.norm()",
        "let dp_norm = dp.norm()",
        "dp_norm",
        "r_max",
        "r",
        "r_max - r",
        "dp_norm < r_max - r",
        "dp_norm < r_max - r",
        "break",
        "dp_norm",
        "r_max",
        "dp_norm > r_max",
        "dp_norm > r_max",
        "r_max",
        "dp_norm",
        "r_max = dp_norm",
        "r_max = dp_norm",
        "if dp_norm < r_max - r:\n            break\n        elif dp_norm > r_max:\n            r_max = dp_norm",
        "dp_norm",
        "r",
        "dp_norm > r",
        "dp_norm > r",
        "go_right",
        "dp",
        "r",
        "go_right(dp, r)",
        "let dp_right = go_right(dp, r)",
        "go_left",
        "dp",
        "r",
        "go_left(dp, r)",
        "let dp_left = go_left(dp, r)",
        "right_bound",
        "dp_right",
        "right_bound.rotation_direction_to(dp_right)",
        "0",
        "right_bound.rotation_direction_to(dp_right) > 0",
        "right_bound.rotation_direction_to(dp_right) > 0",
        "right_bound",
        "dp_right",
        "right_bound = dp_right",
        "right_bound = dp_right",
        "if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right",
        "dp_left",
        "left_bound",
        "dp_left.rotation_direction_to(left_bound)",
        "0",
        "dp_left.rotation_direction_to(left_bound) > 0",
        "dp_left.rotation_direction_to(left_bound) > 0",
        "left_bound",
        "dp_left",
        "left_bound = dp_left",
        "left_bound = dp_left",
        "if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left",
        "if dp_norm > r:\n            let dp_right = go_right(dp, r)\n            let dp_left = go_left(dp, r)\n            if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right\n            if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left",
        "end",
        "end++",
        "end++",
        "dp",
        "ct",
        "start",
        "end",
        "1",
        "end + 1",
        "ct.displacement(start, end + 1)",
        "dp = ct.displacement(start, end + 1)",
        "dp = ct.displacement(start, end + 1)",
        "while end <= max_end\n            and right_bound.rotation_direction_to(dp) >= 0 \n            and dp.rotation_direction_to(left_bound) >= 0:\n        let dp_norm = dp.norm()\n        if dp_norm < r_max - r:\n            break\n        elif dp_norm > r_max:\n            r_max = dp_norm\n        if dp_norm > r:\n            let dp_right = go_right(dp, r)\n            let dp_left = go_left(dp, r)\n            if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right\n            if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left\n        end++\n        dp = ct.displacement(start, end + 1)",
        "end",
        "start",
        "end > start",
        "end > start",
        "assert end > start",
        "end",
        "return end",
        "let mut end = start\n    let mut dp = ct.displacement(start, end + 1)\n    let N = ct.points.ilen()\n    let max_end = start + N\n    while end <= max_end and dp.norm() < r:\n        end++\n        dp = ct.displacement(start, end + 1)\n    if dp.norm() < r:\n        return end\n    let mut right_bound = go_right(dp, r)\n    let mut left_bound = go_left(dp, r)\n    let mut r_max = 0.0\n    while end <= max_end\n            and right_bound.rotation_direction_to(dp) >= 0 \n            and dp.rotation_direction_to(left_bound) >= 0:\n        let dp_norm = dp.norm()\n        if dp_norm < r_max - r:\n            break\n        elif dp_norm > r_max:\n            r_max = dp_norm\n        if dp_norm > r:\n            let dp_right = go_right(dp, r)\n            let dp_left = go_left(dp, r)\n            if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right\n            if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left\n        end++\n        dp = ct.displacement(start, end + 1)\n    assert end > start\n    return end",
    ],
)
```

## `extend_start` decl

```rust
Some(
    [
        "RawContour",
        "~RawContour",
        "i32",
        "i32",
        "f32",
        "i32",
    ],
)
```

## `extend_start` defn

```rust
Some(
    [
        "end",
        "let mut start = end",
        "ct",
        "end",
        "start",
        "1",
        "start - 1",
        "ct.displacement(end, start - 1)",
        "let mut dp0 = ct.displacement(end, start - 1)",
        "end",
        "ct",
        "ct.points",
        "ct.points.ilen()",
        "end - ct.points.ilen()",
        "let min_start = end - ct.points.ilen()",
        "start",
        "min_start",
        "start >= min_start",
        "dp0",
        "dp0.norm()",
        "r",
        "dp0.norm() < r",
        "start >= min_start and dp0.norm() < r",
        "start >= min_start and dp0.norm() < r",
        "start",
        "start--",
        "start--",
        "dp0",
        "ct",
        "end",
        "start",
        "1",
        "start - 1",
        "ct.displacement(end, start - 1)",
        "dp0 = ct.displacement(end, start - 1)",
        "dp0 = ct.displacement(end, start - 1)",
        "while start >= min_start and dp0.norm() < r:\n        start--\n        dp0 = ct.displacement(end, start - 1)",
        "dp0",
        "dp0.norm()",
        "r",
        "dp0.norm() < r",
        "dp0.norm() < r",
        "start",
        "start0",
        "start.min(start0)",
        "return start.min(start0)",
        "if dp0.norm() < r:\n        return start.min(start0)",
        "go_right",
        "dp0",
        "r",
        "go_right(dp0, r)",
        "let mut right_bound = go_right(dp0, r)",
        "go_left",
        "dp0",
        "r",
        "go_left(dp0, r)",
        "let mut left_bound = go_left(dp0, r)",
        "0.0",
        "let mut r_max = 0.0",
        "start",
        "min_start",
        "start >= min_start",
        "start >= min_start",
        "ct",
        "end",
        "start",
        "1",
        "start - 1",
        "ct.displacement(end, start - 1)",
        "let dp = ct.displacement(end, start - 1)",
        "dp",
        "dp.norm()",
        "let dp_norm = dp.norm()",
        "dp_norm",
        "r_max",
        "r",
        "r_max - r",
        "dp_norm < r_max - r",
        "dp_norm < r_max - r",
        "break",
        "dp_norm",
        "r_max",
        "dp_norm > r_max",
        "dp_norm > r_max",
        "r_max",
        "dp_norm",
        "r_max = dp_norm",
        "r_max = dp_norm",
        "if dp_norm < r_max - r:\n            break\n        elif dp_norm > r_max:\n            r_max = dp_norm",
        "dp_norm",
        "r",
        "dp_norm > r",
        "dp_norm > r",
        "go_right",
        "dp",
        "r",
        "go_right(dp, r)",
        "let dp_right = go_right(dp, r)",
        "go_left",
        "dp",
        "r",
        "go_left(dp, r)",
        "let dp_left = go_left(dp, r)",
        "right_bound",
        "dp_right",
        "right_bound.rotation_direction_to(dp_right)",
        "0",
        "right_bound.rotation_direction_to(dp_right) > 0",
        "right_bound.rotation_direction_to(dp_right) > 0",
        "right_bound",
        "dp_right",
        "right_bound = dp_right",
        "right_bound = dp_right",
        "if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right",
        "dp_left",
        "left_bound",
        "dp_left.rotation_direction_to(left_bound)",
        "0",
        "dp_left.rotation_direction_to(left_bound) > 0",
        "dp_left.rotation_direction_to(left_bound) > 0",
        "left_bound",
        "dp_left",
        "left_bound = dp_left",
        "left_bound = dp_left",
        "if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left",
        "if dp_norm > r:\n            let dp_right = go_right(dp, r)\n            let dp_left = go_left(dp, r)\n            if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right\n            if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left",
        "right_bound",
        "left_bound",
        "right_bound.rotation_direction_to(left_bound)",
        "0",
        "right_bound.rotation_direction_to(left_bound) >= 0",
        "right_bound.rotation_direction_to(left_bound) >= 0",
        "start",
        "start0",
        "start <= start0",
        "right_bound",
        "dp",
        "right_bound.rotation_direction_to(dp)",
        "0",
        "right_bound.rotation_direction_to(dp) >= 0",
        "dp",
        "left_bound",
        "dp.rotation_direction_to(left_bound)",
        "0",
        "dp.rotation_direction_to(left_bound) >= 0",
        "right_bound.rotation_direction_to(dp) >= 0 \n                        and dp.rotation_direction_to(left_bound) >= 0",
        "(\n                        right_bound.rotation_direction_to(dp) >= 0 \n                        and dp.rotation_direction_to(left_bound) >= 0\n                    )",
        "!(\n                        right_bound.rotation_direction_to(dp) >= 0 \n                        and dp.rotation_direction_to(left_bound) >= 0\n                    )",
        "start <= start0 \n                    and !(\n                        right_bound.rotation_direction_to(dp) >= 0 \n                        and dp.rotation_direction_to(left_bound) >= 0\n                    )",
        "start <= start0 \n                    and !(\n                        right_bound.rotation_direction_to(dp) >= 0 \n                        and dp.rotation_direction_to(left_bound) >= 0\n                    )",
        "break",
        "if start <= start0 \n                    and !(\n                        right_bound.rotation_direction_to(dp) >= 0 \n                        and dp.rotation_direction_to(left_bound) >= 0\n                    ):\n                break",
        "start",
        "start--",
        "start--",
        "break",
        "if right_bound.rotation_direction_to(left_bound) >= 0:\n            if start <= start0 \n                    and !(\n                        right_bound.rotation_direction_to(dp) >= 0 \n                        and dp.rotation_direction_to(left_bound) >= 0\n                    ):\n                break\n            start--\n        else:\n            break",
        "while start >= min_start:\n        let dp = ct.displacement(end, start - 1)\n        let dp_norm = dp.norm()\n        if dp_norm < r_max - r:\n            break\n        elif dp_norm > r_max:\n            r_max = dp_norm\n        if dp_norm > r:\n            let dp_right = go_right(dp, r)\n            let dp_left = go_left(dp, r)\n            if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right\n            if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left\n        if right_bound.rotation_direction_to(left_bound) >= 0:\n            if start <= start0 \n                    and !(\n                        right_bound.rotation_direction_to(dp) >= 0 \n                        and dp.rotation_direction_to(left_bound) >= 0\n                    ):\n                break\n            start--\n        else:\n            break",
        "start",
        "start0",
        "start <= start0",
        "start <= start0",
        "start",
        "return start",
        "start0",
        "return start0",
        "if start <= start0:\n        return start\n    else:\n        return start0",
        "let mut start = end\n    let mut dp0 = ct.displacement(end, start - 1)\n    let min_start = end - ct.points.ilen()\n    while start >= min_start and dp0.norm() < r:\n        start--\n        dp0 = ct.displacement(end, start - 1)\n    if dp0.norm() < r:\n        return start.min(start0)\n    let mut right_bound = go_right(dp0, r)\n    let mut left_bound = go_left(dp0, r)\n    let mut r_max = 0.0\n    while start >= min_start:\n        let dp = ct.displacement(end, start - 1)\n        let dp_norm = dp.norm()\n        if dp_norm < r_max - r:\n            break\n        elif dp_norm > r_max:\n            r_max = dp_norm\n        if dp_norm > r:\n            let dp_right = go_right(dp, r)\n            let dp_left = go_left(dp, r)\n            if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right\n            if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left\n        if right_bound.rotation_direction_to(left_bound) >= 0:\n            if start <= start0 \n                    and !(\n                        right_bound.rotation_direction_to(dp) >= 0 \n                        and dp.rotation_direction_to(left_bound) >= 0\n                    ):\n                break\n            start--\n        else:\n            break\n    //assert start <= start0\n    if start <= start0:\n        return start\n    else:\n        return start0",
    ],
)
```

## `find_line_segments` decl

```rust
Some(
    [
        "RawContour",
        "~RawContour",
        "f32",
        "[",
        "LineSegmentStroke",
        "[]LineSegmentStroke",
    ],
)
```

## `find_line_segments` defn

```rust
Some(
    [
        "[]",
        "let mut line_segments: []LineSegmentStroke = []",
        "0",
        "let mut start = 0",
        "1",
        "let mut end = 1",
        "ct",
        "ct.points",
        "ct.points.ilen()",
        "let mut max_end = ct.points.ilen()",
        "end",
        "max_end",
        "end <= max_end",
        "end <= max_end",
        "end",
        "extend_end",
        "ct",
        "start",
        "r",
        "extend_end(ct, start, r)",
        "end = extend_end(ct, start, r)",
        "end = extend_end(ct, start, r)",
        "LineSegmentStroke::new",
        "ct",
        "start",
        "end",
        "LineSegmentStroke::new(ct, start, end)",
        "let ls_extend_end = LineSegmentStroke::new(ct, start, end)",
        "true",
        "let mut extend_start_flag = true",
        "line_segments",
        "line_segments.ilen()",
        "0",
        "line_segments.ilen() > 0",
        "line_segments.ilen() > 0",
        "ls_extend_end",
        "ls_extend_end.displacement()",
        "let dp_extend_end = ls_extend_end.displacement()",
        "line_segments",
        "line_segments.last()",
        "line_segments.last()!",
        "line_segments.last()!.displacement()",
        "let dp_previous = line_segments.last()!.displacement()",
        "dp_extend_end",
        "dp_previous",
        "dp_extend_end.cross(dp_previous)",
        "dp_extend_end.cross(dp_previous).abs()",
        "0.01",
        "dp_extend_end.cross(dp_previous).abs() < 0.01",
        "dp_extend_end",
        "dp_previous",
        "dp_extend_end.dot(dp_previous)",
        "0.0",
        "dp_extend_end.dot(dp_previous) > 0.0",
        "dp_extend_end.cross(dp_previous).abs() < 0.01 \n                    and dp_extend_end.dot(dp_previous) > 0.0",
        "dp_extend_end.cross(dp_previous).abs() < 0.01 \n                    and dp_extend_end.dot(dp_previous) > 0.0",
        "ct",
        "ct.points",
        "ct.points.ilen()",
        "let N = ct.points.ilen()",
        "line_segments",
        "line_segments.last()",
        "line_segments.last()!",
        "LineSegmentStroke::new",
        "ct",
        "line_segments",
        "line_segments.last()",
        "line_segments.last()!",
        "line_segments.last()!.points",
        "line_segments.last()!.points.start()",
        "end",
        "LineSegmentStroke::new(ct, line_segments.last()!.points.start(), end)",
        "line_segments.last()! = LineSegmentStroke::new(ct, line_segments.last()!.points.start(), end)",
        "line_segments.last()! = LineSegmentStroke::new(ct, line_segments.last()!.points.start(), end)",
        "extend_start_flag",
        "false",
        "extend_start_flag = false",
        "extend_start_flag = false",
        "if dp_extend_end.cross(dp_previous).abs() < 0.01 \n                    and dp_extend_end.dot(dp_previous) > 0.0:\n                let N = ct.points.ilen()\n                line_segments.last()! = LineSegmentStroke::new(ct, line_segments.last()!.points.start(), end)\n                extend_start_flag = false",
        "if line_segments.ilen() > 0:\n            let dp_extend_end = ls_extend_end.displacement()\n            let dp_previous = line_segments.last()!.displacement()\n            if dp_extend_end.cross(dp_previous).abs() < 0.01 \n                    and dp_extend_end.dot(dp_previous) > 0.0:\n                let N = ct.points.ilen()\n                line_segments.last()! = LineSegmentStroke::new(ct, line_segments.last()!.points.start(), end)\n                extend_start_flag = false",
        "extend_start_flag",
        "extend_start_flag",
        "start",
        "extend_start",
        "ct",
        "start",
        "end",
        "r",
        "extend_start(ct, start, end, r)",
        "start = extend_start(ct, start, end, r)",
        "start = extend_start(ct, start, end, r)",
        "LineSegmentStroke::new",
        "ct",
        "start",
        "end",
        "LineSegmentStroke::new(ct, start, end)",
        "let mut ls = LineSegmentStroke::new(ct, start, end)",
        "line_segments",
        "line_segments.ilen()",
        "0",
        "line_segments.ilen() > 0",
        "line_segments.ilen() > 0",
        "line_segments",
        "line_segments.last()",
        "line_segments.last()!",
        "let ls_last = line_segments.last()!",
        "ls_last",
        "ls_last.displacement()",
        "let dp_last = ls_last.displacement()",
        "ls",
        "ls.displacement()",
        "let dp = ls.displacement()",
        "ls_last",
        "ls_last.start",
        "ls",
        "ls.end",
        "ls_last.start.to(ls.end)",
        "let dp1 = ls_last.start.to(ls.end)",
        "dp",
        "dp_last",
        "dp.cross(dp_last)",
        "dp.cross(dp_last).abs()",
        "0.001",
        "dp.cross(dp_last).abs() < 0.001",
        "dp",
        "dp_last",
        "dp.dot(dp_last)",
        "0.0",
        "dp.dot(dp_last) > 0.0",
        "dp.cross(dp_last).abs() < 0.001 \n                        and dp.dot(dp_last) > 0.0",
        "dp",
        "dp1",
        "dp.cross(dp1)",
        "dp.cross(dp1).abs()",
        "0.001",
        "dp.cross(dp1).abs()<0.001",
        "dp.cross(dp_last).abs() < 0.001 \n                        and dp.dot(dp_last) > 0.0 \n                        and dp.cross(dp1).abs()<0.001",
        "dp",
        "dp1",
        "dp.dot(dp1)",
        "0.0",
        "dp.dot(dp1) > 0.0",
        "dp.cross(dp_last).abs() < 0.001 \n                        and dp.dot(dp_last) > 0.0 \n                        and dp.cross(dp1).abs()<0.001 \n                        and dp.dot(dp1) > 0.0",
        "dp.cross(dp_last).abs() < 0.001 \n                        and dp.dot(dp_last) > 0.0 \n                        and dp.cross(dp1).abs()<0.001 \n                        and dp.dot(dp1) > 0.0",
        "line_segments",
        "line_segments.pop()",
        "line_segments.pop()!",
        "let ls_last = line_segments.pop()!",
        "ls",
        "LineSegmentStroke::new",
        "ct",
        "ls_last",
        "ls_last.points",
        "ls_last.points.start()",
        "ls",
        "ls.points",
        "ls.points.end()",
        "LineSegmentStroke::new(ct, ls_last.points.start(), ls.points.end())",
        "ls = LineSegmentStroke::new(ct, ls_last.points.start(), ls.points.end())",
        "ls = LineSegmentStroke::new(ct, ls_last.points.start(), ls.points.end())",
        "if dp.cross(dp_last).abs() < 0.001 \n                        and dp.dot(dp_last) > 0.0 \n                        and dp.cross(dp1).abs()<0.001 \n                        and dp.dot(dp1) > 0.0:\n                    let ls_last = line_segments.pop()!\n                    ls = LineSegmentStroke::new(ct, ls_last.points.start(), ls.points.end())",
        "max_end",
        "start",
        "ct",
        "ct.points",
        "ct.points.ilen()",
        "start + ct.points.ilen()",
        "max_end = start + ct.points.ilen()",
        "max_end = start + ct.points.ilen()",
        "if line_segments.ilen() > 0:\n                let ls_last = line_segments.last()!\n                let dp_last = ls_last.displacement()\n                let dp = ls.displacement()\n                let dp1 = ls_last.start.to(ls.end)\n                if dp.cross(dp_last).abs() < 0.001 \n                        and dp.dot(dp_last) > 0.0 \n                        and dp.cross(dp1).abs()<0.001 \n                        and dp.dot(dp1) > 0.0:\n                    let ls_last = line_segments.pop()!\n                    ls = LineSegmentStroke::new(ct, ls_last.points.start(), ls.points.end())\n            else:\n                max_end = start + ct.points.ilen()",
        "line_segments",
        "ls",
        "line_segments.push(ls)",
        "line_segments.push(ls)",
        "if extend_start_flag:\n            start = extend_start(ct, start, end, r)\n            let mut ls = LineSegmentStroke::new(ct, start, end)\n            if line_segments.ilen() > 0:\n                let ls_last = line_segments.last()!\n                let dp_last = ls_last.displacement()\n                let dp = ls.displacement()\n                let dp1 = ls_last.start.to(ls.end)\n                if dp.cross(dp_last).abs() < 0.001 \n                        and dp.dot(dp_last) > 0.0 \n                        and dp.cross(dp1).abs()<0.001 \n                        and dp.dot(dp1) > 0.0:\n                    let ls_last = line_segments.pop()!\n                    ls = LineSegmentStroke::new(ct, ls_last.points.start(), ls.points.end())\n            else:\n                max_end = start + ct.points.ilen()\n            line_segments.push(ls)",
        "start",
        "end",
        "start = end",
        "start = end",
        "end",
        "start",
        "1",
        "start + 1",
        "end = start + 1",
        "end = start + 1",
        "while end <= max_end:\n        end = extend_end(ct, start, r)\n        let ls_extend_end = LineSegmentStroke::new(ct, start, end)\n        let mut extend_start_flag = true\n        if line_segments.ilen() > 0:\n            let dp_extend_end = ls_extend_end.displacement()\n            let dp_previous = line_segments.last()!.displacement()\n            if dp_extend_end.cross(dp_previous).abs() < 0.01 \n                    and dp_extend_end.dot(dp_previous) > 0.0:\n                let N = ct.points.ilen()\n                line_segments.last()! = LineSegmentStroke::new(ct, line_segments.last()!.points.start(), end)\n                extend_start_flag = false\n        if extend_start_flag:\n            start = extend_start(ct, start, end, r)\n            let mut ls = LineSegmentStroke::new(ct, start, end)\n            if line_segments.ilen() > 0:\n                let ls_last = line_segments.last()!\n                let dp_last = ls_last.displacement()\n                let dp = ls.displacement()\n                let dp1 = ls_last.start.to(ls.end)\n                if dp.cross(dp_last).abs() < 0.001 \n                        and dp.dot(dp_last) > 0.0 \n                        and dp.cross(dp1).abs()<0.001 \n                        and dp.dot(dp1) > 0.0:\n                    let ls_last = line_segments.pop()!\n                    ls = LineSegmentStroke::new(ct, ls_last.points.start(), ls.points.end())\n            else:\n                max_end = start + ct.points.ilen()\n            line_segments.push(ls)\n        start = end\n        end = start + 1",
        "ct",
        "ct.points",
        "ct.points.ilen()",
        "let N = ct.points.ilen()",
        "line_segments",
        "line_segments.first()",
        "line_segments.first()!",
        "line_segments.first()!.points",
        "line_segments.first()!.points.end()",
        "let first_line_segment_points_end = line_segments.first()!.points.end()",
        "line_segments",
        "line_segments.last()",
        "line_segments.last()!",
        "let last_line_segment = line_segments.last()!",
        "last_line_segment",
        "last_line_segment.points",
        "last_line_segment.points.end()",
        "first_line_segment_points_end",
        "N",
        "first_line_segment_points_end + N",
        "last_line_segment.points.end() >= first_line_segment_points_end + N",
        "last_line_segment.points.end() >= first_line_segment_points_end + N",
        "line_segments",
        "line_segments.pop()",
        "line_segments.pop()!",
        "let last_line_segment = line_segments.pop()!",
        "line_segments",
        "line_segments.first()",
        "line_segments.first()!",
        "LineSegmentStroke::new",
        "ct",
        "last_line_segment",
        "last_line_segment.points",
        "last_line_segment.points.start()",
        "N",
        "last_line_segment.points.start() - N",
        "line_segments",
        "line_segments.first()",
        "line_segments.first()!",
        "line_segments.first()!.points",
        "line_segments.first()!.points.end()",
        "1",
        "line_segments.first()!.points.end() - 1",
        "LineSegmentStroke::new(\n            ct,\n            last_line_segment.points.start() - N,\n            line_segments.first()!.points.end() - 1\n        )",
        "line_segments.first()! = LineSegmentStroke::new(\n            ct,\n            last_line_segment.points.start() - N,\n            line_segments.first()!.points.end() - 1\n        )",
        "line_segments.first()! = LineSegmentStroke::new(\n            ct,\n            last_line_segment.points.start() - N,\n            line_segments.first()!.points.end() - 1\n        )",
        "if last_line_segment.points.end() >= first_line_segment_points_end + N:\n        let last_line_segment = line_segments.pop()!\n        line_segments.first()! = LineSegmentStroke::new(\n            ct,\n            last_line_segment.points.start() - N,\n            line_segments.first()!.points.end() - 1\n        )",
        "line_segments",
        "line_segments",
        "let mut line_segments: []LineSegmentStroke = []\n    let mut start = 0\n    let mut end = 1\n    let mut max_end = ct.points.ilen()\n    while end <= max_end:\n        end = extend_end(ct, start, r)\n        let ls_extend_end = LineSegmentStroke::new(ct, start, end)\n        let mut extend_start_flag = true\n        if line_segments.ilen() > 0:\n            let dp_extend_end = ls_extend_end.displacement()\n            let dp_previous = line_segments.last()!.displacement()\n            if dp_extend_end.cross(dp_previous).abs() < 0.01 \n                    and dp_extend_end.dot(dp_previous) > 0.0:\n                let N = ct.points.ilen()\n                line_segments.last()! = LineSegmentStroke::new(ct, line_segments.last()!.points.start(), end)\n                extend_start_flag = false\n        if extend_start_flag:\n            start = extend_start(ct, start, end, r)\n            let mut ls = LineSegmentStroke::new(ct, start, end)\n            if line_segments.ilen() > 0:\n                let ls_last = line_segments.last()!\n                let dp_last = ls_last.displacement()\n                let dp = ls.displacement()\n                let dp1 = ls_last.start.to(ls.end)\n                if dp.cross(dp_last).abs() < 0.001 \n                        and dp.dot(dp_last) > 0.0 \n                        and dp.cross(dp1).abs()<0.001 \n                        and dp.dot(dp1) > 0.0:\n                    let ls_last = line_segments.pop()!\n                    ls = LineSegmentStroke::new(ct, ls_last.points.start(), ls.points.end())\n            else:\n                max_end = start + ct.points.ilen()\n            line_segments.push(ls)\n        start = end\n        end = start + 1\n    let N = ct.points.ilen()\n    let first_line_segment_points_end = line_segments.first()!.points.end()\n    let last_line_segment = line_segments.last()!\n    if last_line_segment.points.end() >= first_line_segment_points_end + N:\n        let last_line_segment = line_segments.pop()!\n        line_segments.first()! = LineSegmentStroke::new(\n            ct,\n            last_line_segment.points.start() - N,\n            line_segments.first()!.points.end() - 1\n        )\n    line_segments",
    ],
)
```

## `impl Visualize for LineSegmentStroke` decl

```rust
Some(
    [
        "Visualize",
        "LineSegmentStroke",
    ],
)
```

## `impl Visualize for LineSegmentStroke` defn

```rust
None
```

## `(LineSegmentStroke as Visualize)::visualize` decl

```rust
Some(
    [
        "Visual",
    ],
)
```

## `(LineSegmentStroke as Visualize)::visualize` defn

```rust
Some(
    [
        "self",
        "self.start",
        "self",
        "self.end",
        "<LineSegment start = {self.start} end = {self.end} />",
        "<LineSegment start = {self.start} end = {self.end} />",
        "<LineSegment start = {self.start} end = {self.end} />",
    ],
)
```

## `impl LineSegmentStroke` decl

```rust
Some(
    [
        "LineSegmentStroke",
    ],
)
```

## `impl LineSegmentStroke` defn

```rust
None
```

## `LineSegmentStroke::new` decl

```rust
Some(
    [
        "RawContour",
        "~RawContour",
        "i32",
        "i32",
        "LineSegmentStroke",
    ],
)
```

## `LineSegmentStroke::new` defn

```rust
Some(
    [
        "from",
        "to",
        "from <= to",
        "from <= to",
        "assert from <= to",
        "LineSegmentStroke",
        "ct",
        "ct.points",
        "from",
        "to",
        "1",
        "to + 1",
        "ct.points.cyclic_slice_leashed(from, to + 1)",
        "LineSegmentStroke(ct.points.cyclic_slice_leashed(from, to + 1))",
        "LineSegmentStroke(ct.points.cyclic_slice_leashed(from, to + 1))",
        "assert from <= to\n        LineSegmentStroke(ct.points.cyclic_slice_leashed(from, to + 1))",
    ],
)
```

## `LineSegmentStroke::displacement` decl

```rust
Some(
    [
        "Vector2d",
    ],
)
```

## `LineSegmentStroke::displacement` defn

```rust
Some(
    [
        "self",
        "self.start",
        "self",
        "self.end",
        "self.start.to(self.end)",
        "self.start.to(self.end)",
        "self.start.to(self.end)",
    ],
)
```

## `impl Visualize for LineSegmentSketch` decl

```rust
Some(
    [
        "Visualize",
        "LineSegmentSketch",
    ],
)
```

## `impl Visualize for LineSegmentSketch` defn

```rust
None
```

## `(LineSegmentSketch as Visualize)::visualize` decl

```rust
Some(
    [
        "Visual",
    ],
)
```

## `(LineSegmentSketch as Visualize)::visualize` defn

```rust
Some(
    [
        "self",
        "self.strokes",
        "self.strokes.visualize()",
        "self.strokes.visualize()",
        "self.strokes.visualize()",
    ],
)
```

## `impl LineSegmentSketch` decl

```rust
Some(
    [
        "LineSegmentSketch",
    ],
)
```

## `impl LineSegmentSketch` defn

```rust
None
```

## `LineSegmentSketch::concave_components` decl

```rust
Some(
    [
        "[",
        "ConcaveComponent",
        "[]ConcaveComponent",
    ],
)
```

## `LineSegmentSketch::concave_components` defn

```rust
Some(
    [
        "find_concave_components",
        "self",
        "find_concave_components(self)",
        "find_concave_components(self)",
        "find_concave_components(self)",
    ],
)
```

## `LineSegmentSketch::bounding_box` decl

```rust
Some(
    [
        "BoundingBox",
    ],
)
```

## `LineSegmentSketch::bounding_box` defn

```rust
Some(
    [
        "self",
        "self.strokes",
        "0",
        "self.strokes[0]",
        "self.strokes[0].start",
        "let start_point = self.strokes[0].start",
        "start_point",
        "start_point.x",
        "let mut xmin = start_point.x",
        "start_point",
        "start_point.x",
        "let mut xmax = start_point.x",
        "start_point",
        "start_point.y",
        "let mut ymin = start_point.y",
        "start_point",
        "start_point.y",
        "let mut ymax = start_point.y",
        "self",
        "self.strokes",
        "self.strokes.ilen()",
        "i",
        "self",
        "self.strokes",
        "i",
        "self.strokes[i]",
        "self.strokes[i].end",
        "let point = self.strokes[i].end",
        "xmin",
        "xmin",
        "point",
        "point.x",
        "xmin.min(point.x)",
        "xmin = xmin.min(point.x)",
        "xmin = xmin.min(point.x)",
        "xmax",
        "xmax",
        "point",
        "point.x",
        "xmax.max(point.x)",
        "xmax = xmax.max(point.x)",
        "xmax = xmax.max(point.x)",
        "ymin",
        "ymin",
        "point",
        "point.y",
        "ymin.min(point.y)",
        "ymin = ymin.min(point.y)",
        "ymin = ymin.min(point.y)",
        "ymax",
        "ymax",
        "point",
        "point.y",
        "ymax.max(point.y)",
        "ymax = ymax.max(point.y)",
        "ymax = ymax.max(point.y)",
        "for i < self.strokes.ilen():\n            let point = self.strokes[i].end\n            xmin = xmin.min(point.x)\n            xmax = xmax.max(point.x)\n            ymin = ymin.min(point.y)\n            ymax = ymax.max(point.y)",
        "BoundingBox",
        "ClosedRange",
        "xmin",
        "xmax",
        "ClosedRange(xmin, xmax)",
        "ClosedRange",
        "ymin",
        "ymax",
        "ClosedRange(ymin, ymax)",
        "BoundingBox(\n            ClosedRange(xmin, xmax), \n            ClosedRange(ymin, ymax),\n        )",
        "return BoundingBox(\n            ClosedRange(xmin, xmax), \n            ClosedRange(ymin, ymax),\n        )",
        "let start_point = self.strokes[0].start\n        let mut xmin = start_point.x\n        let mut xmax = start_point.x\n        let mut ymin = start_point.y\n        let mut ymax = start_point.y\n        for i < self.strokes.ilen():\n            let point = self.strokes[i].end\n            xmin = xmin.min(point.x)\n            xmax = xmax.max(point.x)\n            ymin = ymin.min(point.y)\n            ymax = ymax.max(point.y)\n        return BoundingBox(\n            ClosedRange(xmin, xmax), \n            ClosedRange(ymin, ymax),\n        )",
    ],
)
```

## `LineSegmentSketch::new` decl

```rust
Some(
    [
        "RawContour",
        "~RawContour",
        "f32",
        "LineSegmentSketch",
    ],
)
```

## `LineSegmentSketch::new` defn

```rust
Some(
    [
        "LineSegmentSketch",
        "ct",
        "find_line_segments",
        "ct",
        "r",
        "find_line_segments(ct, r)",
        "LineSegmentSketch(ct, find_line_segments(ct, r))",
        "LineSegmentSketch(ct, find_line_segments(ct, r))",
        "LineSegmentSketch(ct, find_line_segments(ct, r))",
    ],
)
```

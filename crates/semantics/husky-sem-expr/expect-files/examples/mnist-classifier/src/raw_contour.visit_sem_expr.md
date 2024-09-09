## `RawContour` decl

```rust
Some(
    [
        "ConnectedComponent",
        "~ConnectedComponent",
        "[",
        "Point2d",
        "[]Point2d",
    ],
)
```

## `RawContour` defn

```rust
None
```

## `Direction` decl

```rust
Some(
    [],
)
```

## `Direction` defn

```rust
None
```

## ``Direction`::#derive` decl

```rust
Some(
    [
        "Clone",
        "Copy",
        "PartialEq",
        "Eq",
    ],
)
```

## ``Direction`::#derive` defn

```rust
None
```

## `Direction::Up` decl

```rust
Some(
    [],
)
```

## `Direction::Up` defn

```rust
None
```

## `Direction::Left` decl

```rust
Some(
    [],
)
```

## `Direction::Left` defn

```rust
None
```

## `Direction::Down` decl

```rust
Some(
    [],
)
```

## `Direction::Down` defn

```rust
None
```

## `Direction::Right` decl

```rust
Some(
    [],
)
```

## `Direction::Right` defn

```rust
None
```

## `get_pixel_pair` decl

```rust
Some(
    [
        "r32",
        "i32",
        "r32",
    ],
)
```

## `get_pixel_pair` defn

```rust
Some(
    [
        "row",
        "j",
        "1",
        "j - 1",
        "(j - 1)",
        "row >> (j - 1)",
        "(row >> (j - 1))",
        "3r32",
        "(row >> (j - 1)) & 3r32",
        "(row >> (j - 1)) & 3r32",
        "(row >> (j - 1)) & 3r32",
    ],
)
```

## `get_pixel_to_the_left` decl

```rust
Some(
    [
        "r32",
        "i32",
        "r32",
    ],
)
```

## `get_pixel_to_the_left` defn

```rust
Some(
    [
        "row",
        "j",
        "row >> j",
        "(row >> j)",
        "1r32",
        "(row >> j) & 1r32",
        "(row >> j) & 1r32",
        "(row >> j) & 1r32",
    ],
)
```

## `get_pixel_to_the_right` decl

```rust
Some(
    [
        "r32",
        "i32",
        "r32",
    ],
)
```

## `get_pixel_to_the_right` defn

```rust
Some(
    [
        "row",
        "j",
        "1",
        "j - 1",
        "(j - 1)",
        "row >> (j - 1)",
        "(row >> (j - 1))",
        "1r32",
        "(row >> (j - 1)) & 1r32",
        "(row >> (j - 1)) & 1r32",
        "(row >> (j - 1)) & 1r32",
    ],
)
```

## `get_inward_direction` decl

```rust
Some(
    [
        "r32",
        "r32",
        "i32",
        "Direction",
    ],
)
```

## `get_inward_direction` defn

```rust
Some(
    [
        "get_pixel_pair",
        "row_above",
        "j",
        "get_pixel_pair(row_above, j)",
        "let pixel_pair_above = get_pixel_pair(row_above, j)",
        "get_pixel_pair",
        "row_below",
        "j",
        "get_pixel_pair(row_below, j)",
        "let pixel_pair_below = get_pixel_pair(row_below, j)",
        "pixel_pair_above",
        "pixel_pair_below",
        "Direction::Left",
        "Direction::Left",
        "Direction::Up",
        "Direction::Up",
        "unreachable",
        "unreachable",
        "match pixel_pair_below with\n        | 1\n        | 3 => Direction::Left\n        | 2 => Direction::Up\n        | _ => unreachable",
        "Direction::Down",
        "Direction::Down",
        "pixel_pair_below",
        "Direction::Right",
        "Direction::Right",
        "Direction::Left",
        "Direction::Left",
        "Direction::Up",
        "Direction::Up",
        "unreachable",
        "unreachable",
        "match pixel_pair_below with\n        | 0 => Direction::Right\n        | 1\n        | 3 => Direction::Left\n        | 2 => Direction::Up\n        | _ => unreachable",
        "pixel_pair_below",
        "Direction::Right",
        "Direction::Right",
        "Direction::Up",
        "Direction::Up",
        "unreachable",
        "unreachable",
        "match pixel_pair_below with\n        | 0\n        | 1 => Direction::Right\n        | 2 => Direction::Up\n        | _ => unreachable",
        "unreachable",
        "unreachable",
        "match pixel_pair_above with\n    | 0 =>\n        match pixel_pair_below with\n        | 1\n        | 3 => Direction::Left\n        | 2 => Direction::Up\n        | _ => unreachable\n    | 1 => Direction::Down\n    | 2 => \n        match pixel_pair_below with\n        | 0 => Direction::Right\n        | 1\n        | 3 => Direction::Left\n        | 2 => Direction::Up\n        | _ => unreachable\n    | 3 =>\n        match pixel_pair_below with\n        | 0\n        | 1 => Direction::Right\n        | 2 => Direction::Up\n        | _ => unreachable\n    | _ => unreachable",
        "let pixel_pair_above = get_pixel_pair(row_above, j)\n    let pixel_pair_below = get_pixel_pair(row_below, j)\n    match pixel_pair_above with\n    | 0 =>\n        match pixel_pair_below with\n        | 1\n        | 3 => Direction::Left\n        | 2 => Direction::Up\n        | _ => unreachable\n    | 1 => Direction::Down\n    | 2 => \n        match pixel_pair_below with\n        | 0 => Direction::Right\n        | 1\n        | 3 => Direction::Left\n        | 2 => Direction::Up\n        | _ => unreachable\n    | 3 =>\n        match pixel_pair_below with\n        | 0\n        | 1 => Direction::Right\n        | 2 => Direction::Up\n        | _ => unreachable\n    | _ => unreachable",
    ],
)
```

## `get_angle_change` decl

```rust
Some(
    [
        "Direction",
        "Direction",
        "i32",
    ],
)
```

## `get_angle_change` defn

```rust
Some(
    [
        "outward",
        "i32",
        "outward as i32",
        "(outward as i32)",
        "inward",
        "i32",
        "inward as i32",
        "(inward as i32)",
        "(outward as i32) - (inward as i32)",
        "((outward as i32) - (inward as i32))",
        "r32",
        "((outward as i32) - (inward as i32)) as r32",
        "(((outward as i32) - (inward as i32)) as r32)",
        "2",
        "(((outward as i32) - (inward as i32)) as r32).last_bits(2)",
        "let raw_angle_change = (((outward as i32) - (inward as i32)) as r32).last_bits(2)",
        "raw_angle_change",
        "raw_angle_change",
        "i32",
        "raw_angle_change as i32",
        "raw_angle_change as i32",
        "1",
        "-1",
        "-1",
        "unreachable",
        "unreachable",
        "match raw_angle_change with\n    | 0\n    | 1\n    | 2 => raw_angle_change as i32\n    | 3 => -1\n    | _ => unreachable",
        "let raw_angle_change = (((outward as i32) - (inward as i32)) as r32).last_bits(2)\n    match raw_angle_change with\n    | 0\n    | 1\n    | 2 => raw_angle_change as i32\n    | 3 => -1\n    | _ => unreachable",
    ],
)
```

## `get_outward_direction` decl

```rust
Some(
    [
        "r32",
        "r32",
        "i32",
        "Direction",
        "Direction",
    ],
)
```

## `get_outward_direction` defn

```rust
Some(
    [
        "get_pixel_pair",
        "row_above",
        "j",
        "get_pixel_pair(row_above, j)",
        "let pixel_pair_above = get_pixel_pair(row_above, j)",
        "get_pixel_pair",
        "row_below",
        "j",
        "get_pixel_pair(row_below, j)",
        "let pixel_pair_below = get_pixel_pair(row_below, j)",
        "pixel_pair_above",
        "pixel_pair_below",
        "Direction::Down",
        "Direction::Down",
        "Direction::Left",
        "Direction::Left",
        "unreachable",
        "unreachable",
        "match pixel_pair_below with\n        | 1 => Direction::Down\n        | 2\n        | 3 => Direction::Left\n        | _ => unreachable",
        "pixel_pair_below",
        "Direction::Right",
        "Direction::Right",
        "Direction::Down",
        "Direction::Down",
        "inward_direction",
        "Direction::Left",
        "Direction::Left",
        "Direction::Right",
        "Direction::Right",
        "unreachable",
        "unreachable",
        "match inward_direction with\n            | Direction::Down => Direction::Left\n            | Direction::Up => Direction::Right\n            | _ => unreachable",
        "Direction::Left",
        "Direction::Left",
        "unreachable",
        "unreachable",
        "match pixel_pair_below with\n        | 0 => Direction::Right\n        | 1 => Direction::Down\n        | 2 =>\n            match inward_direction with\n            | Direction::Down => Direction::Left\n            | Direction::Up => Direction::Right\n            | _ => unreachable\n        | 3 => Direction::Left\n        | _ => unreachable",
        "pixel_pair_below",
        "Direction::Up",
        "Direction::Up",
        "inward_direction",
        "Direction::Up",
        "Direction::Up",
        "Direction::Down",
        "Direction::Down",
        "unreachable",
        "unreachable",
        "match inward_direction with\n            | Direction::Left => Direction::Up\n            | Direction::Right => Direction::Down\n            | _ => unreachable",
        "unreachable",
        "unreachable",
        "match pixel_pair_below with\n        | 0\n        | 2\n        | 3 => Direction::Up\n        | 1 =>\n            match inward_direction with\n            | Direction::Left => Direction::Up\n            | Direction::Right => Direction::Down\n            | _ => unreachable\n        | _ => unreachable",
        "pixel_pair_below",
        "Direction::Right",
        "Direction::Right",
        "Direction::Down",
        "Direction::Down",
        "unreachable",
        "unreachable",
        "match pixel_pair_below with\n        | 0\n        | 2 => Direction::Right\n        | 1 => Direction::Down\n        | _ => unreachable",
        "unreachable",
        "unreachable",
        "match pixel_pair_above with\n    | 0 =>\n        match pixel_pair_below with\n        | 1 => Direction::Down\n        | 2\n        | 3 => Direction::Left\n        | _ => unreachable\n    | 1 =>\n        match pixel_pair_below with\n        | 0 => Direction::Right\n        | 1 => Direction::Down\n        | 2 =>\n            match inward_direction with\n            | Direction::Down => Direction::Left\n            | Direction::Up => Direction::Right\n            | _ => unreachable\n        | 3 => Direction::Left\n        | _ => unreachable\n    | 2 =>\n        match pixel_pair_below with\n        | 0\n        | 2\n        | 3 => Direction::Up\n        | 1 =>\n            match inward_direction with\n            | Direction::Left => Direction::Up\n            | Direction::Right => Direction::Down\n            | _ => unreachable\n        | _ => unreachable\n    | 3 =>\n        match pixel_pair_below with\n        | 0\n        | 2 => Direction::Right\n        | 1 => Direction::Down\n        | _ => unreachable\n    | _ => unreachable",
        "let pixel_pair_above = get_pixel_pair(row_above, j)\n    let pixel_pair_below = get_pixel_pair(row_below, j)\n    match pixel_pair_above with\n    | 0 =>\n        match pixel_pair_below with\n        | 1 => Direction::Down\n        | 2\n        | 3 => Direction::Left\n        | _ => unreachable\n    | 1 =>\n        match pixel_pair_below with\n        | 0 => Direction::Right\n        | 1 => Direction::Down\n        | 2 =>\n            match inward_direction with\n            | Direction::Down => Direction::Left\n            | Direction::Up => Direction::Right\n            | _ => unreachable\n        | 3 => Direction::Left\n        | _ => unreachable\n    | 2 =>\n        match pixel_pair_below with\n        | 0\n        | 2\n        | 3 => Direction::Up\n        | 1 =>\n            match inward_direction with\n            | Direction::Left => Direction::Up\n            | Direction::Right => Direction::Down\n            | _ => unreachable\n        | _ => unreachable\n    | 3 =>\n        match pixel_pair_below with\n        | 0\n        | 2 => Direction::Right\n        | 1 => Direction::Down\n        | _ => unreachable\n    | _ => unreachable",
    ],
)
```

## `StreakCache` decl

```rust
Some(
    [
        "i32",
        "i32",
    ],
)
```

## `StreakCache` defn

```rust
None
```

## `get_concave_middle_point` decl

```rust
Some(
    [
        "[",
        "Point2d",
        "[]Point2d",
        "Point2d",
    ],
)
```

## `get_concave_middle_point` defn

```rust
Some(
    [
        "points",
        "points.ilen()",
        "let N = points.ilen()",
        "points",
        "N",
        "2",
        "N-2",
        "points[N-2]",
        "let p0 = points[N-2]",
        "points",
        "N",
        "1",
        "N-1",
        "points[N-1]",
        "let p2 = points[N-1]",
        "Point2d",
        "p0",
        "p0.x",
        "p2",
        "p2.x",
        "p0.x + p2.x",
        "(p0.x + p2.x)",
        "2.0",
        "(p0.x + p2.x) / 2.0",
        "p0",
        "p0.y",
        "p2",
        "p2.y",
        "p0.y + p2.y",
        "(p0.y + p2.y)",
        "2.0",
        "(p0.y + p2.y) / 2.0",
        "Point2d(\n        (p0.x + p2.x) / 2.0, \n        (p0.y + p2.y) / 2.0,\n    )",
        "Point2d(\n        (p0.x + p2.x) / 2.0, \n        (p0.y + p2.y) / 2.0,\n    )",
        "let N = points.ilen()\n    let p0 = points[N-2]\n    let p2 = points[N-1]\n    Point2d(\n        (p0.x + p2.x) / 2.0, \n        (p0.y + p2.y) / 2.0,\n    )",
    ],
)
```

## `find_raw_contours` decl

```rust
Some(
    [
        "ConnectedComponent",
        "~ConnectedComponent",
        "[",
        "RawContour",
        "[]RawContour",
    ],
)
```

## `find_raw_contours` defn

```rust
Some(
    [
        "[]",
        "let mut result: []RawContour = []",
        "BinaryGrid28::new_zeros",
        "BinaryGrid28::new_zeros()",
        "let mut boundary_unsearched = BinaryGrid28::new_zeros()",
        "1",
        "29",
        "i",
        "cc",
        "cc.mask",
        "i",
        "1",
        "i-1",
        "cc.mask[i-1]",
        "let r_ur = cc.mask[i-1]",
        "cc",
        "cc.mask",
        "i",
        "cc.mask[i]",
        "let r_dr = cc.mask[i]",
        "r_ur",
        "1",
        "r_ur << 1",
        "let r_ul = r_ur << 1",
        "r_dr",
        "1",
        "r_dr << 1",
        "let r_dl = r_dr << 1",
        "boundary_unsearched",
        "i",
        "boundary_unsearched[i]",
        "r_ur",
        "r_dr",
        "r_ur|r_dr",
        "r_ul",
        "r_ur|r_dr|r_ul",
        "r_dl",
        "r_ur|r_dr|r_ul|r_dl",
        "(r_ur|r_dr|r_ul|r_dl)",
        "r_ur",
        "r_dr",
        "r_ur&r_dr",
        "r_ul",
        "r_ur&r_dr&r_ul",
        "r_dl",
        "r_ur&r_dr&r_ul&r_dl",
        "(r_ur&r_dr&r_ul&r_dl)",
        "~(r_ur&r_dr&r_ul&r_dl)",
        "(~(r_ur&r_dr&r_ul&r_dl))",
        "(r_ur|r_dr|r_ul|r_dl) & (~(r_ur&r_dr&r_ul&r_dl))",
        "boundary_unsearched[i] = (r_ur|r_dr|r_ul|r_dl) & (~(r_ur&r_dr&r_ul&r_dl))",
        "boundary_unsearched[i] = (r_ur|r_dr|r_ul|r_dl) & (~(r_ur&r_dr&r_ul&r_dl))",
        "for 1 <= i <= 29:\n        let r_ur = cc.mask[i-1]\n        let r_dr = cc.mask[i]\n        let r_ul = r_ur << 1\n        let r_dl = r_dr << 1\n        boundary_unsearched[i] = (r_ur|r_dr|r_ul|r_dl) & (~(r_ur&r_dr&r_ul&r_dl))",
        "1",
        "29",
        "k",
        "boundary_unsearched",
        "k",
        "boundary_unsearched[k]",
        "boundary_unsearched[k]",
        "[]",
        "let mut contour: []Point2d = []",
        "k",
        "let mut i = k",
        "boundary_unsearched",
        "k",
        "boundary_unsearched[k]",
        "boundary_unsearched[k].ctz()",
        "let mut j = boundary_unsearched[k].ctz()",
        "cc",
        "cc.mask",
        "i",
        "1",
        "i-1",
        "cc.mask[i-1]",
        "let mut row_above = cc.mask[i-1]",
        "cc",
        "cc.mask",
        "i",
        "cc.mask[i]",
        "let mut row_below = cc.mask[i]",
        "get_inward_direction",
        "row_above",
        "row_below",
        "j",
        "get_inward_direction(row_above, row_below, j)",
        "let mut inward_direction = get_inward_direction(row_above, row_below, j)",
        "i",
        "let i0 = i",
        "j",
        "let j0 = j",
        "inward_direction",
        "let dir0 = inward_direction",
        "0",
        "let mut prev_angle_change1 = 0",
        "0",
        "let mut prev_angle_change2 = 0",
        "0",
        "let mut total_angle_change = 0",
        "1",
        "-1",
        "let mut prev_streak1 = -1",
        "1",
        "-1",
        "let mut prev_streak2 = -1",
        "1",
        "-1",
        "let mut current_streak = -1",
        "get_outward_direction",
        "row_above",
        "row_below",
        "j",
        "inward_direction",
        "get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )",
        "let outward_direction = get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )",
        "get_angle_change",
        "inward_direction",
        "outward_direction",
        "get_angle_change(inward_direction, outward_direction)",
        "let angle_change = get_angle_change(inward_direction, outward_direction)",
        "boundary_unsearched",
        "i",
        "boundary_unsearched[i]",
        "boundary_unsearched",
        "i",
        "boundary_unsearched[i]",
        "1r32",
        "j",
        "1r32 << j",
        "(1r32 << j)",
        "~(1r32 << j)",
        "(~(1r32 << j))",
        "boundary_unsearched[i] & (~(1r32 << j))",
        "boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))",
        "boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))",
        "angle_change",
        "angle_change",
        "prev_angle_change1",
        "1",
        "-1",
        "prev_angle_change1 == -1",
        "prev_angle_change2",
        "1",
        "-1",
        "prev_angle_change2 == -1",
        "prev_angle_change1 == -1\n                            and prev_angle_change2 == -1",
        "current_streak",
        "1",
        "current_streak == 1",
        "prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1",
        "prev_streak1",
        "1",
        "-1",
        "prev_streak1 != -1",
        "prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1",
        "prev_streak2",
        "1",
        "prev_streak2 == 1",
        "prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1",
        "prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1",
        "contour",
        "contour.last()",
        "contour.last()!",
        "get_concave_middle_point",
        "contour",
        "get_concave_middle_point(contour)",
        "contour.last()! = get_concave_middle_point(contour)",
        "contour.last()! = get_concave_middle_point(contour)",
        "contour",
        "Point2d::from_i_shift28",
        "i",
        "j",
        "Point2d::from_i_shift28(i, j)",
        "contour.push(Point2d::from_i_shift28(i, j))",
        "contour.push(Point2d::from_i_shift28(i, j))",
        "prev_streak2",
        "1",
        "-1",
        "prev_streak2 = -1",
        "prev_streak2 = -1",
        "prev_streak1",
        "1",
        "-1",
        "prev_streak1 = -1",
        "prev_streak1 = -1",
        "prev_angle_change1",
        "1",
        "-1",
        "prev_angle_change1 == -1",
        "prev_streak1",
        "0",
        "prev_streak1 > 0",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0",
        "prev_streak1",
        "1",
        "prev_streak1 == 1",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1",
        "contour",
        "contour.last()",
        "contour.last()!",
        "Point2d::from_i_shift28",
        "i",
        "j",
        "Point2d::from_i_shift28(i, j)",
        "contour.last()! = Point2d::from_i_shift28(i, j)",
        "contour.last()! = Point2d::from_i_shift28(i, j)",
        "prev_streak2",
        "prev_streak1",
        "prev_streak2 = prev_streak1",
        "prev_streak2 = prev_streak1",
        "prev_streak1",
        "current_streak",
        "prev_streak1 = current_streak",
        "prev_streak1 = current_streak",
        "prev_angle_change1",
        "1",
        "-1",
        "prev_angle_change1 == -1",
        "prev_streak1",
        "0",
        "prev_streak1 > 0",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0",
        "current_streak",
        "1",
        "current_streak == 1",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1",
        "prev_streak1",
        "1",
        "prev_streak1 > 1",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1",
        "contour",
        "contour.last()",
        "contour.last()!",
        "Point2d::from_i_shift28",
        "i",
        "j",
        "Point2d::from_i_shift28(i, j)",
        "contour.last()! = Point2d::from_i_shift28(i, j)",
        "contour.last()! = Point2d::from_i_shift28(i, j)",
        "prev_streak2",
        "1",
        "-1",
        "prev_streak2 = -1",
        "prev_streak2 = -1",
        "prev_streak1",
        "1",
        "-1",
        "prev_streak1 = -1",
        "prev_streak1 = -1",
        "contour",
        "Point2d::from_i_shift28",
        "i",
        "j",
        "Point2d::from_i_shift28(i, j)",
        "contour.push(Point2d::from_i_shift28(i, j))",
        "contour.push(Point2d::from_i_shift28(i, j))",
        "prev_streak2",
        "prev_streak1",
        "prev_streak2 = prev_streak1",
        "prev_streak2 = prev_streak1",
        "prev_streak1",
        "current_streak",
        "prev_streak1 = current_streak",
        "prev_streak1 = current_streak",
        "if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak",
        "current_streak",
        "0",
        "current_streak = 0",
        "current_streak = 0",
        "prev_angle_change2",
        "prev_angle_change1",
        "prev_angle_change2 = prev_angle_change1",
        "prev_angle_change2 = prev_angle_change1",
        "prev_angle_change1",
        "angle_change",
        "prev_angle_change1 = angle_change",
        "prev_angle_change1 = angle_change",
        "if angle_change:\n                    // update contour and previous streaks\n                    if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    // reset current streak\n                    current_streak = 0\n                    // update previous angle change\n                    prev_angle_change2 = prev_angle_change1\n                    prev_angle_change1 = angle_change",
        "outward_direction",
        "i",
        "i",
        "1",
        "i - 1",
        "i = i - 1",
        "i = i - 1",
        "row_below",
        "row_above",
        "row_below = row_above",
        "row_below = row_above",
        "row_above",
        "cc",
        "cc.mask",
        "i",
        "1",
        "i-1",
        "cc.mask[i-1]",
        "row_above = cc.mask[i-1]",
        "row_above = cc.mask[i-1]",
        "i",
        "i",
        "1",
        "i + 1",
        "i = i + 1",
        "i = i + 1",
        "row_above",
        "row_below",
        "row_above = row_below",
        "row_above = row_below",
        "row_below",
        "cc",
        "cc.mask",
        "i",
        "cc.mask[i]",
        "row_below = cc.mask[i]",
        "row_below = cc.mask[i]",
        "j",
        "j",
        "1",
        "j + 1",
        "j = j + 1",
        "j = j + 1",
        "j",
        "j",
        "1",
        "j - 1",
        "j = j - 1",
        "j = j - 1",
        "match outward_direction with\n                | Direction::Up =>\n                    i = i - 1 \n                    row_below = row_above\n                    row_above = cc.mask[i-1]\n                | Direction::Down =>\n                    i = i + 1 \n                    row_above = row_below\n                    row_below = cc.mask[i]\n                | Direction::Left => j = j + 1\n                | Direction::Right => j = j - 1",
        "inward_direction",
        "outward_direction",
        "inward_direction = outward_direction",
        "inward_direction = outward_direction",
        "current_streak",
        "1",
        "-1",
        "current_streak != -1",
        "current_streak != -1",
        "current_streak",
        "current_streak++",
        "current_streak++",
        "if current_streak != -1:\n                    current_streak++",
        "i",
        "i0",
        "i == i0",
        "j",
        "j0",
        "j == j0",
        "i == i0 and j == j0",
        "inward_direction",
        "dir0",
        "inward_direction == dir0",
        "i == i0 and j == j0 and inward_direction == dir0",
        "(i == i0 and j == j0 and inward_direction == dir0)",
        "!(i == i0 and j == j0 and inward_direction == dir0)",
        "!(i == i0 and j == j0 and inward_direction == dir0)",
        "do while !(i == i0 and j == j0 and inward_direction == dir0):\n                let outward_direction = get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )\n                let angle_change = get_angle_change(inward_direction, outward_direction)\n                // update boundary_unsearched\n                boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))\n                if angle_change:\n                    // update contour and previous streaks\n                    if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    // reset current streak\n                    current_streak = 0\n                    // update previous angle change\n                    prev_angle_change2 = prev_angle_change1\n                    prev_angle_change1 = angle_change\n                // move\n                // update i, j, row below and above\n                match outward_direction with\n                | Direction::Up =>\n                    i = i - 1 \n                    row_below = row_above\n                    row_above = cc.mask[i-1]\n                | Direction::Down =>\n                    i = i + 1 \n                    row_above = row_below\n                    row_below = cc.mask[i]\n                | Direction::Left => j = j + 1\n                | Direction::Right => j = j - 1\n                // update inward_direction\n                inward_direction = outward_direction\n                // update streak\n                if current_streak != -1:\n                    current_streak++",
        "prev_angle_change1",
        "1",
        "-1",
        "prev_angle_change1 == -1",
        "current_streak",
        "1",
        "current_streak == 1",
        "prev_angle_change1 == -1 and current_streak == 1",
        "prev_streak1",
        "0",
        "prev_streak1 > 0",
        "prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0",
        "prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0",
        "contour",
        "contour.pop()",
        "contour.pop()",
        "if prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0:\n                contour.pop()",
        "result",
        "RawContour",
        "cc",
        "contour",
        "RawContour(cc, contour)",
        "result.push(RawContour(cc, contour))",
        "result.push(RawContour(cc, contour))",
        "while boundary_unsearched[k]:\n            let mut contour: []Point2d = []\n            let mut i = k\n            let mut j = boundary_unsearched[k].ctz()\n            // prepare rows\n            let mut row_above = cc.mask[i-1]\n            let mut row_below = cc.mask[i]\n            // prepare pixel_pairs and initial inward direction\n            let mut inward_direction = get_inward_direction(row_above, row_below, j)\n            // store initial position and direction\n            let i0 = i\n            let j0 = j\n            let dir0 = inward_direction\n            let mut prev_angle_change1 = 0\n            let mut prev_angle_change2 = 0\n            let mut total_angle_change = 0\n            // prepare streaks (raw line segment lengths)\n            // -1 means invalid\n            let mut prev_streak1 = -1\n            let mut prev_streak2 = -1\n            let mut current_streak = -1\n            // loop in the geometric sense!\n            do while !(i == i0 and j == j0 and inward_direction == dir0):\n                let outward_direction = get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )\n                let angle_change = get_angle_change(inward_direction, outward_direction)\n                // update boundary_unsearched\n                boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))\n                if angle_change:\n                    // update contour and previous streaks\n                    if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    // reset current streak\n                    current_streak = 0\n                    // update previous angle change\n                    prev_angle_change2 = prev_angle_change1\n                    prev_angle_change1 = angle_change\n                // move\n                // update i, j, row below and above\n                match outward_direction with\n                | Direction::Up =>\n                    i = i - 1 \n                    row_below = row_above\n                    row_above = cc.mask[i-1]\n                | Direction::Down =>\n                    i = i + 1 \n                    row_above = row_below\n                    row_below = cc.mask[i]\n                | Direction::Left => j = j + 1\n                | Direction::Right => j = j - 1\n                // update inward_direction\n                inward_direction = outward_direction\n                // update streak\n                if current_streak != -1:\n                    current_streak++\n            if prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0:\n                contour.pop();\n            result.push(RawContour(cc, contour))",
        "for 1 <= k <= 29:\n        while boundary_unsearched[k]:\n            let mut contour: []Point2d = []\n            let mut i = k\n            let mut j = boundary_unsearched[k].ctz()\n            // prepare rows\n            let mut row_above = cc.mask[i-1]\n            let mut row_below = cc.mask[i]\n            // prepare pixel_pairs and initial inward direction\n            let mut inward_direction = get_inward_direction(row_above, row_below, j)\n            // store initial position and direction\n            let i0 = i\n            let j0 = j\n            let dir0 = inward_direction\n            let mut prev_angle_change1 = 0\n            let mut prev_angle_change2 = 0\n            let mut total_angle_change = 0\n            // prepare streaks (raw line segment lengths)\n            // -1 means invalid\n            let mut prev_streak1 = -1\n            let mut prev_streak2 = -1\n            let mut current_streak = -1\n            // loop in the geometric sense!\n            do while !(i == i0 and j == j0 and inward_direction == dir0):\n                let outward_direction = get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )\n                let angle_change = get_angle_change(inward_direction, outward_direction)\n                // update boundary_unsearched\n                boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))\n                if angle_change:\n                    // update contour and previous streaks\n                    if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    // reset current streak\n                    current_streak = 0\n                    // update previous angle change\n                    prev_angle_change2 = prev_angle_change1\n                    prev_angle_change1 = angle_change\n                // move\n                // update i, j, row below and above\n                match outward_direction with\n                | Direction::Up =>\n                    i = i - 1 \n                    row_below = row_above\n                    row_above = cc.mask[i-1]\n                | Direction::Down =>\n                    i = i + 1 \n                    row_above = row_below\n                    row_below = cc.mask[i]\n                | Direction::Left => j = j + 1\n                | Direction::Right => j = j - 1\n                // update inward_direction\n                inward_direction = outward_direction\n                // update streak\n                if current_streak != -1:\n                    current_streak++\n            if prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0:\n                contour.pop();\n            result.push(RawContour(cc, contour))",
        "result",
        "return result",
        "let mut result: []RawContour = []\n    let mut boundary_unsearched = BinaryGrid28::new_zeros()\n    for 1 <= i <= 29:\n        let r_ur = cc.mask[i-1]\n        let r_dr = cc.mask[i]\n        let r_ul = r_ur << 1\n        let r_dl = r_dr << 1\n        boundary_unsearched[i] = (r_ur|r_dr|r_ul|r_dl) & (~(r_ur&r_dr&r_ul&r_dl))\n    for 1 <= k <= 29:\n        while boundary_unsearched[k]:\n            let mut contour: []Point2d = []\n            let mut i = k\n            let mut j = boundary_unsearched[k].ctz()\n            // prepare rows\n            let mut row_above = cc.mask[i-1]\n            let mut row_below = cc.mask[i]\n            // prepare pixel_pairs and initial inward direction\n            let mut inward_direction = get_inward_direction(row_above, row_below, j)\n            // store initial position and direction\n            let i0 = i\n            let j0 = j\n            let dir0 = inward_direction\n            let mut prev_angle_change1 = 0\n            let mut prev_angle_change2 = 0\n            let mut total_angle_change = 0\n            // prepare streaks (raw line segment lengths)\n            // -1 means invalid\n            let mut prev_streak1 = -1\n            let mut prev_streak2 = -1\n            let mut current_streak = -1\n            // loop in the geometric sense!\n            do while !(i == i0 and j == j0 and inward_direction == dir0):\n                let outward_direction = get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )\n                let angle_change = get_angle_change(inward_direction, outward_direction)\n                // update boundary_unsearched\n                boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))\n                if angle_change:\n                    // update contour and previous streaks\n                    if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    // reset current streak\n                    current_streak = 0\n                    // update previous angle change\n                    prev_angle_change2 = prev_angle_change1\n                    prev_angle_change1 = angle_change\n                // move\n                // update i, j, row below and above\n                match outward_direction with\n                | Direction::Up =>\n                    i = i - 1 \n                    row_below = row_above\n                    row_above = cc.mask[i-1]\n                | Direction::Down =>\n                    i = i + 1 \n                    row_above = row_below\n                    row_below = cc.mask[i]\n                | Direction::Left => j = j + 1\n                | Direction::Right => j = j - 1\n                // update inward_direction\n                inward_direction = outward_direction\n                // update streak\n                if current_streak != -1:\n                    current_streak++\n            if prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0:\n                contour.pop();\n            result.push(RawContour(cc, contour))\n    return result",
    ],
)
```

## `impl Visualize for RawContour` decl

```rust
Some(
    [
        "Visualize",
        "RawContour",
    ],
)
```

## `impl Visualize for RawContour` defn

```rust
None
```

## `(RawContour as Visualize)::visualize` decl

```rust
Some(
    [
        "Visual",
    ],
)
```

## `(RawContour as Visualize)::visualize` defn

```rust
Some(
    [
        "self",
        "self.points",
        "<Contour points={self.points} />",
        "<Contour points={self.points} />",
        "<Contour points={self.points} />",
    ],
)
```

## `impl RawContour` decl

```rust
Some(
    [
        "RawContour",
    ],
)
```

## `impl RawContour` defn

```rust
None
```

## `RawContour::line_segment_sketch` decl

```rust
Some(
    [
        "LineSegmentSketch",
    ],
)
```

## `RawContour::line_segment_sketch` defn

```rust
Some(
    [
        "LineSegmentSketch::new",
        "self",
        "1.4",
        "LineSegmentSketch::new(self, 1.4)",
        "LineSegmentSketch::new(self, 1.4)",
        "LineSegmentSketch::new(self, 1.4)",
    ],
)
```

## `RawContour::bounding_box` decl

```rust
Some(
    [
        "BoundingBox",
    ],
)
```

## `RawContour::bounding_box` defn

```rust
Some(
    [
        "self",
        "self.points",
        "0",
        "self.points[0]",
        "let start_point = self.points[0]",
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
        "self.points",
        "self.points.ilen()",
        "i",
        "self",
        "self.points",
        "i",
        "self.points[i]",
        "let point = self.points[i]",
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
        "for i < self.points.ilen():\n            let point = self.points[i]\n            xmin = xmin.min(point.x)\n            xmax = xmax.max(point.x)\n            ymin = ymin.min(point.y)\n            ymax = ymax.max(point.y)",
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
        "let start_point = self.points[0]\n        let mut xmin = start_point.x\n        let mut xmax = start_point.x\n        let mut ymin = start_point.y\n        let mut ymax = start_point.y\n        for i < self.points.ilen():\n            let point = self.points[i]\n            xmin = xmin.min(point.x)\n            xmax = xmax.max(point.x)\n            ymin = ymin.min(point.y)\n            ymax = ymax.max(point.y)\n        return BoundingBox(\n            ClosedRange(xmin, xmax), \n            ClosedRange(ymin, ymax),\n        )",
    ],
)
```

## `RawContour::relative_bounding_box` decl

```rust
Some(
    [
        "RelativeBoundingBox",
    ],
)
```

## `RawContour::relative_bounding_box` defn

```rust
Some(
    [
        "self",
        "self.cc",
        "self.cc.raw_contours",
        "0",
        "self.cc.raw_contours[0]",
        "self.cc.raw_contours[0].bounding_box",
        "self",
        "self.bounding_box",
        "self.cc.raw_contours[0].bounding_box.relative_bounding_box(self.bounding_box)",
        "self.cc.raw_contours[0].bounding_box.relative_bounding_box(self.bounding_box)",
        "self.cc.raw_contours[0].bounding_box.relative_bounding_box(self.bounding_box)",
    ],
)
```

## `RawContour::contour_len` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `RawContour::contour_len` defn

```rust
Some(
    [
        "0.0",
        "let mut contour_len = 0.0",
        "0",
        "self",
        "self.points",
        "self.points.ilen()",
        "i",
        "self",
        "self.points",
        "i",
        "1",
        "i-1",
        "self.points[i-1]",
        "let a = self.points[i-1]",
        "self",
        "self.points",
        "i",
        "self.points[i]",
        "let b = self.points[i]",
        "contour_len",
        "a",
        "a.x",
        "b",
        "b.x",
        "a.x - b.x",
        "(a.x - b.x)",
        "(a.x - b.x).abs()",
        "a",
        "a.y",
        "b",
        "b.y",
        "a.y - b.y",
        "(a.y - b.y)",
        "(a.y - b.y).abs()",
        "(a.x - b.x).abs() + (a.y - b.y).abs()",
        "contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()",
        "contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()",
        "for 0 < i < self.points.ilen():\n            let a = self.points[i-1]\n            let b = self.points[i]\n            contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()",
        "self",
        "self.points",
        "self",
        "self.points",
        "self.points.ilen()",
        "1",
        "self.points.ilen() - 1",
        "self.points[self.points.ilen() - 1]",
        "let a = self.points[self.points.ilen() - 1]",
        "self",
        "self.points",
        "0",
        "self.points[0]",
        "let b = self.points[0]",
        "contour_len",
        "a",
        "a.x",
        "b",
        "b.x",
        "a.x - b.x",
        "(a.x - b.x)",
        "(a.x - b.x).abs()",
        "a",
        "a.y",
        "b",
        "b.y",
        "a.y - b.y",
        "(a.y - b.y)",
        "(a.y - b.y).abs()",
        "(a.x - b.x).abs() + (a.y - b.y).abs()",
        "contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()",
        "contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()",
        "contour_len",
        "return contour_len",
        "let mut contour_len = 0.0\n        for 0 < i < self.points.ilen():\n            let a = self.points[i-1]\n            let b = self.points[i]\n            contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()\n        let a = self.points[self.points.ilen() - 1]\n        let b = self.points[0]\n        contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()\n        return contour_len",
    ],
)
```

## `RawContour::displacement` decl

```rust
Some(
    [
        "i32",
        "i32",
        "Vector2d",
    ],
)
```

## `RawContour::displacement` defn

```rust
Some(
    [
        "self",
        "self.points",
        "self.points.ilen()",
        "let N = self.points.ilen()",
        "self",
        "self.points",
        "start",
        "N",
        "start%N",
        "self.points[start%N]",
        "let ct_start = self.points[start%N]",
        "self",
        "self.points",
        "end",
        "N",
        "end%N",
        "self.points[end%N]",
        "let ct_end = self.points[end%N]",
        "ct_start",
        "ct_end",
        "ct_start.to(ct_end)",
        "ct_start.to(ct_end)",
        "let N = self.points.ilen()\n        let ct_start = self.points[start%N]\n        let ct_end = self.points[end%N]\n        ct_start.to(ct_end)",
    ],
)
```

```rust
Some(
    [
        "let mut end = start",
        "let mut dp = ct.displacement(start, end + 1)",
        "let N = ct.points.ilen()",
        "let max_end = start + N",
        "while end <= max_end and dp.norm() < r:\n        end++\n        dp = ct.displacement(start, end + 1)",
        "if dp.norm() < r:\n        return end",
        "let mut right_bound = go_right(dp, r)",
        "let mut left_bound = go_left(dp, r)",
        "let mut r_max = 0.0",
        "while end <= max_end\n            and right_bound.rotation_direction_to(dp) >= 0 \n            and dp.rotation_direction_to(left_bound) >= 0:\n        let dp_norm = dp.norm()\n        if dp_norm < r_max - r:\n            break\n        elif dp_norm > r_max:\n            r_max = dp_norm\n        if dp_norm > r:\n            let dp_right = go_right(dp, r)\n            let dp_left = go_left(dp, r)\n            if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right\n            if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left\n        end++\n        dp = ct.displacement(start, end + 1)",
        "assert end > start",
        "return end",
        "let mut end = start\n    let mut dp = ct.displacement(start, end + 1)\n    let N = ct.points.ilen()\n    let max_end = start + N\n    while end <= max_end and dp.norm() < r:\n        end++\n        dp = ct.displacement(start, end + 1)\n    if dp.norm() < r:\n        return end\n    let mut right_bound = go_right(dp, r)\n    let mut left_bound = go_left(dp, r)\n    let mut r_max = 0.0\n    while end <= max_end\n            and right_bound.rotation_direction_to(dp) >= 0 \n            and dp.rotation_direction_to(left_bound) >= 0:\n        let dp_norm = dp.norm()\n        if dp_norm < r_max - r:\n            break\n        elif dp_norm > r_max:\n            r_max = dp_norm\n        if dp_norm > r:\n            let dp_right = go_right(dp, r)\n            let dp_left = go_left(dp, r)\n            if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right\n            if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left\n        end++\n        dp = ct.displacement(start, end + 1)\n    assert end > start\n    return end",
    ],
)
```
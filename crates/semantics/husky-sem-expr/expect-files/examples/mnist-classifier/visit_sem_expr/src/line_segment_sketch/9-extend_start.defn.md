```rust
Some(
    [
        "let mut start = end",
        "let mut dp0 = ct.displacement(end, start - 1)",
        "let min_start = end - ct.points.ilen()",
        "while start >= min_start and dp0.norm() < r:\n        start--\n        dp0 = ct.displacement(end, start - 1)",
        "if dp0.norm() < r:\n        return start.min(start0)",
        "let mut right_bound = go_right(dp0, r)",
        "let mut left_bound = go_left(dp0, r)",
        "let mut r_max = 0.0",
        "while start >= min_start:\n        let dp = ct.displacement(end, start - 1)\n        let dp_norm = dp.norm()\n        if dp_norm < r_max - r:\n            break\n        elif dp_norm > r_max:\n            r_max = dp_norm\n        if dp_norm > r:\n            let dp_right = go_right(dp, r)\n            let dp_left = go_left(dp, r)\n            if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right\n            if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left\n        if right_bound.rotation_direction_to(left_bound) >= 0:\n            if start <= start0 \n                    and !(\n                        right_bound.rotation_direction_to(dp) >= 0 \n                        and dp.rotation_direction_to(left_bound) >= 0\n                    ):\n                break\n            start--\n        else:\n            break",
        "if start <= start0:\n        return start\n    else:\n        return start0",
        "let mut start = end\n    let mut dp0 = ct.displacement(end, start - 1)\n    let min_start = end - ct.points.ilen()\n    while start >= min_start and dp0.norm() < r:\n        start--\n        dp0 = ct.displacement(end, start - 1)\n    if dp0.norm() < r:\n        return start.min(start0)\n    let mut right_bound = go_right(dp0, r)\n    let mut left_bound = go_left(dp0, r)\n    let mut r_max = 0.0\n    while start >= min_start:\n        let dp = ct.displacement(end, start - 1)\n        let dp_norm = dp.norm()\n        if dp_norm < r_max - r:\n            break\n        elif dp_norm > r_max:\n            r_max = dp_norm\n        if dp_norm > r:\n            let dp_right = go_right(dp, r)\n            let dp_left = go_left(dp, r)\n            if right_bound.rotation_direction_to(dp_right) > 0:\n                right_bound = dp_right\n            if dp_left.rotation_direction_to(left_bound) > 0:\n                left_bound = dp_left\n        if right_bound.rotation_direction_to(left_bound) >= 0:\n            if start <= start0 \n                    and !(\n                        right_bound.rotation_direction_to(dp) >= 0 \n                        and dp.rotation_direction_to(left_bound) >= 0\n                    ):\n                break\n            start--\n        else:\n            break\n    //assert start <= start0\n    if start <= start0:\n        return start\n    else:\n        return start0",
    ],
)
```
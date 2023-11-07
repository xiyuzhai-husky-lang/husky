
struct RawContour{cc: Leash<ConnectedComponent>, points: Vec<Point2d>}

struct Direction

pub fn get_pixel_pair(row: r32, j: i32) -> r32 {
    v0 >> v1 - 1 | 3
}

pub fn get_pixel_to_the_left(row: r32, j: i32) -> r32 {
    v0 >> v1 | 1
}

pub fn get_pixel_to_the_right(row: r32, j: i32) -> r32 {
    v0 >> v1 - 1 | 1
}

pub fn get_inward_direction(row_above: r32, row_below: r32, j: i32) -> Direction {
    let pixel_pair_above = get_pixel_pair(v0, v2);
    let pixel_pair_below = get_pixel_pair(v1, v2);
    match 
}

pub fn get_angle_change(inward: Direction, outward: Direction) -> i32 {
    let raw_angle_change = ((v1 as i32) - (v0 as i32) as r32).last_bits(2);
    match 
}

pub fn get_outward_direction(row_above: r32, row_below: r32, j: i32, inward_direction: Direction) -> Direction {
    let pixel_pair_above = get_pixel_pair(v0, v2);
    let pixel_pair_below = get_pixel_pair(v1, v2);
    match 
}

struct StreakCache{prev1: i32, prev2: i32}

pub fn get_concave_middle_point(points: Vec<Point2d>) -> Point2d {
    let N = v0.ilen();
    let p0 = v0[v1 - 2];
    let p2 = v0[v1 - 1];
    Point2d((v2.x + v3.x) / 2, (v2.y + v3.y) / 2)
}

pub fn find_raw_contours(cc: Leash<ConnectedComponent>) -> Vec<RawContour> {
    let mut result: Vec<RawContour> = vec![];
    let mut boundary_unsearched = new_zeros();
    for i in 1..=29 {
        let r_ur = v0.mask[v3 - 1];
        let r_dr = v0.mask[v3];
        let r_ul = v4 << 1;
        let r_dl = v5 << 1;
        v2[v3] = v4 | v5 | v6 | v7 | !(v4 | v5 | v6 | v7)
    }
    for k in 1..=29 {
        while v2[v8] {
            let mut contour: Vec<Point2d> = vec![];
            let mut i = v8;
            let mut j = v2[v8].ctz();
            let mut row_above = v0.mask[v10 - 1];
            let mut row_below = v0.mask[v10];
            let mut inward_direction = get_inward_direction(v12, v13, v11);
            let i0 = v10;
            let j0 = v11;
            let dir0 = v14;
            let mut prev_angle_change1 = 0;
            let mut prev_angle_change2 = 0;
            let mut total_angle_change = 0;
            let mut prev_streak1 = -1;
            let mut prev_streak2 = -1;
            let mut current_streak = -1;// DoWhile incomplete
            while true
            if v18 == -1 && v23 == 1 && v21 > 0 {
                v9.pop();
            }
            v1.push(RawContour(v0, v9))
        }
    }
    return v1;
}
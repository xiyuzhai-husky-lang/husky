
struct RawContour{cc: Leash<ConnectedComponent>, points: Vec<Point2d>}

struct Direction

pub fn get_pixel_pair(, ) {
    row >> j - 1 | 3;
}

pub fn get_pixel_to_the_left(, ) {
    row >> j | 1;
}

pub fn get_pixel_to_the_right(, ) {
    row >> j - 1 | 1;
}

pub fn get_inward_direction(, , ) {
    let pixel_pair_above = get_pixel_pair(row_above, j);
    let pixel_pair_below = get_pixel_pair(row_below, j);
    match 
}

pub fn get_angle_change(, ) {
    let raw_angle_change = outward as i32 - inward as i32 as r32.last_bits(2);
    match 
}

pub fn get_outward_direction(, , , ) {
    let pixel_pair_above = get_pixel_pair(row_above, j);
    let pixel_pair_below = get_pixel_pair(row_below, j);
    match 
}

struct StreakCache{prev1: i32, prev2: i32}

pub fn get_concave_middle_point() {
    let N = points.ilen();
    let p0 = points[N - 2];
    let p2 = points[N - 1];
    Point2d(p0.x + p2.x / 2, p0.y + p2.y / 2);
}

pub fn find_raw_contours() {
    let result: Vec<RawContour> = vec![];
    let boundary_unsearched = new_zeros();
    for {
        let r_ur = cc.mask[i - 1];
        let r_dr = cc.mask[i];
        let r_ul = r_ur << 1;
        let r_dl = r_dr << 1;
        boundary_unsearched[i] = r_ur | r_dr | r_ul | r_dl | !r_ur | r_dr | r_ul | r_dl;
    }
    for {
        while boundary_unsearched[k] {
            let contour: Vec<Point2d> = vec![];
            let i = k;
            let j = boundary_unsearched[k].ctz();
            let row_above = cc.mask[i - 1];
            let row_below = cc.mask[i];
            let inward_direction = get_inward_direction(row_above, row_below, j);
            let i0 = i;
            let j0 = j;
            let dir0 = inward_direction;
            let prev_angle_change1 = 0;
            let prev_angle_change2 = 0;
            let total_angle_change = 0;
            let prev_streak1 = -1;
            let prev_streak2 = -1;
            let current_streak = -1;
            while true
            if prev_angle_change1 == -1 && current_streak == 1 && prev_streak1 > 0 {
                contour.pop();
            }
            result.push(RawContour(cc, contour));
        }
    }
    return result;
}
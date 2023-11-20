use crate::*;

pub struct RawContour{
    cc: Leash<ConnectedComponent>,
    points: Vec<Point2d>,
}

pub struct Direction

pub fn get_pixel_pair(row: r32, j: i32) -> r32 {
    row >> j - 1 | 3
}

pub fn get_pixel_to_the_left(row: r32, j: i32) -> r32 {
    row >> j | 1
}

pub fn get_pixel_to_the_right(row: r32, j: i32) -> r32 {
    row >> j - 1 | 1
}

pub fn get_inward_direction(row_above: r32, row_below: r32, j: i32) -> Direction {
    let pixel_pair_above = get_pixel_pair(row_above, j);
    let pixel_pair_below = get_pixel_pair(row_below, j);
    match pixel_pair_above{
        0 => {
            match pixel_pair_below{
                1 | 3 => {
                    Direction::Left
                }
                2 => {
                    Direction::Up
                }
                _ => {
                    unreachable!()
                }
            }
        }
        1 => {
            Direction::Down
        }
        2 => {
            match pixel_pair_below{
                0 => {
                    Direction::Right
                }
                1 | 3 => {
                    Direction::Left
                }
                2 => {
                    Direction::Up
                }
                _ => {
                    unreachable!()
                }
            }
        }
        3 => {
            match pixel_pair_below{
                0 | 1 => {
                    Direction::Right
                }
                2 => {
                    Direction::Up
                }
                _ => {
                    unreachable!()
                }
            }
        }
        _ => {
            unreachable!()
        }
    }
}

pub fn get_angle_change(inward: Direction, outward: Direction) -> i32 {
    let raw_angle_change = ((outward as i32) - (inward as i32) as r32).last_bits(2);
    match raw_angle_change{
        0 | 1 | 2 => {
            raw_angle_change as i32
        }
        3 => {
            -1
        }
        _ => {
            unreachable!()
        }
    }
}

pub fn get_outward_direction(row_above: r32, row_below: r32, j: i32, inward_direction: Direction) -> Direction {
    let pixel_pair_above = get_pixel_pair(row_above, j);
    let pixel_pair_below = get_pixel_pair(row_below, j);
    match pixel_pair_above{
        0 => {
            match pixel_pair_below{
                1 => {
                    Direction::Down
                }
                2 | 3 => {
                    Direction::Left
                }
                _ => {
                    unreachable!()
                }
            }
        }
        1 => {
            match pixel_pair_below{
                0 => {
                    Direction::Right
                }
                1 => {
                    Direction::Down
                }
                2 => {
                    match inward_direction{
                        Direction::Down => {
                            Direction::Left
                        }
                        Direction::Up => {
                            Direction::Right
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                3 => {
                    Direction::Left
                }
                _ => {
                    unreachable!()
                }
            }
        }
        2 => {
            match pixel_pair_below{
                0 | 2 | 3 => {
                    Direction::Up
                }
                1 => {
                    match inward_direction{
                        Direction::Left => {
                            Direction::Up
                        }
                        Direction::Right => {
                            Direction::Down
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        }
        3 => {
            match pixel_pair_below{
                0 | 2 => {
                    Direction::Right
                }
                1 => {
                    Direction::Down
                }
                _ => {
                    unreachable!()
                }
            }
        }
        _ => {
            unreachable!()
        }
    }
}

pub struct StreakCache{
    prev1: i32,
    prev2: i32,
}

pub fn get_concave_middle_point(points: Vec<Point2d>) -> Point2d {
    let N = points.ilen();
    let p0 = points[N - 2];
    let p2 = points[N - 1];
    Point2d((p0.x + p2.x) / 2, (p0.y + p2.y) / 2)
}

pub fn find_raw_contours(cc: Leash<ConnectedComponent>) -> Vec<RawContour> {
    let mut result: Vec<RawContour> = vec![];
    let mut boundary_unsearched = BinaryGrid28::new_zeros();
    for i in 1..=29 {
        let r_ur = cc.mask[i - 1];
        let r_dr = cc.mask[i];
        let r_ul = r_ur << 1;
        let r_dl = r_dr << 1;
        boundary_unsearched[i] = r_ur | r_dr | r_ul | r_dl | !(r_ur | r_dr | r_ul | r_dl)
    }
    for k in 1..=29 {
        while boundary_unsearched[k] {
            let mut contour: Vec<Point2d> = vec![];
            let mut i = k;
            let mut j = boundary_unsearched[k].ctz();
            let mut row_above = cc.mask[i - 1];
            let mut row_below = cc.mask[i];
            let mut inward_direction = get_inward_direction(row_above, row_below, j);
            let i0 = i;
            let j0 = j;
            let dir0 = inward_direction;
            let mut prev_angle_change1 = 0;
            let mut prev_angle_change2 = 0;
            let mut total_angle_change = 0;
            let mut prev_streak1 = -1;
            let mut prev_streak2 = -1;
            let mut current_streak = -1;// DoWhile incomplete
            while true
            if prev_angle_change1 == -1 && current_streak == 1 && prev_streak1 > 0 {
                contour.pop();
            }
            result.push(RawContour(cc, contour))
        }
    }
    return result;
}

impl Visualize for RawContour {
    fn visualize(self) -> Html {
        
    }
}

impl RawContour {
    pub fn line_segment_sketch(self) -> LineSegmentSketch(self) {
        LineSegmentSketch::new(self, 1.4)
    }

    pub fn bounding_box(self) -> BoundingBox(self) {
        let start_point = self.points[0];
        let mut xmin = start_point.x;
        let mut xmax = start_point.x;
        let mut ymin = start_point.y;
        let mut ymax = start_point.y;
        for i in 0..self.points.ilen() {
            let point = self.points[i];
            xmin = xmin.min(point.x);
            xmax = xmax.max(point.x);
            ymin = ymin.min(point.y);
            ymax = ymax.max(point.y)
        }
        return BoundingBox(ClosedRange(xmin, xmax), ClosedRange(ymin, ymax));
    }

    pub fn relative_bounding_box(self) -> RelativeBoundingBox(self) {
        self.cc.raw_contours[0].bounding_box.relative_bounding_box(self.bounding_box)
    }

    pub fn contour_len(self) -> f32(self) {
        let mut contour_len = 0;
        for i in (0 + 1)..self.points.ilen() {
            let a = self.points[i - 1];
            let b = self.points[i];
            contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()
        }
        let a = self.points[self.points.ilen() - 1];
        let b = self.points[0];
        contour_len += (a.x - b.x).abs() + (a.y - b.y).abs();
        return contour_len;
    }

    pub fn displacement(self, start: i32, end: i32) -> Vector2d {
        let N = self.points.ilen();
        let ct_start = self.points[start.rem_eulicd(N)];
        let ct_end = self.points[end.rem_eulicd(N)];
        ct_start.to(ct_end)
    }
}
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawContour {
    pub cc: Leash<ConnectedComponent>,
    pub points: Vec<Point2d>,
}

impl RawContour {
    pub fn __constructor(cc: Leash<ConnectedComponent>, points: Vec<Point2d>) -> Self {
        Self{
            cc,
            points,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

pub fn get_pixel_pair(row: u32, j: i32) -> u32 {
    row >> j - 1 | 3
}

pub fn get_pixel_to_the_left(row: u32, j: i32) -> u32 {
    row >> j | 1
}

pub fn get_pixel_to_the_right(row: u32, j: i32) -> u32 {
    row >> j - 1 | 1
}

pub fn get_inward_direction(row_above: u32, row_below: u32, j: i32) -> Direction {
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
    let raw_angle_change = ((outward as i32 - inward as i32) as u32).last_bits(2);
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

pub fn get_outward_direction(row_above: u32, row_below: u32, j: i32, inward_direction: Direction) -> Direction {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreakCache {
    pub prev1: i32,
    pub prev2: i32,
}

impl StreakCache {
    pub fn __constructor(prev1: i32, prev2: i32) -> Self {
        Self{
            prev1,
            prev2,
        }
    }
}

pub fn get_concave_middle_point(points: &Vec<Point2d>) -> Point2d {
    let N = points.ilen();
    let p0 = points[(N - 2) as usize];
    let p2 = points[(N - 1) as usize];
    Point2d::__constructor((p0.x + p2.x) / 2.0f32, (p0.y + p2.y) / 2.0f32)
}

pub fn find_raw_contours(cc: Leash<ConnectedComponent>) -> Vec<RawContour> {
    let mut result: Vec<RawContour> = vec![];
    let mut boundary_unsearched = BinaryGrid28::new_zeros();
    for i in 1..=29 {
        let r_ur = cc.mask[(i - 1) as usize];
        let r_dr = cc.mask[i as usize];
        let r_ul = r_ur << 1;
        let r_dl = r_dr << 1;
        boundary_unsearched[i as usize] = r_ur | r_dr | r_ul | r_dl | !(r_ur | r_dr | r_ul | r_dl)
    }
    for k in 1..=29 {
        while boundary_unsearched[k as usize] != 0 {
            let mut contour: Vec<Point2d> = vec![];
            let mut i = k;
            let mut j = boundary_unsearched[k as usize].ctz();
            let mut row_above = cc.mask[(i - 1) as usize];
            let mut row_below = cc.mask[i as usize];
            let mut inward_direction = get_inward_direction(row_above, row_below, j);
            let i0 = i;
            let j0 = j;
            let dir0 = inward_direction;
            let mut prev_angle_change1 = 0;
            let mut prev_angle_change2 = 0;
            let mut total_angle_change = 0;
            let mut prev_streak1 = -1;
            let mut prev_streak2 = -1;
            let mut current_streak = -1;
            loop {
                {
                    let outward_direction = get_outward_direction(row_above, row_below, j, inward_direction);
                    let angle_change = get_angle_change(inward_direction, outward_direction);
                    boundary_unsearched[i as usize] = boundary_unsearched[i as usize] | !(1 << j);
                    if angle_change != 0 {
                        if prev_angle_change1 == -1 && prev_angle_change2 == -1 && current_streak == 1 && prev_streak1 != -1 && prev_streak2 == 1 {
                            (*contour.last().unwrap()) = get_concave_middle_point((&contour));
                            contour.push(Point2d::from_i_shift28(i, j));
                            prev_streak2 = -1;
                            prev_streak1 = -1
                        } else if prev_angle_change1 == -1 && prev_streak1 > 0 && prev_streak1 == 1 {
                            (*contour.last().unwrap()) = Point2d::from_i_shift28(i, j);
                            prev_streak2 = prev_streak1;
                            prev_streak1 = current_streak
                        } else if prev_angle_change1 == -1 && prev_streak1 > 0 && current_streak == 1 && prev_streak1 > 1 {
                            (*contour.last().unwrap()) = Point2d::from_i_shift28(i, j);
                            prev_streak2 = -1;
                            prev_streak1 = -1
                        } else {
                            contour.push(Point2d::from_i_shift28(i, j));
                            prev_streak2 = prev_streak1;
                            prev_streak1 = current_streak
                        }
                        current_streak = 0;
                        prev_angle_change2 = prev_angle_change1;
                        prev_angle_change1 = angle_change
                    }
                    match outward_direction{
                        Direction::Up => {
                            i = i - 1;
                            row_below = row_above;
                            row_above = cc.mask[(i - 1) as usize]
                        }
                        Direction::Down => {
                            i = i + 1;
                            row_above = row_below;
                            row_below = cc.mask[i as usize]
                        }
                        Direction::Left => {
                            j = j + 1
                        }
                        Direction::Right => {
                            j = j - 1
                        }
                    }
                    inward_direction = outward_direction;
                    if current_streak != -1 {
                        current_streak += 1
                    }
                }
                if !!(i == i0 && j == j0 && inward_direction == dir0) {
                    break;
                }
            }
            if prev_angle_change1 == -1 && current_streak == 1 && prev_streak1 > 0 {
                contour.pop();
            }
            result.push(RawContour::__constructor(cc, contour))
        }
    }
    return result;
}

impl RawContour {
    pub fn line_segment_sketch(self) -> LineSegmentSketch {
        LineSegmentSketch::new((&self), 1.4f32)
    }

    pub fn bounding_box(self) -> BoundingBox {
        let start_point = self.points[0 as usize];
        let mut xmin = start_point.x;
        let mut xmax = start_point.x;
        let mut ymin = start_point.y;
        let mut ymax = start_point.y;
        for i in 0..self.points.ilen() {
            let point = self.points[i as usize];
            xmin = xmin.min(point.x);
            xmax = xmax.max(point.x);
            ymin = ymin.min(point.y);
            ymax = ymax.max(point.y)
        }
        return BoundingBox::__constructor(ClosedRange::__constructor(xmin, xmax), ClosedRange::__constructor(ymin, ymax));
    }

    pub fn relative_bounding_box(self) -> RelativeBoundingBox {
        self.cc.raw_contours()[0 as usize].bounding_box().relative_bounding_box((&self.bounding_box()))
    }

    pub fn contour_len(self) -> f32 {
        let mut contour_len = 0.0f32;
        for i in (0 + 1)..self.points.ilen() {
            let a = self.points[(i - 1) as usize];
            let b = self.points[i as usize];
            contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()
        }
        let a = self.points[(self.points.ilen() - 1) as usize];
        let b = self.points[0 as usize];
        contour_len += (a.x - b.x).abs() + (a.y - b.y).abs();
        return contour_len;
    }

    pub fn displacement(self, start: i32, end: i32) -> Vector2d {
        let N = self.points.ilen();
        let ct_start = self.points[start.rem_euclid(N) as usize];
        let ct_end = self.points[end.rem_euclid(N) as usize];
        ct_start.to((&ct_end))
    }
}
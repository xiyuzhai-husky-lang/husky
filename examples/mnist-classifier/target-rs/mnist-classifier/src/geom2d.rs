struct Point2d{
    x: f32,
    y: f32,
}

struct RelativePoint2d{
    x: f32,
    y: f32,
}

struct Vector2d{
    x: f32,
    y: f32,
}

struct ClosedRange{
    min: f32,
    max: f32,
}

struct BoundingBox{
    xrange: ClosedRange,
    yrange: ClosedRange,
}

struct RelativeBoundingBox{
    xrange: ClosedRange,
    yrange: ClosedRange,
}

impl {
    fn from_i_shift28(i: i32, shift: i32) {
        Point2d(29 - shift as f32, 29 - i as f32)
    }

    fn vector(self) {
        Vector2d(self.x, self.y)
    }

    fn to(self, other: Point2d) {
        Vector2d(other.x - self.x, other.y - self.y)
    }

    fn norm(self) {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dist(self, other: Point2d) {
        self.to(other).norm()
    }
}

impl {
    fn point(self) {
        Point2d(self.x, self.y)
    }

    fn to(self, other: Vector2d) {
        Vector2d(other.x - self.x, other.y - self.y)
    }

    fn norm(self) {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dot(self, other: Vector2d) {
        self.x * other.x + self.y * other.y
    }

    fn cross(self, other: Vector2d) {
        self.x * other.y - self.y * other.x
    }

    fn angle(self, is_branch_cut_positive: bool) {
        let cos_value = (self.x / self.norm()).min(1);
        if cos_value + 1 < 0.001 {
            if is_branch_cut_positive {
                180
            } else {
                -180
            }
        } else {
            (self.y.sgnx() as f32) * cos_value.acos() * 180 / 3.1415925
        }
    }

    fn rotation_direction_to(self, other: Vector2d) {
        self.cross(other).sgnx()
    }

    fn angle_to(self, other: Vector2d, is_branch_cut_positive: bool) {
        let self_norm = self.norm();
        assert!(self_norm > 0);
        let other_norm = other.norm();
        assert!(other_norm > 0);
        let cos_value = (self.dot(other) / (self_norm * other_norm)).min(1);
        if cos_value + 1 < 0.001 {
            if is_branch_cut_positive {
                180
            } else {
                -180
            }
        } else {
            let arc_angle = (self.rotation_direction_to(other) as f32) * cos_value.acos();
            arc_angle * 180 / 3.1415925
        }
    }
}

impl {
    fn relative_range(self, other: ClosedRange) {
        assert!(self.max > self.min);
        let span = self.max - self.min;
        let rel_min = (other.min - self.min) / span;
        let rel_max = (other.max - self.min) / span;
        ClosedRange(rel_min, rel_max)
    }

    fn relative_point(self, v: f32) {
        let span = self.max - self.min;
        (v - self.min) / span
    }
}

impl {
    fn relative_bounding_box(self, other: BoundingBox) {
        RelativeBoundingBox(self.xrange.relative_range(other.xrange), self.yrange.relative_range(other.yrange))
    }

    fn relative_point(self, point: Point2d) {
        RelativePoint2d(self.xrange.relative_point(point.x), self.yrange.relative_point(point.x))
    }

    fn xmin(self) {
        self.xrange.min
    }

    fn xmax(self) {
        self.xrange.max
    }

    fn ymin(self) {
        self.yrange.min
    }

    fn ymax(self) {
        self.yrange.max
    }
}

impl {
    fn xmin(self) {
        self.xrange.min
    }

    fn xmax(self) {
        self.xrange.max
    }

    fn ymin(self) {
        self.yrange.min
    }

    fn ymax(self) {
        self.yrange.max
    }
}
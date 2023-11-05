
struct Point2d{ x : f32, y : f32}

struct RelativePoint2d{ x : f32, y : f32}

struct Vector2d{ x : f32, y : f32}

struct ClosedRange{ min : f32, max : f32}

struct BoundingBox{ xrange : ClosedRange, yrange : ClosedRange}

struct RelativeBoundingBox{ xrange : ClosedRange, yrange : ClosedRange}

impl {
    fn from_i_shift28(, ) {
        Point2d(29- shiftas f32, 29- ias f32);
    }

    fn vector() {
        Vector2d(Self.x, Self.y);
    }

    fn to(, ) {
        Vector2d( other.x-Self.x, other.y-Self.y);
    }

    fn norm() {
        Self.x*Self.x+Self.y*Self.y.sqrt();
    }

    fn dist(, ) {
        Self.to( other).norm();
    }
}

fn from_i_shift28(, ) {
    Point2d(29- shiftas f32, 29- ias f32);
}

fn vector() {
    Vector2d(Self.x, Self.y);
}

fn to(, ) {
    Vector2d( other.x-Self.x, other.y-Self.y);
}

fn norm() {
    Self.x*Self.x+Self.y*Self.y.sqrt();
}

fn dist(, ) {
    Self.to( other).norm();
}

impl {
    fn point() {
        Point2d(Self.x, Self.y);
    }

    fn to(, ) {
        Vector2d( other.x-Self.x, other.y-Self.y);
    }

    fn norm() {
        Self.x*Self.x+Self.y*Self.y.sqrt();
    }

    fn dot(, ) {
        Self.x* other.x+Self.y* other.y;
    }

    fn cross(, ) {
        Self.x* other.y-Self.y* other.x;
    }

    fn angle(, ) {
        let cos_value =Self.x/Self.norm().min(1);
        if cos_value+1<0.001 {
            if is_branch_cut_positive {
                180;
            } else {
                -180;
            }
        } else {
            Self.y.sgnx()as f32* cos_value.acos()*180/3.1415925;
        }
    }

    fn rotation_direction_to(, ) {
        Self.cross( other).sgnx();
    }

    fn angle_to(, , ) {
        let self_norm =Self.norm(); assert!( self_norm>0)
        let other_norm = other.norm(); assert!( other_norm>0)
        let cos_value =Self.dot( other)/ self_norm* other_norm.min(1);
        if cos_value+1<0.001 {
            if is_branch_cut_positive {
                180;
            } else {
                -180;
            }
        } else {
            let arc_angle =Self.rotation_direction_to( other)as f32* cos_value.acos();
            arc_angle*180/3.1415925;
        }
    }
}

fn point() {
    Point2d(Self.x, Self.y);
}

fn to(, ) {
    Vector2d( other.x-Self.x, other.y-Self.y);
}

fn norm() {
    Self.x*Self.x+Self.y*Self.y.sqrt();
}

fn dot(, ) {
    Self.x* other.x+Self.y* other.y;
}

fn cross(, ) {
    Self.x* other.y-Self.y* other.x;
}

fn angle(, ) {
    let cos_value =Self.x/Self.norm().min(1);
    if cos_value+1<0.001 {
        if is_branch_cut_positive {
            180;
        } else {
            -180;
        }
    } else {
        Self.y.sgnx()as f32* cos_value.acos()*180/3.1415925;
    }
}

fn rotation_direction_to(, ) {
    Self.cross( other).sgnx();
}

fn angle_to(, , ) {
    let self_norm =Self.norm(); assert!( self_norm>0)
    let other_norm = other.norm(); assert!( other_norm>0)
    let cos_value =Self.dot( other)/ self_norm* other_norm.min(1);
    if cos_value+1<0.001 {
        if is_branch_cut_positive {
            180;
        } else {
            -180;
        }
    } else {
        let arc_angle =Self.rotation_direction_to( other)as f32* cos_value.acos();
        arc_angle*180/3.1415925;
    }
}

impl {
    fn relative_range(, ) { assert!(Self.max>Self.min)
        let span =Self.max-Self.min;
        let rel_min = other.min-Self.min/ span;
        let rel_max = other.max-Self.min/ span;
        ClosedRange( rel_min, rel_max);
    }

    fn relative_point(, ) {
        let span =Self.max-Self.min;
        v-Self.min/ span;
    }
}

fn relative_range(, ) { assert!(Self.max>Self.min)
    let span =Self.max-Self.min;
    let rel_min = other.min-Self.min/ span;
    let rel_max = other.max-Self.min/ span;
    ClosedRange( rel_min, rel_max);
}

fn relative_point(, ) {
    let span =Self.max-Self.min;
    v-Self.min/ span;
}

impl {
    fn relative_bounding_box(, ) {
        RelativeBoundingBox(Self.xrange.relative_range( other.xrange), Self.yrange.relative_range( other.yrange));
    }

    fn relative_point(, ) {
        RelativePoint2d(Self.xrange.relative_point( point.x), Self.yrange.relative_point( point.x));
    }

    fn xmin() {
        Self.xrange.min;
    }

    fn xmax() {
        Self.xrange.max;
    }

    fn ymin() {
        Self.yrange.min;
    }

    fn ymax() {
        Self.yrange.max;
    }
}

fn relative_bounding_box(, ) {
    RelativeBoundingBox(Self.xrange.relative_range( other.xrange), Self.yrange.relative_range( other.yrange));
}

fn relative_point(, ) {
    RelativePoint2d(Self.xrange.relative_point( point.x), Self.yrange.relative_point( point.x));
}

fn xmin() {
    Self.xrange.min;
}

fn xmax() {
    Self.xrange.max;
}

fn ymin() {
    Self.yrange.min;
}

fn ymax() {
    Self.yrange.max;
}

impl {
    fn xmin() {
        Self.xrange.min;
    }

    fn xmax() {
        Self.xrange.max;
    }

    fn ymin() {
        Self.yrange.min;
    }

    fn ymax() {
        Self.yrange.max;
    }
}

fn xmin() {
    Self.xrange.min;
}

fn xmax() {
    Self.xrange.max;
}

fn ymin() {
    Self.yrange.min;
}

fn ymax() {
    Self.yrange.max;
}
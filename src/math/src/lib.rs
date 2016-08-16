#[macro_use]
extern crate log;
extern crate env_logger;

pub type Float = f32;

#[derive(Copy, Clone)]
pub struct Rect {
    corners: LineSeg,
}

impl Rect {
    pub fn new_from_coords(x0: Float, y0: Float, x1: Float, y1: Float) -> Rect {
        Rect::new_from_points(Point2::new(x0, y0), Point2::new(x1, y1))
    }

    pub fn new_from_points(a: Point2, b: Point2) -> Rect {
        Rect::new(LineSeg::new(a, b))
    }

    pub fn new(corners: LineSeg) -> Rect {
        assert!(corners.get_a().get_x() < corners.get_b().get_x());
        assert!(corners.get_a().get_y() < corners.get_b().get_y());
        Rect {
            corners: corners,
        }
    }

    pub fn get_corners(&self) -> LineSeg {
        self.corners
    }

    pub fn get_bot_left(&self) -> Point2 {
        self.corners.get_a()
    }

    pub fn get_top_right(&self) -> Point2 {
        self.corners.get_b()
    }

    pub fn check_collide_point(&self, point: Point2) -> bool {
        trace!("Rect Points: ({},{}) ({},{})", self.get_bot_left().get_x(), self.get_bot_left().get_y(), self.get_top_right().get_x(), self.get_top_right().get_y());
        self.get_bot_left().get_x() <= point.get_x() &&
        self.get_bot_left().get_y() <= point.get_y() &&
        self.get_top_right().get_x() >= point.get_x() &&
        self.get_top_right().get_y() >= point.get_y()
    }

    pub fn check_collide_point_with_offset(&self, offset: Point2, point: Point2) -> bool {
        self.check_collide_point(point.add(offset))
    }
}

#[derive(Copy, Clone)]
pub struct LineSeg {
    a: Point2,
    b: Point2,
}

impl LineSeg {
    pub fn new_from_coords(x0: Float, y0: Float, x1: Float, y1: Float) -> LineSeg {
        LineSeg::new(Point2::new(x0, y0), Point2::new(x1, y1))
    }

    pub fn new(a: Point2, b: Point2) -> LineSeg {
        LineSeg {
            a: a,
            b: b,
        }
    }

    pub fn get_a(&self) -> Point2 {
        self.a
    }

    pub fn get_b(&self) -> Point2 {
        self.b
    }
}

#[derive(Copy, Clone)]
pub struct Point2 {
    x: Float,
    y: Float,
}

impl Point2 {
    pub fn new(x: Float, y: Float) -> Point2 {
        Point2 {
            x: x,
            y: y,
        }
    }

    pub fn get_x(&self) -> Float {
        self.x
    }

    pub fn get_y(&self) -> Float {
        self.y
    }

    pub fn add(&self, other: Point2) -> Point2 {
        Point2::new(self.get_x() + other.get_x(), self.get_y() + other.get_y())
    }
}

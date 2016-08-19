extern crate nalgebra;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate utils;

pub mod ortho_helper;

pub use self::ortho_helper::OrthographicHelper;

#[derive(Debug, Clone)]
pub struct Rect {
    corners: LineSeg,
}

impl Rect {
    pub fn new_from_coords(x0: ::utils::Coord, y0: ::utils::Coord, x1: ::utils::Coord, y1: ::utils::Coord) -> Rect {
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
        self.corners.clone()
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
}

#[derive(Debug, Clone)]
pub struct LineSeg {
    a: Point2,
    b: Point2,
}

impl LineSeg {
    pub fn new_from_coords(x0: ::utils::Coord, y0: ::utils::Coord, x1: ::utils::Coord, y1: ::utils::Coord) -> LineSeg {
        LineSeg::new(Point2::new(x0, y0), Point2::new(x1, y1))
    }

    pub fn new(a: Point2, b: Point2) -> LineSeg {
        LineSeg {
            a: a,
            b: b,
        }
    }

    pub fn get_a(&self) -> Point2 {
        self.a.clone()
    }

    pub fn get_b(&self) -> Point2 {
        self.b.clone()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point2 {
    x: ::utils::Coord,
    y: ::utils::Coord,
}

impl Point2 {
    pub fn new(x: ::utils::Coord, y: ::utils::Coord) -> Point2 {
        Point2 {
            x: x,
            y: y,
        }
    }

    pub fn zero() -> Point2 {
        Point2::new(0.0, 0.0)
    }

    pub fn get_x(&self) -> ::utils::Coord {
        self.x
    }

    pub fn get_y(&self) -> ::utils::Coord {
        self.y
    }

    pub fn get_mut_x(&mut self) -> &mut ::utils::Coord {
        &mut self.x
    }

    pub fn get_mut_y(&mut self) -> &mut ::utils::Coord {
        &mut self.y
    }

    pub fn normalized(&self) -> Point2 {
        self.clone() / self.length()
    }

    pub fn length(&self) -> ::utils::Coord {
        (self.get_x().powi(2) + self.get_y().powi(2)).sqrt()
    }

    pub fn is_zero(&self) -> bool {
        self.get_x() == 0.0 && self.get_y() == 0.0
    }

    pub fn is_normal(&self) -> bool {
        self.get_x().is_normal() && self.get_y().is_normal()
    }

    pub fn is_finite(&self) -> bool {
        self.get_x().is_finite() && self.get_y().is_finite()
    }

    pub fn abs(&self) -> Point2 {
        Point2::new(self.get_x().abs(), self.get_y().abs())
    }
}

impl std::ops::Add<Point2> for Point2 {
    type Output = Point2;

    fn add(self, other: Point2) -> Point2 {
        Point2::new(self.get_x() + other.get_x(), self.get_y() + other.get_y())
    }
}

impl std::ops::Sub<Point2> for Point2 {
    type Output = Point2;

    fn sub(self, other: Point2) -> Point2 {
        Point2::new(self.get_x() - other.get_x(), self.get_y() - other.get_y())
    }
}

impl std::ops::Mul<Point2> for Point2 {
    type Output = Point2;

    fn mul(self, other: Point2) -> Point2 {
        Point2::new(self.get_x() * other.get_x(), self.get_y() * other.get_y())
    }
}

impl std::ops::Mul<::utils::Coord> for Point2 {
    type Output = Point2;

    fn mul(self, other: ::utils::Coord) -> Point2 {
        Point2::new(self.get_x() * other, self.get_y() * other)
    }
}

impl std::ops::Div<::utils::Coord> for Point2 {
    type Output = Point2;

    fn div(self, other: ::utils::Coord) -> Point2 {
        Point2::new(self.get_x() / other, self.get_y() / other)
    }
}

impl std::ops::SubAssign<Point2> for Point2 {
    fn sub_assign(&mut self, other: Point2) {
        self.x -= other.get_x();
        self.y -= other.get_y();
    }
}

impl std::ops::MulAssign<Point2> for Point2 {
    fn mul_assign(&mut self, other: Point2) {
        self.x *= other.get_x();
        self.y *= other.get_y();
    }
}

impl std::ops::MulAssign<::utils::Coord> for Point2 {
    fn mul_assign(&mut self, other: ::utils::Coord) {
        self.x *= other;
        self.y *= other;
    }
}

impl std::fmt::Display for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.get_x(), self.get_y())
    }
}

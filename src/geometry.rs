use core::panic;
use geo::{ClosestPoint, Coord, LineString};
use geom::{bounding_rect, centroid};
use nannou::prelude::*;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub point1: Point2,
    pub point2: Point2,
}

#[derive(Debug, Clone, PartialEq)]
enum LineIntersection {
    Point(Point2),
    Coincident,
    Parallel,
}

impl Line {
    fn from_point_gradient(point: Point2, m: f32) -> Self {
        // y - y1 = m * (x - x1)
        // c - y1 = m * (0 - x1)
        let c = point.y - m * point.x;
        Line {
            point1: pt2(0., c),
            point2: point,
        }
    }

    pub fn gradient(&self) -> f32 {
        // m = (y2 - y1) / (x2 - x1)
        (self.point2.y - self.point1.y) / (self.point2.x - self.point1.x)
    }

    pub fn y_intercept(&self) -> f32 {
        // y = mx + c, c = y - mx
        self.point1.y - self.gradient() * self.point1.x
    }

    fn intersection_with_line(&self, segment: Line) -> LineIntersection {
        let m = [self.gradient(), segment.gradient()];
        let c = [self.y_intercept(), segment.y_intercept()];

        // Check if parallel
        match (m[0] == m[1], c[0] == c[1]) {
            (true, true) => return LineIntersection::Coincident,
            (true, false) => return LineIntersection::Parallel,
            (false, true) => return LineIntersection::Point(pt2(0., c[0])),
            (false, false) => {}
        }

        // Find intersection
        let x = (c[1] - c[0]) / (m[0] - m[1]);
        let y = m[0] * x + c[0];

        return LineIntersection::Point(vec2(x, y));
    }

    fn reflect_point(&self, point: Point2) -> Point2 {
        let m = -1. / self.gradient();
        let line = Line::from_point_gradient(point, m);
        let intersection = self.intersection_with_line(line);

        match intersection {
            LineIntersection::Point(p) => pt2(
                point.x + 2.0 * (p.x - point.x),
                point.y + 2.0 * (p.y - point.y),
            ),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Above,
    Right,
    Below,
    Left,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Polygon {
    pub points: Vec<Point2>,
}

impl Polygon {
    pub fn new(radius: f32, sides: usize) -> Self {
        let points = (0..sides)
            .map(|side| {
                let angle = ((180. / sides as f32)
                    * (1. + (if sides % 2 == 1 { 1. } else { 0. }) + 2. * side as f32))
                    .to_radians();
                // Get the sine of the radian to find the x co-ordinate of this point of the circle
                // and multiply it by the radius.
                let x = angle.sin() * radius;
                // Do the same with cosine to find the y co-ordinate.
                let y = angle.cos() * radius;
                pt2(x, y)
            })
            .collect();
        Polygon { points }
    }

    /** Align self to be above, below, or to the right or left of polygon */
    pub fn align(&mut self, polygon: &Polygon, direction: Direction) {
        let bounding_boxes = [
            bounding_rect(self.points.clone()).unwrap(),
            bounding_rect(polygon.points.clone()).unwrap(),
        ];

        match direction {
            Direction::Above => {
                self.translate(vec2(
                    (bounding_boxes[1].x.start + bounding_boxes[1].x.end) / 2.
                        - (bounding_boxes[0].x.start + bounding_boxes[0].x.end) / 2.,
                    bounding_boxes[1].y.end - bounding_boxes[0].y.start,
                ));
            }
            Direction::Right => {
                self.translate(vec2(
                    bounding_boxes[1].x.end - bounding_boxes[0].x.start,
                    (bounding_boxes[1].y.start + bounding_boxes[1].y.end) / 2.
                        - (bounding_boxes[0].y.start + bounding_boxes[0].y.end) / 2.,
                ));
            }
            Direction::Below => {
                self.translate(vec2(
                    (bounding_boxes[1].x.start + bounding_boxes[1].x.end) / 2.
                        - (bounding_boxes[0].x.start + bounding_boxes[0].x.end) / 2.,
                    bounding_boxes[1].y.start - bounding_boxes[0].y.end,
                ));
            }
            Direction::Left => {
                self.translate(vec2(
                    bounding_boxes[1].x.start - bounding_boxes[0].x.end,
                    (bounding_boxes[1].y.start + bounding_boxes[1].y.end) / 2.
                        - (bounding_boxes[0].y.start + bounding_boxes[0].y.end) / 2.,
                ));
            }
        }
    }

    pub fn translate(&mut self, translation: Vec2) {
        for point in &mut self.points {
            point.x += translation.x;
            point.y += translation.y;
        }
    }

    pub fn reflect(&mut self, axis: Line) {
        for point in &mut self.points {
            *point = axis.reflect_point(*point);
        }
    }

    /** Rotate clockwise around centroid */
    pub fn rotate(&mut self, angle: f32) {
        let centre = centroid(self.points.clone()).unwrap();
        self.rotate_around_point(centre, angle);
    }

    /** Rotate clockwise, angle is in radians */
    pub fn rotate_around_point(&mut self, centre: Point2, angle: f32) {
        for point in &mut self.points {
            rotate_point(point, centre, angle.sin(), angle.cos());
        }
    }

    pub fn dilate(&mut self, scale: f32) {
        self.dilate_from_point(scale, centroid(self.points.clone()).unwrap());
    }

    pub fn dilate_from_point(&mut self, scale: f32, centre: Point2) {
        for point in &mut self.points {
            let dilated = pt2(point.x - centre.x, point.y - centre.y) * scale;

            // Translate back
            *point = pt2(dilated.x + centre.x, dilated.y + centre.y);
        }
    }

    pub fn distance_to_point(&self, point: Point2) -> f32 {
        let points: Vec<Coord<f32>> = self
            .points
            .iter()
            .map(|p| geo::coord! {x: p.x, y: p.y})
            .collect();
        let polygon = geo::Polygon::new(LineString::from(points), vec![]);
        let closest = polygon.closest_point(&geo::point! {x: point.x, y: point.y});
        match closest {
            geo::Closest::SinglePoint(p) => pt2(p.x(), p.y()).distance(point),
            geo::Closest::Intersection(_) => 0.,
            geo::Closest::Indeterminate => panic!("Unable to determine closest point"),
        }
    }
}

/**
Assumes that b is the centre point, angle is in radians
*/
pub fn angle_between_points(a: Point2, b: Point2, c: Point2) -> f32 {
    let ba = a - b;
    let bc = c - b;

    let dot_product = ba.dot(bc);
    let magnitude_ba = ba.length();
    let magnitude_bc = bc.length();

    if dot_product == 0. || magnitude_ba == 0. || magnitude_bc == 0. {
        return 0.;
    }

    (dot_product / magnitude_ba / magnitude_bc).acos()
}

/** Rotate a point clockwise around another point */
pub fn rotate_point(point: &mut Point2, centre: Point2, sin: f32, cos: f32) {
    // Translate the point to the origin (relative to the centre)
    let translated_x = point.x - centre.x;
    let translated_y = point.y - centre.y;

    // Perform the rotation
    let rotated_x = translated_x * cos - translated_y * sin;
    let rotated_y = translated_x * sin + translated_y * cos;

    // Translate the point back
    point.x = rotated_x + centre.x;
    point.y = rotated_y + centre.y;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointU32 {
    pub x: u32,
    pub y: u32,
}

impl PointU32 {
    pub fn new(x: u32, y: u32) -> Self {
        PointU32 { x, y }
    }
}

impl Add<u32> for PointU32 {
    type Output = Self;
    fn add(self, n: u32) -> PointU32 {
        PointU32 {
            x: self.x + n,
            y: self.y + n,
        }
    }
}

impl Sub<u32> for PointU32 {
    type Output = Self;
    fn sub(self, n: u32) -> PointU32 {
        PointU32 {
            x: self.x - n,
            y: self.y - n,
        }
    }
}

impl Mul<u32> for PointU32 {
    type Output = Self;
    fn mul(self, n: u32) -> PointU32 {
        PointU32 {
            x: self.x * n,
            y: self.y * n,
        }
    }
}

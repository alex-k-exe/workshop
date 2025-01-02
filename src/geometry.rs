use core::panic;
use geom::centroid;
use nannou::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    pub point1: Point2,
    pub point2: Point2,
}

#[derive(Debug, PartialEq)]
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
        if m[0] == m[1] {
            // Check if the intercepts are the same
            if c[0] == c[1] {
                return LineIntersection::Coincident;
            } else {
                return LineIntersection::Parallel;
            }
        } else if c[0] == c[1] {
            return LineIntersection::Point(pt2(0., c[0]));
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

    pub fn rotate(&mut self, angle: f32) {
        let centre = centroid(self.points.clone()).expect("Polygon should have points");
        self.rotate_around_point(centre, angle);
    }

    /**
    angle is in radians
    */
    pub fn rotate_around_point(&mut self, centre: Point2, angle: f32) {
        for point in &mut self.points {
            rotate_point(point, centre, angle.sin(), angle.cos());
        }
    }

    pub fn dilate(&mut self, scale: f32) {
        self.dilate_from_point(
            scale,
            centroid(self.points.clone()).expect("Polygon should have points"),
        );
    }

    pub fn dilate_from_point(&mut self, scale: f32, centre: Point2) {
        for point in &mut self.points {
            let dilated = pt2(point.x - centre.x, point.y - centre.y) * scale;

            // Translate back
            *point = pt2(dilated.x + centre.x, dilated.y + centre.y);
        }
    }

    pub fn point_touches_edge(&self, point: Point2) -> bool {
        for index in 0..self.points.len() {
            let line = Line {
                point1: self.points[index],
                point2: self.points[(index + 1) % self.points.len()],
            };
            if point.y == line.gradient() * point.x + line.y_intercept() {
                return true;
            }
        }
        false
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

    (dot_product / magnitude_ba / magnitude_bc).acos()
}

pub fn rotate_point(point: &mut Point2, centre: Point2, sin: f32, cos: f32) {
    let x = (point.x - centre.x) * cos + (point.y - centre.y) * sin;
    let y = -(point.x - centre.x) * sin + (point.y - centre.y) * cos;

    point.x = x + centre.x;
    point.y = y + centre.y;
}

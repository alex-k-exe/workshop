use core::panic;

use nannou::prelude::*;

const ORIGINAL_WIDTH: f32 = 200.;
const ORIGINAL_SIDES: usize = 4;

pub struct Model {
    polygon: Polygon,
}

pub fn model(app: &App) -> Model {
    app.new_window().fullscreen().view(view).build().unwrap();

    Model {
        polygon: Polygon::new(),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let window = app.window_rect();

    let not_surrounded_polygons = vec![model.polygon.clone()];

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

#[derive(Debug, PartialEq, Clone)]
struct Line {
    point1: Point2,
    point2: Point2,
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

    fn gradient(&self) -> f32 {
        // m = (y2 - y1) / (x2 - x1)
        (self.point2.y - self.point1.y) / (self.point2.x - self.point1.x)
    }

    fn y_intercept(&self) -> f32 {
        // y = mx + c, c = y - mx
        self.point1.y - self.gradient() * self.point1.x
    }

    fn intersection(&self, segment: Line) -> LineIntersection {
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
        let intersection = self.intersection(line);

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
struct Polygon {
    points: Vec<Point2>,
}

impl Polygon {
    fn new() -> Self {
        let radius = ORIGINAL_WIDTH / 2.;
        let points = (0..=360)
            .step_by(360 / ORIGINAL_SIDES)
            .map(|i| {
                // Convert each degree to radians.
                let radian = deg_to_rad(i as f32);
                // Get the sine of the radian to find the x co-ordinate of this point of the circle
                // and multiply it by the radius.
                let x = radian.sin() * radius;
                // Do the same with cosine to find the y co-ordinate.
                let y = radian.cos() * radius;
                pt2(x, y)
            })
            .collect();
        Polygon { points }
    }

    fn translate(&mut self, translation: Vec2) {
        for point in &mut self.points {
            point.x += translation.x;
            point.y += translation.y;
        }
    }

    fn reflect(&mut self, axis: Line) {
        for point in &mut self.points {
            *point = axis.reflect_point(*point);
        }
    }

    fn rotate(&mut self, centre: Point2, angle: f32) {
        for point in &mut self.points {
            let angle_radians = angle.to_radians();

            // Translate point to origin
            let translated_x = point.x - centre.x;
            let translated_y = point.y - centre.y;

            // Rotate
            let rotated_x = translated_x * angle_radians.cos() - translated_y * angle_radians.sin();
            let rotated_y = translated_x * angle_radians.sin() + translated_y * angle_radians.cos();

            // Translate back
            *point = pt2(rotated_x + centre.x, rotated_y + centre.y);
        }
    }

    fn dilate(&mut self, scale: f32) {
        for point in &mut self.points {
            *point = pt2(point.x * scale, point.y * scale);
        }
    }
}

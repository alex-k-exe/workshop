use core::panic;

use crate::geometry::{Line, Polygon};
use geom::bounding_rect;
use nannou::{color::white_point::B, prelude::*};

const RADIUS: f32 = 100.;

pub struct Model {
    fixed: Polygon,
    rotating: Polygon,
    touching_points: Line,
    // next_point is the vertex on the polygon that is next to touch the other polygon
    next_point_fixed: Point2,
    next_point_rotating: Point2,
}

pub fn model(app: &App) -> Model {
    app.new_window().fullscreen().view(view).build().unwrap();

    let fixed = Polygon::new(RADIUS, 3);
    let mut rotating = Polygon::new(RADIUS, 4);
    let bounding_boxes = [
        bounding_rect(fixed.points.clone()).expect("Polygon should have points"),
        bounding_rect(rotating.points.clone()).expect("Polygon should have points"),
    ];

    rotating.translate(vec2(
        0.,
        bounding_boxes[0].y.end - bounding_boxes[1].y.start,
    ));

    let touching_points: Line;
    let next_point_fixed: Point2;
    let next_point_rotating: Point2;
    if bounding_boxes[0].w() > bounding_boxes[1].w() {
        let bottom_left = rotating.points.len() / 4;
        touching_points = Line {
            point1: rotating.points[bottom_left + 1],
            point2: rotating.points[bottom_left],
        };
    } else {
        let length = fixed.points.len();
        let top_left = length - 1 - length / 4;
        touching_points = Line {
            point1: fixed.points[top_left],
            point2: fixed.points[if length % 2 == 0 {
                top_left + 1
            } else {
                top_left
            }],
        };
    }

    Model {
        fixed,
        rotating,
        touching_points,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.polygon()
        .points(model.fixed.points.clone())
        .color(LIGHTBLUE);
    draw.polygon()
        .points(model.rotating.points.clone())
        .color(LIGHTGREEN);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

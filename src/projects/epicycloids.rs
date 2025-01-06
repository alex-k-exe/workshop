use crate::geometry::{angle_between_points, rotate_point, Polygon};
use geom::bounding_rect;
use nannou::prelude::*;

const RADIUS: f32 = 150.;

pub struct Model {
    fixed: Polygon,
    rotating: Polygon,
    // rotating_point is the point that rotating is rotating around
    rotating_point: (Point2, usize),
    // next_point is the vertex on the polygon that is next to touch the other polygon
    next_point_fixed: (Point2, usize),
    next_point_rotating: (Point2, usize),
    traced_path: Vec<Point2>,
    tracing_point: Point2,
}

pub fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();

    let (fixed, rotating, fixed_is_bigger) = initialize_polygons();

    let rotating_point: (Point2, usize);
    let next_point_fixed: (Point2, usize);
    let next_point_rotating: (Point2, usize);

    if fixed.points.len() % 2 == 0 {
        // if fixed is bigger than rotating
        if fixed_is_bigger {
            let bottom_right = rotating.points.len() / 4;
            rotating_point = (rotating.points[bottom_right], bottom_right);
            next_point_fixed = (fixed.points[0], 0);
            if bottom_right == 0 {
                next_point_rotating = (
                    rotating.points[rotating.points.len() - 1],
                    rotating.points.len() - 1,
                );
            } else {
                next_point_rotating = (rotating.points[bottom_right - 1], bottom_right - 1);
            };
        } else {
            rotating_point = (fixed.points[0], 0);
            next_point_fixed = (fixed.points[1], 1);
            next_point_rotating = (
                rotating.points[rotating.points.len() / 4],
                rotating.points.len() / 4,
            );
        }
    } else {
        rotating_point = (fixed.points[fixed.points.len() - 1], fixed.points.len() - 1);
        next_point_fixed = (fixed.points[0], 0);
        next_point_rotating = (
            rotating.points[rotating.points.len() / 4],
            rotating.points.len() / 4,
        );
    }

    Model {
        fixed,
        rotating: rotating.clone(),
        rotating_point,
        next_point_fixed,
        next_point_rotating,
        traced_path: vec![],
        tracing_point: rotating.points[0],
    }
}

pub(crate) fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut angle = angle_between_points(
        model.next_point_rotating.0,
        model.rotating_point.0,
        model.next_point_fixed.0,
    );

    if angle > (5.).to_radians() {
        angle = (5.).to_radians();
        model
            .rotating
            .rotate_around_point(model.rotating_point.0, angle);
        rotate_point(
            &mut model.tracing_point,
            model.rotating_point.0,
            angle.sin(),
            angle.cos(),
        );

        model.next_point_rotating.0 = model.rotating.points[model.next_point_rotating.1];
        model.traced_path.push(model.tracing_point);

        return;
    }

    if model.rotating.distance_to_point(model.next_point_fixed.0)
        < model.fixed.distance_to_point(model.next_point_rotating.0)
    {
        model.rotating_point = model.next_point_fixed.clone();
        (model.next_point_fixed).1 = if (model.next_point_fixed).1 == model.fixed.points.len() - 1 {
            0
        } else {
            (model.next_point_fixed).1 + 1
        };
        (model.next_point_fixed).0 = model.fixed.points[(model.next_point_fixed).1];
        model.next_point_rotating.0 = model.rotating.points[model.next_point_rotating.1];
    } else {
        model.rotating_point = model.next_point_rotating.clone();
        (model.next_point_rotating).1 = if (model.next_point_rotating).1 == 0 {
            model.rotating.points.len() - 1
        } else {
            (model.next_point_rotating).1 - 1
        };
        (model.next_point_rotating).0 = model.rotating.points[(model.next_point_rotating).1];
        model.next_point_fixed.0 = model.fixed.points[model.next_point_fixed.1];
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
    if !model.traced_path.is_empty() {
        draw.polyline().points(model.traced_path.clone()).color(RED);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn initialize_polygons() -> (Polygon, Polygon, bool) {
    let fixed = Polygon::new(RADIUS, 3);
    let mut rotating = Polygon::new(RADIUS * 0.5, 3);
    let bounding_boxes = [
        bounding_rect(fixed.points.clone()).expect("Polygon should have points"),
        bounding_rect(rotating.points.clone()).expect("Polygon should have points"),
    ];

    rotating.translate(vec2(
        0.,
        bounding_boxes[0].y.end - bounding_boxes[1].y.start,
    ));

    (
        fixed,
        rotating,
        bounding_boxes[0].w() > bounding_boxes[1].w(),
    )
}

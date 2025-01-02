use crate::geometry::{angle_between_points, rotate_point, Line, Polygon};
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

    let (fixed, mut rotating, fixed_is_bigger) = initialize_polygons();

    let mut rotating_point: (Point2, usize);
    let mut next_point_fixed: (Point2, usize);
    let mut next_point_rotating: (Point2, usize);

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
        next_point_rotating = (fixed.points[fixed.points.len() - 1], fixed.points.len() - 1);
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

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.polygon()
        .points(model.fixed.points.clone())
        .color(LIGHTBLUE);
    draw.polygon()
        .points(model.rotating.points.clone())
        .color(LIGHTGREEN);

    let line = Line {
        point1: model.rotating_point.0,
        point2: model.next_point_fixed.0,
    };
    draw.line()
        .start(vec2(200., line.gradient() * 200. + line.y_intercept()))
        .end(vec2(-100., line.gradient() * -100. + line.y_intercept()))
        .weight(1.)
        .color(RED);
    draw.ellipse()
        .xy(model.rotating_point.0)
        .radius(5.)
        .color(BLUE);
    draw.ellipse()
        .xy(model.next_point_fixed.0)
        .radius(5.)
        .color(PURPLE);
    draw.ellipse()
        .xy(model.next_point_rotating.0)
        .radius(5.)
        .color(ORANGE);

    draw.to_frame(app, &frame).unwrap();
}

fn initialize_polygons() -> (Polygon, Polygon, bool) {
    let fixed = Polygon::new(RADIUS, 6);
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

fn rotate_things(
    rotating: &mut Polygon,
    rotating_point: &mut (Point2, usize),
    next_point_fixed: &mut (Point2, usize),
    next_point_rotating: &mut (Point2, usize),
    tracing_point: &mut Point2,
) {
    let angle = angle_between_points(next_point_rotating.0, rotating_point.0, next_point_fixed.0);
    rotating.rotate_around_point(rotating_point.0, angle);
    rotate_point(tracing_point, rotating_point.0, angle.sin(), angle.cos());

    next_point_rotating.0 = rotating.points[next_point_rotating.1];
}

fn update_points(
    fixed: &Polygon,
    rotating: &mut Polygon,
    rotating_point: &mut (Point2, usize),
    next_point_fixed: &mut (Point2, usize),
    next_point_rotating: &mut (Point2, usize),
) {
    if rotating.point_touches_edge(next_point_fixed.0) {
        println!("ajjjjj");
        *rotating_point = next_point_fixed.clone();
        (*next_point_fixed).1 = if (*next_point_fixed).1 == fixed.points.len() - 1 {
            0
        } else {
            (*next_point_fixed).1 + 1
        };
        (*next_point_fixed).0 = fixed.points[(*next_point_fixed).1];
        next_point_rotating.0 = rotating.points[next_point_rotating.1];
    } else if fixed.point_touches_edge(next_point_rotating.0) {
        println!("020202");
        *rotating_point = next_point_rotating.clone();
        (*next_point_rotating).1 = if (*next_point_rotating).1 == 0 {
            rotating.points.len() - 1
        } else {
            (*next_point_rotating).1 - 1
        };
        (*next_point_rotating).0 = rotating.points[(*next_point_rotating).1];
        next_point_fixed.0 = fixed.points[next_point_fixed.1];
    } else {
        println!("Oh no!");
    }
}

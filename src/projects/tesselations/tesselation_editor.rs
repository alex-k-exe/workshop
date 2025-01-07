use crate::geometry::{Direction, Polygon, NO_VERTICES_ERROR};
use geom::bounding_rect;
use nannou::prelude::*;

const ORIGINAL_WIDTH: f32 = 100.;
const ORIGINAL_SIDES: usize = 4;

pub struct Model {
    polygons: Vec<Polygon>,
}

pub fn model(app: &App) -> Model {
    app.new_window().fullscreen().view(view).build().unwrap();
    let window = app.window_rect();

    let mut polygon = Polygon::new(ORIGINAL_WIDTH, ORIGINAL_SIDES);
    polygon.translate(vec2(window.x.start, window.y.end));
    let mut polygons = vec![polygon.clone()];

    let mut previous_row_polygon = polygon.clone();
    let mut bounding_box = bounding_rect(polygon.points.clone()).expect(NO_VERTICES_ERROR);

    while bounding_box.y.end > window.y.start {
        while bounding_box.x.start < window.x.end {
            polygon.align(&polygon.clone(), Direction::Right);
            polygons.push(polygon.clone());
            bounding_box = bounding_rect(polygon.points.clone()).expect(NO_VERTICES_ERROR);
        }
        polygon.align(&previous_row_polygon, Direction::Below);
        polygons.push(polygon.clone());
        bounding_box = bounding_rect(polygon.points.clone()).expect(NO_VERTICES_ERROR);
        previous_row_polygon = polygon.clone();
    }

    Model { polygons }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for (i, polygon) in model.polygons.iter().enumerate() {
        draw.polygon()
            .points(polygon.points.clone())
            .color(if i == model.polygons.len() / 2 {
                RED
            } else if i % 2 == 0 {
                LIGHTBLUE
            } else {
                LIGHTGREEN
            });
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

use crate::geometry::Polygon;
use nannou::prelude::*;

const ORIGINAL_WIDTH: f32 = 100.;
const ORIGINAL_SIDES: usize = 3;

pub struct Model {}

pub fn model(app: &App) -> Model {
    app.new_window().fullscreen().view(view).build().unwrap();

    let polygon = Polygon::new(ORIGINAL_WIDTH, ORIGINAL_SIDES);

    for _iteration in 0..5 {
        for _point in 0..(polygon.points.len() - 1) {}
    }

    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

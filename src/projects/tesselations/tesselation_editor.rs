use crate::geometry::Polygon;
use nannou::prelude::*;
use petgraph::graph::UnGraph;

const ORIGINAL_WIDTH: f32 = 100.;
const ORIGINAL_SIDES: usize = 3;

pub struct Model {}

pub fn model(app: &App) -> Model {
    app.new_window().fullscreen().view(view).build().unwrap();

    let mut polygon = Polygon::new(ORIGINAL_WIDTH, ORIGINAL_SIDES);

    for iteration in 0..5 {
        for point in 0..(polygon.points.len() - 1) {}
    }

    Model {}
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for i in 0..20 {}

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

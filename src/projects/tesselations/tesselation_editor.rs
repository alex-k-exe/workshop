use crate::geometry::Polygon;
use nannou::prelude::*;
use petgraph::graph::UnGraph;

const ORIGINAL_WIDTH: f32 = 100.;
const ORIGINAL_SIDES: usize = 3;

pub struct Model {
    graph: UnGraph<Polygon, i32>,
}

pub fn model(app: &App) -> Model {
    app.new_window().fullscreen().view(view).build().unwrap();
    let mut graph = UnGraph::new_undirected();
    graph.add_node(Polygon::new(ORIGINAL_WIDTH, ORIGINAL_SIDES));

    for i in 0..20 {}

    Model { graph }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for i in 0..20 {}

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

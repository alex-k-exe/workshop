use crate::geometry::{Direction, Polygon, NO_VERTICES_ERROR};
use geom::bounding_rect;
use nannou::prelude::*;

const ORIGINAL_WIDTH: f32 = 100.;

pub struct Model {
    polygon: Polygon,
}

pub fn model(app: &App) -> Model {
    app.new_window().fullscreen().view(view).build().unwrap();

    Model {
        polygon: Polygon::new(ORIGINAL_WIDTH, 4),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw();
    draw.background().color(WHITE);

    let polygon = model.polygon.clone();
    let bounding_box = bounding_rect(polygon.points.clone()).expect(NO_VERTICES_ERROR);

    draw.polygon().points(polygon.points.clone()).color(RED);

    second_align_thing(
        &window,
        &draw,
        &mut polygon.clone(),
        &mut bounding_box.clone(),
        &mut model.polygon.clone(),
        Direction::Right,
    );
    second_align_thing(
        &window,
        &draw,
        &mut polygon.clone(),
        &mut bounding_box.clone(),
        &mut model.polygon.clone(),
        Direction::Left,
    );

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

fn align_thing(
    draw: &Draw,
    polygon: &mut Polygon,
    bounding_box: &mut Rect,
    alignment_polygon: &Polygon,
    i: &mut i32,
    direction: &Direction,
) {
    println!("{:?}", polygon.clone());
    draw.polygon()
        .points(polygon.points.clone())
        .color(if *i % 2 == 0 { LIGHTBLUE } else { LIGHTGREEN });
    *bounding_box = bounding_rect(polygon.points.clone()).expect(NO_VERTICES_ERROR);
    *i = *i + 1;
    polygon.align(alignment_polygon, direction);
}

fn second_align_thing(
    window: &Rect,
    draw: &Draw,
    mut polygon: &mut Polygon,
    mut bounding_box: &mut Rect,
    alignment_polygon: &mut Polygon,
    direction: Direction,
) {
    let mut i = 0;
    while bounding_box.overlap(*window).is_some() {
        while bounding_box.y.start < window.y.end {
            let temp = polygon.clone();
            align_thing(
                &draw,
                &mut polygon,
                &mut bounding_box,
                &temp,
                &mut i,
                &Direction::Above,
            );
        }
        align_thing(
            &draw,
            &mut polygon,
            &mut bounding_box,
            &alignment_polygon,
            &mut i,
            &Direction::Below,
        );
        while bounding_box.y.end > window.y.start {
            let temp = polygon.clone();
            align_thing(
                &draw,
                &mut polygon,
                &mut bounding_box,
                &temp,
                &mut i,
                &Direction::Below,
            );
        }
        align_thing(
            &draw,
            &mut polygon,
            &mut bounding_box,
            &alignment_polygon,
            &mut i,
            &direction,
        );
        *alignment_polygon = polygon.clone();
    }
}

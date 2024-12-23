use nannou::geom::bounding_rect;
use nannou::prelude::*;

const ORIGINAL_WIDTH: f32 = 200.;

pub struct Model {
    polygon: Polygon,
}

pub fn model(app: &App) -> Model {
    app.new_window().fullscreen().view(view).build().unwrap();
    let window = app.window_rect();

    let rect = Rect::from_w_h(ORIGINAL_WIDTH, ORIGINAL_WIDTH).top_left_of(window);

    Model {
        polygon: Polygon::from_rect(rect),
    }
}

// fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
//     match button {
//         MouseButton::Left => {}
//         _ => return,
//     }
// }

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let window = app.window_rect();

    let hori_polygon_num = window.x() / ORIGINAL_WIDTH;
    let vert_polygon_num = window.y() / ORIGINAL_WIDTH;

    for i in 0..5 {
        let mut poly = model.polygon.clone();
        let bounding_box = bounding_rect(poly.points.clone());
        match bounding_box {
            None => break,
            Some(bound) => match bound.overlap(window) {
                None => break,
                Some(..) => poly.translate(vec2(i as f32 * bound.w(), 0.)),
            },
        };
        draw.polygon()
            .points(poly.points)
            .color(if i % 2 == 0 { LIGHTBLUE } else { LIGHTGREEN });
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

#[derive(Clone)]
struct Polygon {
    points: Vec<Point2>,
}

impl Polygon {
    fn from_rect(rect: Rect) -> Self {
        Self {
            points: vec![
                pt2(rect.x.start, rect.y.start),
                pt2(rect.x.end, rect.y.start),
                pt2(rect.x.end, rect.y.end),
                pt2(rect.x.start, rect.y.end),
            ],
        }
    }

    fn translate(&mut self, translation: Vec2) {
        for point in &mut self.points {
            point.x += translation.x;
            point.y += translation.y;
        }
    }
}

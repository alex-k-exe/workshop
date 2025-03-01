use crate::geometry::{angle_between_points, Polygon};
use nannou::prelude::*;

pub struct Model {
    boids: Vec<Boid>,
}

impl Model {
    fn new(window: &Rect) -> Self {
        let boids = (0..10).map(|_| Boid::new_random(window)).collect();

        Model { boids }
    }
}

struct Boid {
    position: Point2,
    direction: Vec2,
}

impl Boid {
    fn new_random(boundary: &Rect) -> Self {
        let position = pt2(
            random_range(boundary.x.start, boundary.x.end),
            random_range(boundary.x.start, boundary.x.end),
        );

        let angle = random_range(0., 2. * PI);

        Boid {
            position,
            direction: vec2(angle.sin(), angle.cos()),
        }
    }
}

pub fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();

    Model::new(&app.window_rect())
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for boid in &model.boids {
        let mut tri = Polygon::new(1., 3);
        tri.translate(boid.position);
        let angle = angle_between_points(boid.direction, pt2(0., 0.), pt2(0., 1.));

        if boid.direction.x < 0. {
            tri.rotate_around_point(boid.position, 2. * PI - angle);
        } else {
            tri.rotate_around_point(boid.position, angle);
        }

        draw.tri()
            .points(tri.points[0], tri.points[1], tri.points[2]);
        draw.line()
            .points(boid.position, boid.position + boid.direction);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

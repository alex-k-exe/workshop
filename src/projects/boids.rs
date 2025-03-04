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

#[derive(Clone)]
struct Boid {
    position: Point2,
    velocity: Vec2,
}

const MAX_SPEED: f32 = 5.;
const VISUAL_RANGE: f32 = 100.;

impl Boid {
    fn new_random(boundary: &Rect) -> Self {
        let position = pt2(
            random_range(boundary.x.start, boundary.x.end),
            random_range(boundary.y.start, boundary.y.end),
        );

        let angle = random_range(0., 2. * PI);

        Boid {
            position,
            velocity: vec2(angle.cos(), angle.sin()).normalize(),
        }
    }
}

pub fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();

    Model::new(&app.window_rect())
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    app.draw().reset();

    for boid in &model.boids {
        let mut tri = Polygon {
            points: vec![pt2(10., 0.), pt2(-5., 5.), pt2(-5., -5.)],
        };
        tri.translate(boid.position);
        let angle = angle_between_points(boid.velocity, pt2(0., 0.), pt2(1., 0.));

        if boid.velocity.y > 0. {
            if boid.velocity.x == 0. {
                tri.rotate(2. * PI - angle);
            } else {
                tri.rotate(angle);
            }
        } else {
            if boid.velocity.x == 0. {
                tri.rotate(angle);
            } else {
                tri.rotate(2. * PI - angle);
            }
        }

        draw.tri()
            .points(tri.points[0], tri.points[1], tri.points[2]);
        draw.line()
            .points(tri.points[0], tri.points[0] + boid.velocity)
            .color(RED);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    let boids = model.boids.clone();
    for boid in model.boids.iter_mut() {
        keep_within_bounds(boid, app.window_rect());
        coherence(boid, &boids);
        seperation(boid, &boids);
        alignment(boid, &boids);

        if boid.velocity.length() > MAX_SPEED {
            boid.velocity = boid.velocity.normalize() * MAX_SPEED;
        }
        boid.position += boid.velocity
    }
}

fn keep_within_bounds(boid: &mut Boid, window: Rect) {
    let margin = 10.;
    let turn_factor = 1.;

    if boid.position.x < window.x.start + margin {
        boid.velocity.x += turn_factor;
    } else if boid.position.x > window.x.end - margin {
        boid.velocity.x -= turn_factor
    }
    if boid.position.y < window.y.start + margin {
        boid.velocity.y += turn_factor;
    } else if boid.position.y > window.y.end - margin {
        boid.velocity.y -= turn_factor;
    }
}

// Find the center of mass of the other boids and adjust velocity slightly to
// point towards the center of mass.
fn coherence(boid: &mut Boid, boids: &Vec<Boid>) {
    // adjust velocity by this %
    let centering_factor: f32 = 0.005;
    let mut center_position = pt2(0., 0.);
    let mut num_neighbors = 0;

    for other_boid in boids {
        if boid.position.distance(other_boid.position) < VISUAL_RANGE {
            center_position += other_boid.position;
            num_neighbors += 1;
        }
    }

    if num_neighbors == 0 {
        return;
    }
    center_position /= num_neighbors as f32;
    boid.velocity += (center_position - boid.position) * centering_factor;
}

/** Move away from other boids that are too close to avoid colliding */
fn seperation(boid: &mut Boid, boids: &Vec<Boid>) {
    let min_distance = 20.; // The distance to stay away from other boids
    let avoid_factor: f32 = 0.05; // Adjust velocity by this %

    let mut move_boid = vec2(0., 0.);
    for other_boid in boids {
        if other_boid.position == boid.position && other_boid.velocity == boid.velocity {
            continue;
        }
        if boid.position.distance(other_boid.position) < min_distance {
            continue;
        }
        move_boid += boid.position - other_boid.position;
    }

    boid.velocity += move_boid * avoid_factor;
}

// Find the average velocity (speed and direction) of the other boids and
// adjust velocity slightly to match.
fn alignment(boid: &mut Boid, boids: &Vec<Boid>) {
    let matching_factor = 0.05; // Adjust by this % of average velocity

    let mut avg_velocity = vec2(0., 0.);
    let mut num_neighbours = 0;

    for other_boid in boids {
        if boid.position.distance(other_boid.position) < VISUAL_RANGE {
            avg_velocity += other_boid.velocity;
            num_neighbours += 1;
        }
    }

    if num_neighbours == 0 {
        return;
    }
    avg_velocity /= num_neighbours as f32;

    boid.velocity += (avg_velocity - boid.velocity) * matching_factor;
}

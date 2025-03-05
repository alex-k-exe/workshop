use crate::geometry::{angle_between_points, Polygon};
use nannou::prelude::*;
use nannou_egui::{
    egui::{self, Ui},
    Egui,
};

const MAX_SPEED: f32 = 5.;

pub struct Model {
    boids: Vec<Boid>,
    settings: Settings,
    unapplied_settings: Settings,
    egui: Egui,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let window_id = app
            .new_window()
            .view(view)
            .fullscreen()
            .raw_event(raw_window_event)
            .build()
            .unwrap();

        Model {
            boids: Model::new_boids(&app.window_rect()),
            settings: Settings::new(),
            unapplied_settings: Settings::new(),
            egui: Egui::from_window(&app.window(window_id).unwrap()),
        }
    }

    fn new_boids(window: &Rect) -> Vec<Boid> {
        (0..3).map(|_| Boid::new_random(window)).collect()
    }
}

#[derive(Clone, Copy)]
struct Settings {
    visual_range: f32,

    containment: f32,
    cohesion: f32,
    separation: f32,
    alignment: f32,

    paused: bool,
}

impl Settings {
    fn new() -> Self {
        Settings {
            visual_range: 100.,
            containment: 1.,
            cohesion: 0.5,
            separation: 0.5,
            alignment: 0.5,
            paused: false,
        }
    }
}

#[derive(Clone, Copy)]
struct Boid {
    position: Point2,
    velocity: Vec2,
}

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

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn create_ui(
    ui: &mut Ui,
    window: &Rect,
    boids: &mut Vec<Boid>,
    mut settings: Settings,
    mut unapplied_settings: Settings,
) {
    ui.label("Visual range of boids:");
    ui.add(egui::Slider::new(
        &mut unapplied_settings.visual_range,
        (10.)..=(1000.),
    ));

    ui.label("Containment factor:");
    ui.add(egui::Slider::new(
        &mut unapplied_settings.containment,
        0.1..=10.,
    ));

    ui.label("Cohesion factor:");
    ui.add(egui::Slider::new(
        &mut unapplied_settings.cohesion,
        0.1..=10.,
    ));

    ui.label("Separation factor:");
    ui.add(egui::Slider::new(
        &mut unapplied_settings.separation,
        0.1..=10.,
    ));

    ui.label("Alignment factor:");
    ui.add(egui::Slider::new(
        &mut unapplied_settings.alignment,
        0.1..=10.,
    ));

    let apply = ui.button("Apply changes").clicked();
    let reset = ui.button("Reset changes").clicked();
    let toggle_pause = ui
        .button(if settings.paused { "Resume" } else { "Pause" })
        .clicked();

    if apply {
        settings = unapplied_settings;
        *boids = Model::new_boids(window);
    } else if reset {
        settings = Settings::new();
        unapplied_settings = settings;
        *boids = Model::new_boids(window);
    } else if toggle_pause {
        settings.paused = !settings.paused;
    }
}

pub fn update(app: &App, model: &mut Model, update: Update) {
    model.egui.set_elapsed_time(update.since_start);
    let ctx = model.egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        create_ui(
            ui,
            &app.window_rect(),
            &mut model.boids,
            model.settings,
            model.unapplied_settings,
        );
    });

    if model.settings.paused {
        return;
    }

    let boids = model.boids.clone();
    for boid in model.boids.iter_mut() {
        containment(boid, app.window_rect(), model.settings.containment);
        cohesion(boid, &boids, model.settings);
        separation(boid, &boids, model.settings.separation);
        alignment(boid, &boids, model.settings);

        if boid.velocity.length() > MAX_SPEED {
            boid.velocity = boid.velocity.normalize() * MAX_SPEED;
        }
        boid.position += boid.velocity
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

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

fn containment(boid: &mut Boid, window: Rect, factor: f32) {
    let margin = 10.;

    if boid.position.x < window.x.start + margin {
        boid.velocity.x += factor;
    } else if boid.position.x > window.x.end - margin {
        boid.velocity.x -= factor
    }
    if boid.position.y < window.y.start + margin {
        boid.velocity.y += factor;
    } else if boid.position.y > window.y.end - margin {
        boid.velocity.y -= factor;
    }
}

/** Find the center of mass of the other boids and adjust velocity slightly to point towards the
center of mass. */
fn cohesion(boid: &mut Boid, boids: &Vec<Boid>, settings: Settings) {
    // adjust velocity by this %
    let mut center_position = pt2(0., 0.);
    let mut num_neighbors = 0;

    for other_boid in boids {
        if boid.position.distance(other_boid.position) < settings.visual_range {
            center_position += other_boid.position;
            num_neighbors += 1;
        }
    }

    if num_neighbors == 0 {
        return;
    }
    center_position /= num_neighbors as f32;
    boid.velocity += (center_position - boid.position) * settings.cohesion;
}

/** Move away from other boids that are too close to avoid colliding */
fn separation(boid: &mut Boid, boids: &Vec<Boid>, factor: f32) {
    let min_distance = 20.; // The distance to stay away from other boids

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

    boid.velocity += move_boid * factor;
}

// Find the average velocity (speed and direction) of the other boids and
// adjust velocity slightly to match.
fn alignment(boid: &mut Boid, boids: &Vec<Boid>, settings: Settings) {
    let mut avg_velocity = vec2(0., 0.);
    let mut num_neighbours = 0;

    for other_boid in boids {
        if boid.position.distance(other_boid.position) < settings.visual_range {
            avg_velocity += other_boid.velocity;
            num_neighbours += 1;
        }
    }

    if num_neighbours == 0 {
        return;
    }
    avg_velocity /= num_neighbours as f32;

    boid.velocity += (avg_velocity - boid.velocity) * settings.alignment;
}

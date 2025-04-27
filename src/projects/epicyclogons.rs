use crate::geometry::{angle_between_points, rotate_point, Direction, Polygon, NO_VERTICES_ERROR};
use geom::bounding_rect;
use nannou::prelude::*;
use nannou_egui::{
    egui::{self, Ui},
    Egui,
};
use std::cmp::Ordering;

const ROTATING_ANGLE: f32 = 0.5 * PI / 180.0;

struct State {
    fixed: Polygon,
    rotating: Polygon,
    // rotating_point is the point that rotating is rotating around
    rotating_point: (Point2, usize),
    // next_point is the vertex on the polygon that is next to touch the other polygon
    next_point_fixed: (Point2, usize),
    next_point_rotating: (Point2, usize),
    traced_path: Vec<Point2>,
    tracing_point: Point2,
    collisions_num: u32,
}

impl State {
    fn new(settings: &Settings) -> Self {
        let fixed = Polygon::new(settings.fixed_radius, settings.fixed_sides);
        let mut rotating = Polygon::new(settings.rotating_radius, settings.rotating_sides);

        rotating.align(&fixed, Direction::Above);

        let rotating_point;
        let next_point_fixed;
        let next_point_rotating;

        let bounding_boxes = [
            bounding_rect(fixed.points.clone()).unwrap(),
            bounding_rect(rotating.points.clone()).unwrap(),
        ];
        if fixed.points.len() % 2 == 0 {
            // if fixed is bigger than rotating
            if bounding_boxes[0].w() > bounding_boxes[1].w() {
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
            next_point_rotating = (
                rotating.points[rotating.points.len() / 4],
                rotating.points.len() / 4,
            );
        }

        let lowest_vertex_rotating = rotating
            .points
            .iter()
            .min_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(Ordering::Equal))
            .unwrap()
            .clone();

        State {
            fixed,
            rotating,
            rotating_point,
            next_point_fixed,
            next_point_rotating,
            traced_path: vec![],
            tracing_point: pt2(0., lowest_vertex_rotating.y),
            collisions_num: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Settings {
    fixed_radius: f32,
    rotating_radius: f32,
    fixed_sides: usize,
    rotating_sides: usize,
    /* if collisions_num is -1, animate the rolling from start
     * otherwise, perform that number of collisions but don't animate anything
     */
    collisions_num: i32,
    speed: u32,
}

impl Settings {
    fn new() -> Self {
        Settings {
            fixed_radius: 100.,
            rotating_radius: 100.,
            fixed_sides: 3,
            rotating_sides: 4,
            collisions_num: -1,
            speed: 4,
        }
    }
}

pub struct Model {
    state: State,
    settings: Settings,
    unapplied_settings: Settings,
    egui: Egui,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let window_id = app
            .new_window()
            .view(view)
            .raw_event(raw_window_event)
            .build()
            .unwrap();

        let settings = Settings::new();
        let state = State::new(&settings);

        Model {
            state,
            settings: settings.clone(),
            unapplied_settings: settings.clone(),
            egui: Egui::from_window(&*app.window(window_id).unwrap()),
        }
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

pub fn update(app: &App, model: &mut Model, update: Update) {
    model.egui.set_elapsed_time(update.since_start);
    let ctx = model.egui.begin_frame();

    let state = &mut model.state;
    let settings = &mut model.settings;
    let unapplied_settings = &mut model.unapplied_settings;

    let window = app.window_rect();
    egui::Window::new("Settings")
        .current_pos([window.x.len() / 10., window.y.len() / 4.])
        .show(&ctx, |ui| {
            create_ui(ui, state, settings, unapplied_settings);
        });

    if settings.collisions_num >= 0 {
        while (state.collisions_num as i32) < settings.collisions_num {
            rotate_things(state);
        }
    } else {
        for _ in 0..settings.speed {
            rotate_things(state);
        }
    }
}

fn create_ui(
    ui: &mut Ui,
    state: &mut State,
    settings: &mut Settings,
    unapplied_settings: &mut Settings,
) {
    ui.label("Radius of fixed polygon:");
    ui.add(egui::Slider::new(
        &mut unapplied_settings.fixed_radius,
        (40.)..=(300.),
    ));

    ui.label("Radius of rotating polygon:");
    ui.add(egui::Slider::new(
        &mut unapplied_settings.rotating_radius,
        (40.)..=(300.),
    ));

    ui.label("Sides of fixed polygon:");
    ui.add(egui::Slider::new(
        &mut unapplied_settings.fixed_sides,
        3..=10,
    ));

    ui.label("Sides of rotating polygon:");
    ui.add(egui::Slider::new(
        &mut unapplied_settings.rotating_sides,
        3..=10,
    ));

    ui.label("Collisions to do:");
    ui.add(egui::Slider::new(
        &mut unapplied_settings.collisions_num,
        -1..=1000,
    ));

    ui.label("Speed:");
    ui.add(egui::Slider::new(&mut unapplied_settings.speed, 1..=10));

    let apply = ui.button("Apply changes").clicked();
    let reset = ui.button("Reset changes").clicked();

    if apply {
        *settings = unapplied_settings.clone();
        *state = State::new(settings);
    } else if reset {
        *settings = Settings::new();
        *unapplied_settings = settings.clone();
        *state = State::new(&settings);
    }
}

fn rotate_things(state: &mut State) {
    let mut angle = angle_between_points(
        state.next_point_rotating.0,
        state.rotating_point.0,
        state.next_point_fixed.0,
    );

    if angle > ROTATING_ANGLE {
        angle = 2. * PI - ROTATING_ANGLE;
        state
            .rotating
            .rotate_around_point(state.rotating_point.0, angle);
        rotate_point(
            &mut state.tracing_point,
            state.rotating_point.0,
            angle.sin(),
            angle.cos(),
        );

        state.next_point_rotating.0 = state.rotating.points[state.next_point_rotating.1];
        state.traced_path.push(state.tracing_point);

        return;
    }

    state.collisions_num = state.collisions_num + 1;

    if state.rotating.distance_to_point(state.next_point_fixed.0)
        < state.fixed.distance_to_point(state.next_point_rotating.0)
    {
        state.rotating_point = state.next_point_fixed.clone();
        (state.next_point_fixed).1 = if (state.next_point_fixed).1 == state.fixed.points.len() - 1 {
            0
        } else {
            (state.next_point_fixed).1 + 1
        };
        (state.next_point_fixed).0 = state.fixed.points[(state.next_point_fixed).1];
        state.next_point_rotating.0 = state.rotating.points[state.next_point_rotating.1];
    } else {
        state.rotating_point = state.next_point_rotating.clone();
        (state.next_point_rotating).1 = if (state.next_point_rotating).1 == 0 {
            state.rotating.points.len() - 1
        } else {
            (state.next_point_rotating).1 - 1
        };
        (state.next_point_rotating).0 = state.rotating.points[(state.next_point_rotating).1];
        state.next_point_fixed.0 = state.fixed.points[state.next_point_fixed.1];
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let state = &model.state;
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.polygon()
        .points(state.fixed.points.clone())
        .color(LIGHTBLUE);
    draw.polygon()
        .points(state.rotating.points.clone())
        .color(LIGHTGREEN);
    if !state.traced_path.is_empty() {
        draw.polyline().points(state.traced_path.clone()).color(RED);
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

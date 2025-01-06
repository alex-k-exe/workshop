use crate::geometry::{angle_between_points, rotate_point, Polygon};
use geom::bounding_rect;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};
use std::cmp::Ordering;

const ROTATING_ANGLE: f32 = 5. * std::f32::consts::PI / 180.0;

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
    // TODO constructor
}

#[derive(Debug, Clone)]
struct Settings {
    fixed_radius: f32,
    rotating_radius: f32,
    fixed_sides: usize,
    rotating_sides: usize,
}

impl Settings {
    fn new() -> Self {
        Settings {
            fixed_radius: 100.,
            rotating_radius: 100.,
            fixed_sides: 3,
            rotating_sides: 4,
        }
    }

    fn reset(&mut self) {
        let default = Settings::new();
        self.fixed_radius = default.fixed_radius;
        self.rotating_radius = default.rotating_radius;
        self.fixed_sides = default.fixed_sides;
        self.rotating_sides = default.rotating_sides;
    }
}

pub struct Model {
    state: State,
    settings: Settings,
    unapplied_settings: Settings,
    egui: Egui,
}

pub fn model(app: &App) -> Model {
    let window_id = app.new_window().fullscreen().view(view).build().unwrap();

    let settings = Settings::new();

    let (fixed, rotating, fixed_is_bigger) = initialize_polygons(&settings);

    let rotating_point: (Point2, usize);
    let next_point_fixed: (Point2, usize);
    let next_point_rotating: (Point2, usize);

    if fixed.points.len() % 2 == 0 {
        // if fixed is bigger than rotating
        if fixed_is_bigger {
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
        .expect("Polygon should have vertices");

    Model {
        state: State {
            fixed,
            rotating: rotating.clone(),
            rotating_point,
            next_point_fixed,
            next_point_rotating,
            traced_path: vec![],
            tracing_point: pt2(0., lowest_vertex_rotating.y),
        },
        settings: settings.clone(),
        unapplied_settings: settings.clone(),
        egui: Egui::from_window(&app.window(window_id).unwrap()),
    }
}

pub(crate) fn update(_app: &App, model: &mut Model, update: Update) {
    let state = &mut model.state;
    let egui = &mut model.egui;
    let mut unapplied_settings = &mut model.unapplied_settings;
    let mut settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
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

        let apply = ui.button("Apply").clicked();
        let cancel = ui.button("Cancel").clicked();
        let reset = ui.button("Reset").clicked();

        if apply {
            settings = unapplied_settings;
        } else if cancel {
            unapplied_settings = settings;
        } else if reset {
            settings.reset();
            unapplied_settings.reset();
        }
    });

    let mut angle = angle_between_points(
        state.next_point_rotating.0,
        state.rotating_point.0,
        state.next_point_fixed.0,
    );

    if angle > ROTATING_ANGLE {
        angle = ROTATING_ANGLE;
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

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
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
}

fn initialize_polygons(settings: &Settings) -> (Polygon, Polygon, bool) {
    let fixed = Polygon::new(settings.fixed_radius, settings.fixed_sides);
    let mut rotating = Polygon::new(settings.rotating_radius, settings.rotating_sides);
    let bounding_boxes = [
        bounding_rect(fixed.points.clone()).expect("Polygon should have points"),
        bounding_rect(rotating.points.clone()).expect("Polygon should have points"),
    ];

    rotating.translate(vec2(
        0.,
        bounding_boxes[0].y.end - bounding_boxes[1].y.start,
    ));

    (
        fixed,
        rotating,
        bounding_boxes[0].w() > bounding_boxes[1].w(),
    )
}

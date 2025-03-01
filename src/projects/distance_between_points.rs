use nannou::prelude::*;
use nannou_egui::{egui, Egui};

pub struct Model {
    points: Vec<(Point2, f32)>,
    selected_function: Functions,
    egui: Egui,
}

const POINT_WIDTH: f32 = 5.;
const GREYSCALE: bool = false;

#[derive(PartialEq)]
enum Functions {
    Power,
    Rational,
    Log,
    Sqrt,
}

fn some_math_function(x: f32, function_name: &Functions) -> f32 {
    match function_name {
        Functions::Power => x.powi(2),
        Functions::Rational => 1. / x,
        Functions::Log => x.ln(),
        Functions::Sqrt => x.sqrt(),
    }
}

pub fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .raw_event(raw_window_event)
        .view(view)
        .build()
        .unwrap();
    let window = app.window_rect();

    let mut points: Vec<(Vec2, f32)> = vec![];
    let mut current_point = pt2(window.x.start, window.y.end);

    while current_point.y > window.y.start {
        points.push((current_point, 0.));

        if current_point[0] + POINT_WIDTH * 1.5 > window.x.end {
            current_point[0] = window.x.start;
            current_point[1] -= POINT_WIDTH;
        } else {
            current_point[0] += POINT_WIDTH;
        }
    }

    color_points(&mut points, &Functions::Power);
    Model {
        points,
        selected_function: Functions::Power,
        egui: Egui::from_window(&app.window(window_id).unwrap()),
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

pub(crate) fn update(_app: &App, model: &mut Model, update: Update) {
    model.egui.set_elapsed_time(update.since_start);
    let ctx = model.egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Function to use");
        ui.radio_value(&mut model.selected_function, Functions::Power, "x^2");
        ui.radio_value(&mut model.selected_function, Functions::Rational, "1/x");
        ui.radio_value(&mut model.selected_function, Functions::Log, "ln(x)");
        ui.radio_value(&mut model.selected_function, Functions::Sqrt, "sqrt(x)");

        let changed = ui.button("Change function").clicked();
        if changed {
            color_points(&mut model.points, &model.selected_function);
        }
    });
}

fn color_points(points: &mut Vec<(Vec2, f32)>, function_name: &Functions) {
    let mut max_distance: f32 = 0.;
    let mut min_distance: f32 = 99999.;
    for (point, value) in points {
        let distance = pt2(point.x, some_math_function(point.x, &function_name))
            .distance(pt2(point.y, some_math_function(point.y, &function_name)));

        if max_distance < distance {
            max_distance = distance;
        } else if min_distance > distance {
            min_distance = distance;
        }
        *value = clamp(
            map_range(
                distance,
                min_distance,
                max_distance,
                0.,
                if GREYSCALE { 1. } else { 255. },
            ),
            0.,
            if GREYSCALE { 1. } else { 255. },
        );
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for point in &model.points {
        if GREYSCALE {
            draw.rect()
                .xy(point.0)
                .wh(vec2(POINT_WIDTH, POINT_WIDTH))
                .rgb(point.1, point.1, point.1);
        } else {
            draw.rect()
                .xy(point.0)
                .wh(vec2(POINT_WIDTH, POINT_WIDTH))
                .hsv(point.1, 50., 50.);
        }
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

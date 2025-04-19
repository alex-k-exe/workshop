use nannou::prelude::*;

pub struct Model {
    primary_points: [Point2; 3],
    secondary_points: Vec<(Point2, f32)>,
    selected_point: i8,
}

impl Model {
    pub fn new(app: &App) -> Self {
        app.new_window()
            .view(view)
            .mouse_pressed(mouse_pressed)
            .build()
            .unwrap();

        let window_width = app.window_rect().w();
        let window_height = app.window_rect().h();

        let primary_points = [pt2(240., 63.), pt2(-127., 127.), pt2(-255., -96.)];

        let mut secondary_points: Vec<(Vec2, f32)> = vec![];
        let mut current_point = pt2(-window_width / 2., window_height / 2.);

        while current_point.y > -window_height / 2. {
            secondary_points.push((current_point, 0.));

            if current_point[0] + POINT_WIDTH * 1.5 > window_width / 2. {
                current_point[0] = -window_width / 2.;
                current_point[1] -= POINT_WIDTH;
            } else {
                current_point[0] += POINT_WIDTH;
            }
        }

        color_secondary_points(primary_points, &mut secondary_points);
        Model {
            primary_points,
            secondary_points,
            selected_point: -1,
        }
    }
}

const POINT_WIDTH: f32 = 5.;
const GREYSCALE: bool = false;

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {}
        _ => return,
    }
    let mouse_point = pt2(app.mouse.x, app.mouse.y);

    let mut pressed_point: i8 = 0;
    let width_height = vec2(15., 15.);
    for point in model.primary_points {
        let rect = Rect::from_xy_wh(point, width_height);
        if rect.contains(mouse_point) {
            break;
        }
        pressed_point += 1;
    }

    // if a primary point is pressed
    if pressed_point < model.primary_points.len() as i8 {
        if pressed_point == model.selected_point {
            model.selected_point = -1;
        } else {
            model.selected_point = pressed_point;
        }
        return;
    }

    if model.selected_point == -1 {
        return;
    }

    model.primary_points[model.selected_point as usize] = mouse_point;
    color_secondary_points(model.primary_points, &mut model.secondary_points);
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for point in &model.secondary_points {
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

    let mut index = 0;
    for point in model.primary_points {
        draw.rect()
            .xy(point)
            .wh(vec2(15., 15.))
            .color(if model.selected_point == index {
                if GREYSCALE {
                    RED
                } else {
                    WHITE
                }
            } else {
                if GREYSCALE {
                    BLUE
                } else {
                    BLACK
                }
            });
        index += 1;
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

pub fn color_secondary_points(
    primary_points: [Point2; 3],
    secondary_points: &mut Vec<(Vec2, f32)>,
) {
    // First pass: find min/max distances
    let (min_distance, max_distance) = {
        let mut min = f32::MAX;
        let mut max = f32::MIN;

        for (point, _) in secondary_points.iter() {
            let total_distance = primary_points
                .iter()
                .map(|p| point.distance(*p))
                .sum::<f32>()
                / primary_points.len() as f32;

            min = min.min(total_distance);
            max = max.max(total_distance);
        }

        (min, max)
    };

    for (point, color) in secondary_points.iter_mut() {
        let total_distance = primary_points
            .iter()
            .map(|p| point.distance(*p))
            .sum::<f32>()
            / primary_points.len() as f32;

        *color = clamp(
            map_range(
                total_distance,
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

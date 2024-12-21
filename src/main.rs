use nannou::prelude::*;

fn main() {
    nannou::app(model).run()
}

struct Model {
    primary_points: [Point2; 3],
    //TODO make secondary_points have fixed length
    secondary_points: Vec<(Point2, f32)>,
    selected_point: i8,
}

const POINT_WIDTH: f32 = 7.;

fn model(app: &App) -> Model {
    app.new_window()
        .fullscreen_with(Some(Fullscreen::Borderless(None)))
        .view(view)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    let window = app.window_rect();

    let primary_points = [pt2(240., 63.), pt2(-127., 127.), pt2(-255., -96.)];

    let mut secondary_points: Vec<(Vec2, f32)> = vec![];
    let mut current_point = pt2(-window.w() / 2., window.h() / 2.);

    while current_point.y > -window.h() / 2. {
        secondary_points.push((current_point, 0.));

        if current_point[0] + POINT_WIDTH * 1.5 > window.w() / 2. {
            current_point[0] = -window.w() / 2.;
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

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {}
        _ => return,
    }

    let mut pressed_point: i8 = 0;
    for point in model.primary_points {
        let rect = Rect::from_xy_wh(point, vec2(15., 15.));
        if rect.contains(pt2(app.mouse.x, app.mouse.y)) {
            break;
        }
        pressed_point += 1;
    }

    // if a point is pressed
    if pressed_point < model.primary_points.len() as i8 {
        // if point pressed is currently selected
        if model.selected_point == pressed_point {
            println!("0");
            model.selected_point = -1;
        } else {
            println!("10");
            model.selected_point = pressed_point;
        }
        return;
    }

    if model.selected_point < 0 {
        return;
    }
    println!("1 {}", model.primary_points[pressed_point as usize]);
    model.primary_points[pressed_point as usize] = pt2(app.mouse.x, app.mouse.y);
    println!("2 {}", model.primary_points[pressed_point as usize]);
    color_secondary_points(model.primary_points, &mut model.secondary_points);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for point in &model.secondary_points {
        draw.rect()
            .xy(point.0)
            .wh(vec2(POINT_WIDTH, POINT_WIDTH))
            .hsv(point.1, 50., 80.);
    }

    let mut index = 0;
    for point in model.primary_points {
        draw.rect()
            .xy(point)
            .wh(vec2(15., 15.))
            .color(if model.selected_point == index {
                WHITE
            } else {
                BLACK
            });

        index += 1;
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

fn color_secondary_points(primary_points: [Point2; 3], secondary_points: &mut Vec<(Vec2, f32)>) {
    let mut max_distance: f32 = 0.;
    for secondary in secondary_points {
        let mut total_distance: f32 = 0.;
        for primary in primary_points {
            total_distance += secondary.0.distance(primary);
        }
        total_distance /= primary_points.len() as f32;

        if max_distance < total_distance {
            max_distance = total_distance;
        }
        secondary.1 = clamp(
            map_range(total_distance, 0., max_distance, 0., 255.),
            0.,
            255.,
        );
    }
}

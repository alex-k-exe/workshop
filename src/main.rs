use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let window = app.window_rect();

    const POINT_WIDTH: f32 = 2.;
    let set_points = [pt2(3., 1.), pt2(-2., 2.), pt2(-4., -2.5), pt2(4., -3.)];

    let mut current_point = pt2(-window.w() / 2., window.h() / 2.);
    let mut points: Vec<(Vec2, f32)> = vec![];

    let mut max_value = 0.;
    while current_point.y > -window.h() / 2. {
        let mut total_distance: f32 = 0.;
        for red_point in set_points {
            total_distance += current_point.distance(red_point);
        }
        total_distance /= set_points.len() as f32;
        points.push((current_point, total_distance));
        if max_value < total_distance {
            max_value = total_distance;
        }

        if current_point[0] + POINT_WIDTH * 1.5 > window.w() / 2. {
            current_point[0] = -window.w() / 2.;
            current_point[1] -= POINT_WIDTH;
        } else {
            current_point[0] += POINT_WIDTH;
        }
    }

    for (point, mut value) in points {
        value = clamp(map_range(value, 0., max_value, 0., 255.), 0., 255.);
        draw.rect()
            .xy(point)
            .wh(vec2(POINT_WIDTH, POINT_WIDTH))
            .hsv(value, 50., 100.);
    }

    for point in set_points {
        let mapped_x = map_range(point[0], -10., 10., -window.w() / 2., window.w() / 2.);
        let mapped_y = map_range(point[1], -10., 10., -window.h() / 2., window.h() / 2.);

        draw.rect()
            .xy(pt2(mapped_x, mapped_y))
            .wh(vec2(10., 10.))
            .color(BLACK);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

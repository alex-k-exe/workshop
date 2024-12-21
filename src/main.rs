use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let window = app.window_rect();

    draw.background().color(WHITE);

    const POINT_WIDTH: f32 = 10.;
    let red_points = [vec2(3., 1.), vec2(-2., 2.), vec2(-4., -2.5), vec2(4., -3.)];

    for point in red_points {
        let mapped_x = map_range(point[0], -10., 10., -window.w() / 2., window.w() / 2.);
        let mapped_y = map_range(point[1], -10., 10., -window.h() / 2., window.h() / 2.);

        draw.rect()
            .xy(pt2(mapped_x, mapped_y))
            .wh(vec2(POINT_WIDTH, POINT_WIDTH))
            .color(RED);
    }

    let x_points_num = (window.w() / POINT_WIDTH).floor();
    let y_points_num = (window.h() / POINT_WIDTH).floor();

    let points_values: Vec<f32> = vec![0.; (x_points_num * y_points_num) as usize];
    let current_point = pt2(-window.w() / 2., window.h() / 2.);

    for mut value in points_values {
        let mut total_distance: f32 = 0.;
        for red_point in red_points {
            total_distance += current_point.distance(red_point);
        }
        value = total_distance;
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

use projects::three_points;

pub mod projects {
    pub mod digits_to_zero;
    pub mod three_points;
}

fn main() {
    nannou::app(three_points::model).run();
}

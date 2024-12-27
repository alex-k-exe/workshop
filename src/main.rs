use projects::epicycloids;

pub mod geometry;
pub mod projects {
    pub mod digits_to_zero;
    pub mod tesselations {
        pub mod tesselation_editor;
    }
    pub mod epicycloids;
    pub mod three_points;
}

fn main() {
    nannou::app(epicycloids::model).run();
}

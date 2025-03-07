use projects::epicycloids::{self, update};

pub mod geometry;
pub mod projects {
    pub mod digits_to_zero;
    pub mod tesselations {
        pub mod tesselation_editor;
    }
    pub mod epicycloids;
    // pub mod distance_between_points;
    // pub mod boids;
    pub mod three_points;
}

fn main() {
    nannou::app(epicycloids::Model::new).update(update).run();
}

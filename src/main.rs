use projects::distance_between_points;

pub mod geometry;
pub mod projects {
    pub mod digits_to_zero;
    // pub mod tesselations {
    //     pub mod tesselation_editor;
    // }
    // pub mod epicycloids;
    pub mod distance_between_points;
    // pub mod epicycloids;
    pub mod three_points;
}

fn main() {
    nannou::app(distance_between_points::model)
        .update(distance_between_points::update)
        .run();
}

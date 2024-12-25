use projects::tesselations::tesselation_editor;

pub mod geometry;
pub mod projects {
    pub mod digits_to_zero;
    pub mod tesselations {
        pub mod tesselation_editor;
    }
    pub mod three_points;
}

fn main() {
    nannou::app(tesselation_editor::model).run();
}

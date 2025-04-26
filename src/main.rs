use projects::severance::four_tempers::{update, Model};

pub mod geometry;
pub mod projects {
    pub mod severance {
        pub mod colors;
        // pub mod contour_lines;
        pub mod four_tempers;
        pub mod recursive_rotation;
    }
}

fn main() {
    nannou::app(Model::new).update(update).run();
}

use projects::recursive_rotation::Model;

pub mod geometry;
pub mod projects {
    pub mod digits_to_zero;
    pub mod tesselations {
        pub mod tesselation_editor;
    }
    // pub mod distance_between_points;
    // pub mod boids;
    pub mod epicyclogons;
    pub mod game_of_life;
    pub mod recursive_rotation;
    pub mod three_points;
}

fn main() {
    nannou::app(Model::new).run();
    // let mut input = String::new();

    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");
}

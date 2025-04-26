use super::colors;
use nannou::prelude::*;

const G: f32 = 9.81;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Temper {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
    color: Srgb<u8>,
}

impl Default for Temper {
    fn default() -> Self {
        Temper {
            position: pt2(0., 0.),
            velocity: vec2(0., 0.),
            acceleration: vec2(0., 0.),
            mass: 10.,
            color: WHITE,
        }
    }
}

impl Temper {
    fn new(position: Point2, color: Srgb<u8>) -> Self {
        Temper {
            position,
            color,
            ..Default::default()
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.mass)
            .color(self.color);
    }

    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.;
    }

    fn attract(&self, other_temper: Temper) -> Vec2 {
        // Calculate direction of force
        let force = self.position - other_temper.position;
        // Distance between objects
        // Limiting the distance to eliminate "extreme" results for very close or very far objects
        let distance = force.length().max(5.).min(25.);

        // Calculate gravitional force magnitude
        let strength = (G * self.mass * other_temper.mass) / distance.pow(2);
        // Get force vector --> magnitude * direction
        println!("1 {force} {distance} {strength} {0}", force.normalize());
        force.normalize() * strength
    }
}

pub struct Model {
    tempers: [Temper; 5],
}

impl Model {
    pub fn new(app: &App) -> Self {
        app.new_window()
            .size_pixels(640, 800)
            .view(view)
            .build()
            .unwrap();
        let quarter_screen_size = app.window_rect().wh() / 4.;

        Model {
            tempers: [
                Temper::new(
                    pt2(-quarter_screen_size.x, quarter_screen_size.y),
                    colors::RED,
                ),
                Temper::new(quarter_screen_size, colors::LIGHTBLUE),
                Temper::new(
                    pt2(quarter_screen_size.x, -quarter_screen_size.y),
                    colors::GREEN,
                ),
                Temper::new(-quarter_screen_size, colors::YELLOW),
                Temper::default(),
            ],
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(colors::DARKBLUE);

    for temper in &model.tempers {
        temper.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    let tempers = model.tempers.clone();
    for (i, temper) in model.tempers.iter_mut().enumerate() {
        for (j, other_temper) in tempers.iter().enumerate() {
            if i == j {
                continue;
            }
            temper.apply_force(other_temper.attract(*temper));
        }
        temper.update();
    }
}

use super::colors;
use itertools::Itertools;
use nannou::{geom::bounding_rect, prelude::*};

const G: f32 = 9.81;
const DEFAULT_TEMPER_MASS: f32 = 5.;

#[derive(Clone, Debug, PartialEq)]
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
            mass: DEFAULT_TEMPER_MASS,
            color: colors::SILVER,
        }
    }
}

impl Temper {
    fn new(color: Srgb<u8>, window: Rect) -> Self {
        Temper {
            position: pt2(
                random_range(window.x.start, window.x.end),
                random_range(window.y.start, window.y.end),
            ),
            color,
            ..Default::default()
        }
    }

    fn set_mass(&mut self, mass: f32) -> &mut Self {
        self.mass = mass;
        return self;
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
        let new_velocity = self.velocity + self.acceleration;
        let speed = new_velocity.length();
        if speed > 20. {
            self.velocity = new_velocity.normalize() * 15.;
        } else {
            self.velocity = new_velocity;
        }
        self.position += self.velocity;
        self.acceleration *= 0.;
    }

    fn attract(&self, other_temper: &Temper) -> Vec2 {
        // Calculate direction of force
        let force = self.position - other_temper.position;
        // Distance between objects
        // Limiting the distance to eliminate "extreme" results for very close or very far objects
        let distance = force.length().min(25.);

        // Calculate gravitional force magnitude
        let strength = (G * self.mass * other_temper.mass) / distance.pow(2);
        // Get force vector --> magnitude * direction
        force.normalize() * strength
    }
}

pub struct Model {
    tempers: [Temper; 5],
    show_bounding_box: bool,
    show_lines_to_fixed: bool,
    show_lines_to_other_tempers: bool,
}

impl Model {
    pub fn new(app: &App) -> Self {
        app.new_window()
            .size_pixels(640, 800)
            .view(view)
            .key_pressed(key_pressed)
            .key_released(key_released)
            .build()
            .unwrap();
        let window = app.window_rect();

        Model {
            tempers: Model::new_tempers(window),
            show_bounding_box: false,
            show_lines_to_fixed: false,
            show_lines_to_other_tempers: false,
        }
    }

    fn new_tempers(window: Rect) -> [Temper; 5] {
        [
            Temper::new(colors::RED, window),
            Temper::new(colors::LIGHTBLUE, window),
            Temper::new(colors::GREEN, window),
            Temper::new(colors::YELLOW, window),
            Temper::default().set_mass(20.).clone(),
        ]
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(colors::DARKBLUE);

    if model.show_bounding_box {
        let bounding_box =
            bounding_rect(model.tempers.clone().iter().map(|temper| temper.position)).unwrap();
        draw.rect()
            .xy(bounding_box.xy())
            .wh(bounding_box.wh())
            .no_fill()
            .stroke_color(colors::SILVER)
            .stroke_weight(5.);
    }
    if model.show_lines_to_fixed {
        for temper in &model.tempers {
            draw.line()
                .start(temper.position)
                .end(model.tempers[4].position)
                .color(temper.color)
                .stroke_weight(5.);
        }
    }
    if model.show_lines_to_other_tempers {
        let (_, tempers) = model
            .tempers
            .split_last()
            .expect("Tempers should not be empty");

        for combination in tempers.iter().combinations(2) {
            draw.line()
                .start(combination[0].position)
                .end(combination[1].position)
                .color(colors::SILVER)
                .stroke_weight(5.);
        }
    }

    for temper in &model.tempers {
        temper.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    let tempers = model.tempers.clone();
    for (i, temper) in model.tempers.iter_mut().enumerate() {
        if i == 4 {
            continue;
        }
        for (j, other_temper) in tempers.iter().enumerate() {
            if i == j {
                continue;
            }
            temper.apply_force(other_temper.attract(temper));
        }
        temper.update();
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::B => model.show_bounding_box = true,
        Key::F => model.show_lines_to_fixed = true,
        Key::O => model.show_lines_to_other_tempers = true,
        Key::R => model.tempers = Model::new_tempers(app.window_rect()),
        Key::Key1 => model.tempers[0].mass += 0.5,
        Key::Key2 => model.tempers[1].mass += 0.5,
        Key::Key3 => model.tempers[2].mass += 0.5,
        Key::Key4 => model.tempers[3].mass += 0.5,
        _ => return,
    }
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::B => model.show_bounding_box = false,
        Key::F => model.show_lines_to_fixed = false,
        Key::O => model.show_lines_to_other_tempers = false,
        Key::Key1 => model.tempers[0].mass = DEFAULT_TEMPER_MASS,
        Key::Key2 => model.tempers[1].mass = DEFAULT_TEMPER_MASS,
        Key::Key3 => model.tempers[2].mass = DEFAULT_TEMPER_MASS,
        Key::Key4 => model.tempers[3].mass = DEFAULT_TEMPER_MASS,
        _ => return,
    }
}

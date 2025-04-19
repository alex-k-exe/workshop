use nannou::{prelude::*, wgpu::Texture};

const IMAGE_NAME: &str = "ms_casey.jpg";
// assuming this is a power of 2
const IMAGE_WIDTH: u32 = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Tile {
    row: u32,
    column: u32,
}

pub struct Model {
    texture: Texture,
    recursion_layers: u32,
}

impl Model {
    pub fn new(app: &App) -> Self {
        app.new_window()
            .size(IMAGE_WIDTH, IMAGE_WIDTH)
            .size_pixels(IMAGE_WIDTH, IMAGE_WIDTH)
            .view(view)
            .fullscreen()
            .key_pressed(key_pressed)
            .build()
            .unwrap();

        let assets = app.assets_path().unwrap();
        let img_path = assets.join(IMAGE_NAME);
        let texture = wgpu::Texture::from_path(app, img_path).unwrap();

        Model {
            texture,
            recursion_layers: 3,
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if model.recursion_layers < 2 {
        draw.texture(&model.texture);
        draw.to_frame(app, &frame).unwrap();
        return;
    }

    let texture = &model.texture;
    let rows = (2 as u32).pow(model.recursion_layers - 1);
    let normalised_tile_width = 1 as f32 / rows as f32;
    let tile_width = normalised_tile_width * IMAGE_WIDTH as f32;

    for row in 0..rows {
        for column in 0..rows {
            let source_tile = source_tile(row, column);
            let area = Rect::from_xy_wh(
                relative_tile_position(source_tile.row, source_tile.column, tile_width)
                    / IMAGE_WIDTH as f32,
                vec2(normalised_tile_width, normalised_tile_width),
            );

            let destination_tile = relative_tile_position(row, column, tile_width)
                + vec2(IMAGE_WIDTH as f32 / -2., IMAGE_WIDTH as f32 / -2.);

            draw.texture(texture)
                .area(area)
                .xy(destination_tile)
                .w_h(tile_width, tile_width);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn relative_tile_position(row: u32, column: u32, tile_width: f32) -> Point2 {
    pt2(row as f32, column as f32) * tile_width + tile_width / 2.
}

/** Find the tile that should be translated into the position of the tile given through the parameters */
fn source_tile(row: u32, column: u32) -> Tile {
    match (row % 2 == 0, column % 2 == 0) {
        // top left
        (true, true) => Tile {
            row: row + 1,
            column,
        },
        // top right
        (true, false) => Tile {
            row,
            column: column - 1,
        },
        // bottom left
        (false, true) => Tile {
            row,
            column: column + 1,
        },
        // bottom right
        (false, false) => Tile {
            row: row - 1,
            column,
        },
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Right => model.recursion_layers += 1,
        Key::Left => model.recursion_layers -= 1,
        _ => {}
    }

    if model.recursion_layers < 1 || IMAGE_WIDTH / model.recursion_layers == 1 {
        model.recursion_layers = 1;
    }
}

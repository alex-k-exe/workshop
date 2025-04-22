use nannou::{
    image::{DynamicImage, GenericImageView, RgbaImage},
    prelude::*,
    wgpu::Texture,
};

use crate::geometry::PointU32;

const IMAGE_NAME: &str = "ms_casey.jpg";
// assuming this is a power of 2
const IMAGE_WIDTH: u32 = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Tile {
    row: u32,
    column: u32,
}

impl Tile {
    fn center_coords(&self, tile_width: u32) -> PointU32 {
        PointU32::new(self.row, self.column) * tile_width + tile_width / 2
    }

    /** Find the tile that should be translated into the position of the tile given through the parameters */
    fn source_tile(&self) -> Tile {
        match (self.row % 2 == 0, self.column % 2 == 0) {
            // top left
            (true, true) => Tile {
                row: self.row + 1,
                column: self.column,
            },
            // top right
            (true, false) => Tile {
                row: self.row,
                column: self.column - 1,
            },
            // bottom left
            (false, true) => Tile {
                row: self.row,
                column: self.column + 1,
            },
            // bottom right
            (false, false) => Tile {
                row: self.row - 1,
                column: self.column,
            },
        }
    }
}

pub struct Model {
    image: Texture,
    recursion_layers: u32,
}

impl Model {
    pub fn new(app: &App) -> Self {
        app.new_window()
            .size(IMAGE_WIDTH, IMAGE_WIDTH)
            .size_pixels(IMAGE_WIDTH, IMAGE_WIDTH)
            .view(view)
            .key_pressed(key_pressed)
            .build()
            .unwrap();

        let assets = app.assets_path().unwrap();
        let img_path = assets.join(IMAGE_NAME);

        Model {
            image: Texture::from_path(app, img_path).expect("Image should exist"),
            recursion_layers: 1,
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.texture(&model.image);
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Right => model.recursion_layers += 1,
        Key::Left => model.recursion_layers -= 1,
        _ => {}
    }

    if model.recursion_layers < 1 || IMAGE_WIDTH / model.recursion_layers <= 1 {
        model.recursion_layers = 1;
    }

    model.image = process_tiles(&model.image, model.recursion_layers);
}

fn process_tiles(old_image: &Texture, recursion_layers: u32) -> Texture {
    let rows = (2 as u32).pow(recursion_layers - 1);
    let tile_width = IMAGE_WIDTH / rows;

    let new_image = RgbaImage::new(IMAGE_WIDTH, IMAGE_WIDTH);

    for row in 0..rows {
        for column in 0..rows {
            let destination_tile = Tile { row, column };
            let source_tile = destination_tile.source_tile();

            let destination_tile_position = destination_tile.center_coords(tile_width).to_i32();
            let source_tile_position = source_tile.center_coords(tile_width).to_i32();
            let half_tile_width = (tile_width / 2) as i32;

            for x in -half_tile_width..half_tile_width {
                for y in -half_tile_width..half_tile_width {
                    let pixel = old_image.(
                        (x + source_tile_position[0]) as u32,
                        (y + source_tile_position[1]) as u32,
                    );
                    new_image.put_pixel(
                        (y + destination_tile_position[0]) as u32,
                        (y + destination_tile_position[1]) as u32,
                        pixel,
                    );
                }
            }
        }
    }

    new_image
}

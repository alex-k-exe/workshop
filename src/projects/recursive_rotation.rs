use crate::geometry::PointU32;
use nannou::{
    image::{DynamicImage, GenericImage, GenericImageView, RgbaImage},
    prelude::*,
    wgpu::Texture,
};

// assuming it's square and side length is a power of 2
const IMAGE_PATH: &str = "ms_casey.jpg";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Tile {
    row: u32,
    column: u32,
}

impl Tile {
    fn top_left_coords(&self, tile_width: u32) -> PointU32 {
        PointU32::new(self.row, self.column) * tile_width
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
    image: DynamicImage,
    recursion_layers: u32,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let image = make_image(app);

        app.new_window()
            .size(image.width(), image.width())
            .size_pixels(image.width(), image.width())
            .view(view)
            .key_pressed(key_pressed)
            .build()
            .unwrap();

        Model {
            image,
            recursion_layers: 1,
        }
    }
}

fn make_image(app: &App) -> DynamicImage {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join(IMAGE_PATH);
    let dynamic_img = nannou::image::io::Reader::open(img_path)
        .unwrap()
        .decode()
        .unwrap();
    dynamic_img
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background();
    draw.texture(&Texture::from_image(app, &model.image));
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Right => model.recursion_layers += 1,
        Key::Left => model.recursion_layers -= 1,
        _ => {}
    }

    if model.recursion_layers <= 1 || model.image.width() / model.recursion_layers <= 1 {
        model.recursion_layers = 1;
        model.image = make_image(app);
        return;
    }
    model.image = process_tiles(&model.image, model.recursion_layers);
}

fn process_tiles(old_image: &DynamicImage, recursion_layers: u32) -> DynamicImage {
    println!("1");
    let rows = 2u32.pow(recursion_layers - 1);
    let tile_width = old_image.width() / rows;

    let mut new_image =
        DynamicImage::ImageRgba8(RgbaImage::new(old_image.width(), old_image.width()));

    for row in 0..rows {
        for column in 0..rows {
            let destination_tile = Tile { row, column };
            let source_tile = destination_tile.source_tile();

            let destination_tile_position = destination_tile.top_left_coords(tile_width);
            let source_tile_position = source_tile.top_left_coords(tile_width);

            for x in 0..tile_width {
                for y in 0..tile_width {
                    let pixel =
                        old_image.get_pixel(x + source_tile_position.x, y + source_tile_position.y);
                    new_image.put_pixel(
                        y + destination_tile_position.x,
                        y + destination_tile_position.y,
                        pixel,
                    );
                }
            }
        }
    }
    println!("2");

    new_image
}

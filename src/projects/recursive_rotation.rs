use crate::geometry::PointU32;
use nannou::{
    image::{
        io::Reader,
        {DynamicImage, GenericImage, GenericImageView, RgbaImage},
    },
    prelude::*,
    wgpu::Texture,
};

// assuming the image is square and its side length is a power of 2
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

    /**
    Find the tile that should be translated into the position of the tile given through the parameters
    Works for both clockwise and anticlockwise translation
    */
    fn source_tile(&self, decreased_recursion_layers: bool) -> Tile {
        match (
            self.row % 2 == 0,
            self.column % 2 == 0,
            decreased_recursion_layers,
        ) {
            // top left increasing or top right decreasing
            (true, true, true) | (true, false, false) => Tile {
                row: self.row + 1,
                column: self.column,
            },
            // top right increasing or bottom right decreasing
            (true, false, true) | (false, false, false) => Tile {
                row: self.row,
                column: self.column - 1,
            },
            // bottom left increasing or top left decreasing
            (false, true, true) | (true, true, false) => Tile {
                row: self.row,
                column: self.column + 1,
            },
            // bottom right or bottom left decreasing
            (false, false, true) | (false, true, false) => Tile {
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

        if image.width() != image.height() {
            panic!(
                "Image width is {0} pixels and height is {1} pixels, but image should be square",
                image.width(),
                image.height()
            );
        }
        if !image.width().is_power_of_two() {
            panic!(
                "Image width is {0}, which is not a power of 2",
                image.width()
            )
        }

        app.new_window()
            .size(image.width(), image.width())
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
    Reader::open(img_path).unwrap().decode().unwrap()
}

fn find_number_of_rows(recursion_layers: u32) -> u32 {
    2u32.pow(recursion_layers.max(1) - 1)
}

fn find_number_of_layers(rows: u32) -> u32 {
    if rows == 0 {
        1
    } else {
        rows.next_power_of_two().trailing_zeros() + 1
    }
}

fn process_tiles(
    old_image: &DynamicImage,
    recursion_layers: u32,
    decreased_recursion_layers: bool,
) -> DynamicImage {
    let rows = find_number_of_rows(recursion_layers);
    let tile_width = old_image.width() / rows;

    let mut new_image =
        DynamicImage::ImageRgba8(RgbaImage::new(old_image.width(), old_image.width()));

    for row in 0..rows {
        for column in 0..rows {
            let destination_tile = Tile { row, column };
            let source_tile = destination_tile.source_tile(decreased_recursion_layers);

            let destination_tile_position = destination_tile.top_left_coords(tile_width);
            let source_tile_position = source_tile.top_left_coords(tile_width);

            for x in 0..tile_width {
                for y in 0..tile_width {
                    let pixel =
                        old_image.get_pixel(source_tile_position.x + x, source_tile_position.y + y);
                    new_image.put_pixel(
                        destination_tile_position.x + x,
                        destination_tile_position.y + y,
                        pixel,
                    );
                }
            }
        }
    }

    new_image
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.texture(&Texture::from_image(app, &model.image));
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Right => {
            model.recursion_layers += 1;
            if model.image.width() / find_number_of_rows(model.recursion_layers) <= 1 {
                model.recursion_layers = 2;
            }
            model.image = process_tiles(&model.image, model.recursion_layers, false);
        }
        Key::Left => {
            if model.recursion_layers == 1 {
                model.recursion_layers = find_number_of_layers(model.image.width()) - 1;
            }
            model.image = process_tiles(&model.image, model.recursion_layers, true);
            model.recursion_layers -= 1;
        }
        Key::Space => {
            model.recursion_layers = 2;
        }
        Key::R => {
            model.image = make_image(app);
            model.recursion_layers = 1;
        }
        _ => return,
    }
}

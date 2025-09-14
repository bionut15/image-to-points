use image_to_points::coord_iter;
use image_to_points::coord_push;
use image_to_points::iteration_coord;
use image_to_points::Iterate_path;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use visioncortex::CompoundPathElement;
use visioncortex::PathSimplifyMode;
use visioncortex::Point2;
use vtracer::{convert, ColorImage, ColorMode, Config, Hierarchical, SvgFile};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f64,
    z: f64,
    color: u8,
}


pub fn convert_to_json(img: ColorImage) -> Result<Value, Box<dyn std::error::Error>> {

    let svg_setup = Config {
        color_mode: ColorMode::Color,
        hierarchical: Hierarchical::Stacked,
        filter_speckle: 29,
        color_precision: 6,
        layer_difference: 25,
        mode: PathSimplifyMode::Polygon,
        corner_threshold: 60,
        length_threshold: 4.0,
        max_iterations: 5,
        splice_threshold: 45,
        path_precision: Some(2),
    };

    let img = convert_to_ColorImage(imgpath);
    let memory_svg = convert(img, svg_setup);
    let final_json_file = get_coordinates_from_svg(memory_svg);

    Ok(final_json_file?)
}

fn get_coordinates_from_svg(
    input_image: Result<SvgFile, String>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let svg_file = input_image?;
    let mut coords: Vec<Point> = Vec::new();

    for path in &svg_file.paths {
        let color = path.color;
        let avg = ((color.r as u32 + color.g as u32 + color.b as u32 + color.a as u32) / 4) as u8;
        let color_index_move = (avg % 6 + 1) as u8;
        let color_index_line = ((avg + 3) % 6 + 1) as u8;

        for element in path.path.iter() {
            Iterate_path!(element, coords, color_index_move, color_index_line);
        }
    }

    Ok(serde_json::to_value(&coords)?)
}

pub fn convert_to_ColorImage(path: &str) -> ColorImage {

}
fn write_to_json(_arg: &str) -> &str {
    todo!()
}

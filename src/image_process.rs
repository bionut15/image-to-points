                // PathCommand::MoveTo(x, z) => coords.push(Point {
                //     x: x * 0.5,
                //     z: z * 0.5,
                //     color: color_index,
                // }),
                // PathCommand::LineTo(x, z) => coords.push(Point {
                //     x: ((x.round() as i32).clamp(1, 6)) as f64,
                //     z: ((z.round() as i32).clamp(1, 6)) as f64,
                //     color: color_index,
                // }),
// }
//
// fn handle_path(attributes: &svg::node::Attributes) {
//     let stroke = attributes.get("stroke").map_or("none", |v| v);
//     if let Some(d_attr) = attributes.get("d") {
//         handle_path_coords(d_attr, stroke);
//     }
// }
//
// fn handle_path_coords(d_attr: &str, stroke: &str) {
//     if let Ok(data) = Data::parse(d_attr) {
//         for command in data.iter() {
//             match command {
//                 Command::Move(x, y) => print_point("Move", *x, *y, stroke),
//                 Command::Line(x, y) => print_point("Line", *x, *y, stroke),
//                 _ => {}
//             }
//         }
//     }
// }
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

fn convert_to_json(img: ColorImage) -> Result<Value, Box<dyn std::error::Error>> {
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

    let memory_svg = convert(img, svg_setup);
    get_coordinates_from_svg(memory_svg)
}

fn get_coordinates_from_svg(
    input_image: Result<SvgFile, String>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let svg_file = input_image?;
    let mut coords: Vec<Point> = Vec::new();

    for path in &svg_file.paths {
        let color = path.color;
        let avg = ((color.r as u32 + color.g as u32 + color.b as u32 + color.a as u32) / 4) as u8;
        let color_index = (avg % 6 + 1) as u8;

        // path.path is a CompoundPath
        for element in path.path.iter() {
            match element {
                CompoundPathElement::PathI32(path) => {
                    // Process PathI32
                    for point in path.iter() {
                        println!("PathI32 Point: ({}, {})", point.x, point.y);
                    }
                }
                CompoundPathElement::PathF64(path) => {
                    // Process PathF64
                    for point in path.iter() {
                        println!("PathF64 Point: ({}, {})", point.x, point.y);
                    }
                }
                CompoundPathElement::Spline(spline) => {
                    // Process Spline
                    for point in spline.points.iter() {
                        println!("Spline Point: ({}, {})", point.x, point.y);
                    }
                }
            }
        }
    }

    Ok(serde_json::to_value(&coords)?)
}

fn write_to_json(_arg: &str) -> &str {
    todo!()
}
fn post_to_cli(_arg: &str) -> &str {
    todo!()
}

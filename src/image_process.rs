use serde_json::json;
use usvg::{NodeKind, PathSegment};
use visioncortex::PathSimplifyMode;
use vtracer::{ColorImage, ColorMode, Config, Hierarchical, SvgFile, convert};

fn convert_to_json(img: ColorImage) {
    let temp_json: serde_json;

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

    get_coordinates_from_svg(memory_svg);
}

fn get_coordinates_from_svg(input_image: Result<SvgFile, String>) {
    /*
    delete the first two linesZZ
    get the x and y coordiantes
    if its M its .5 value and if its L  is fixed 1-6
    output  and json file
    */

    let mut points: Vec<(f64, f64, f64)> = Vec::new();
    let mut colors: Vec<usvg::Color> = Vec::new();
 

    for node in tree.root().descendants(
        if let NodeKind::Path(path) = &*node.borrow() {
            let color = path.fill.as_ref().map(|f| f.paint.to_color()).flatten();

            if let Some(c) = color {
                colors.push(c);
            }

            let z_val = if path
                .data
                .0
                .iter()
                .any(|s| matches!(s, PathSegment::MoveTo { .. }))
            {
                0.5
            } else {
                1.0            };

            for segment in &path.data.0 {
                match segment {
                    PathSegment::MoveTo { x, y } | PathSegment::LineTo { x, y } => {
                        points.push((*x, *y, z_val));
                    }
                    _ => {}
                }
            }
        }
    }
}

fn write_to_json(arg: &str) -> &str {
    todo!();
}
fn post_to_cli(arg: &str) -> &str {
    todo!();
}

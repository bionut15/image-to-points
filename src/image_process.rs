use visioncortex::PathSimplifyMode;
use vtracer::{ColorMode, Config, Hierarchical};

fn convert_to_json() {



    let SVG_Setup = Config {
        color_mode: ColorMode::Color,
        hierarchical: Hierarchical::Cutout,
        filter_speckle: 4,
        color_precision: 6,
        layer_difference: 16,
        mode: PathSimplifyMode::Polygon,
        corner_threshold: 60,
        length_threshold: 4.0,
        max_iterations: 5,
        splice_threshold: 45,
        path_precision: Some(2),
    };
}

fn parse_svg(arg: &str) -> &str {
    todo!();
}
fn write_to_json(arg: &str) -> &str {
    todo!();
}
fn post_to_cli(arg: &str) -> &str {
    todo!();
}

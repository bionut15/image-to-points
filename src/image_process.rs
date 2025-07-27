use regex::Regex;
use serde_json::json;
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

fn get_coordinates_from_svg(input_image: SvgFile) -> json!() {
    /*
    delete the first two lines
    get the x and y coordiantes
    if its M its .5 value and if its L  is fixed 1-6
    output  and json file
    */

    let mut reader = Reader::from_str(&svg_data);

    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut points = Vec::new();

    let re = Regex::new(
        r"(?P<cmd>[ML])\s*(?P<x>-?\d*\.?\d+)[,\s]*(?P<y>-?\d*\.?\d+)(?:[,\s]*(?P<z>-?\d*\.?\d+))?",
    )?;

    let mut color_map: HashMap<String, f32> = HashMap::new();
    let mut color_index = 1.0;
    let mut svg_width = 0.0;
    let mut svg_height = 0.0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) if e.name().as_ref() == b"path" => {
                let mut stroke_color = String::from("default");
                let mut d_attr_value = String::new();

                for attr in e.attributes() {
                    let attr = attr?;
                    let key = attr.key.as_ref();
                    let val = attr.unescape_value()?.into_owned();

                    match key {
                        b"d" => d_attr_value = val,
                        b"stroke" => stroke_color = val,
                        b"width" => svg_width = parse_svg_dimension(&val)?,
                        b"height" => svg_height = parse_svg_dimension(&val)?,
                        _ => {}
                    }
                }

                if d_attr_value.is_empty() {
                    continue;
                }

                let base_n = *color_map.entry(stroke_color.clone()).or_insert_with(|| {
                    let assigned = color_index;
                    color_index += 1.0;
                    if color_index > 6.0 {
                        color_index = 6.0;
                    }
                    assigned
                });

                for cap in re.captures_iter(&d_attr_value) {
                    let cmd = &cap["cmd"];
                    let x: f32 = cap["x"].parse()?;

                    let z: f32 = if let Some(z_match) = cap.name("z") {
                        z_match.as_str().parse()?
                    } else {
                        0.0
                    };

                    let n = match cmd {
                        "M" => (base_n + 0.5).min(6.0),
                        "L" => base_n,
                        _ => continue,
                    };
                    let canvas_width = 720.0;
                    let canvas_height = 500.0;

                    let offset_x = (canvas_width - svg_width) / 2.0;
                    let offset_z = (canvas_height - svg_height) / 2.0;

                    points.push(Point {
                        x: x + offset_x,
                        z: z + offset_z,
                        n,
                    });
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    let svg_path = Path::new(svg_path);
    let json_path = svg_path.with_extension("json");

    let json_str = serde_json::to_string_pretty(&points)?;
    fs::write(&json_path, json_str)?;
    println!("JSON saved to: {}", json_path.display());

    Ok(())
}

fn write_to_json(arg: &str) -> &str {
    todo!();
}
fn post_to_cli(arg: &str) -> &str {
    todo!();
}

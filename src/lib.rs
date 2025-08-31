#[macro_export]
macro_rules! coord_push {
    ($coords:expr, $point:expr, $color:expr) => {
        $coords.push(Point {
            x: $point.x as f64 * 0.5,
            z: $point.y as f64 * 0.5,
            color: $color,
        });
    };
}

#[macro_export]
macro_rules! iteration_coord {
    ($coords:expr, $point:expr, $i:expr, $color_move:expr, $color_line:expr) => {
        let color = if $i == 0 { $color_move } else { $color_line };
        coord_push!($coords, $point, color);
    };
}
#[macro_export]
macro_rules! coord_iter {
    ($path:expr, $coords:expr, $color_move:expr, $color_line:expr) => {{
        for (i, point) in $path.iter().enumerate() {
            iteration_coord!($coords, point, i, $color_move, $color_line);
        }
    }};
}

#[macro_export]
macro_rules! Iterate_path {
    ($element:expr, $coords:expr, $color_move:expr, $color_line:expr) => {
        match $element {
            CompoundPathElement::PathI32(path) => {
                coord_iter!(path, $coords, $color_move, $color_line);
            }
            CompoundPathElement::PathF64(path) => {
                coord_iter!(path, $coords, $color_move, $color_line);
            }
            CompoundPathElement::Spline(path) => {
                coord_iter!(path, $coords, $color_move, $color_line);
            }
        }
    };
}

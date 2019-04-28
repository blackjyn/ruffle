use crate::matrix::Matrix;
use std::collections::{HashMap, VecDeque};
use svg::node::element::{
    path::Data, Definitions, Image, LinearGradient, Path as SvgPath, Pattern, RadialGradient, Stop,
};
use svg::Document;
use swf::{Color, FillStyle, LineStyle, Shape};

pub fn swf_shape_to_svg(shape: &Shape) -> String {
    //let mut svg = String::new();
    let (width, height) = (
        shape.shape_bounds.x_max - shape.shape_bounds.x_min,
        shape.shape_bounds.y_max - shape.shape_bounds.y_min,
    );
    let mut document = Document::new()
        .set("width", width)
        .set("height", height)
        .set(
            "viewBox",
            (
                shape.shape_bounds.x_min,
                shape.shape_bounds.y_min,
                width,
                height,
            ),
        );

    let mut defs = Definitions::new();
    let mut num_defs = 0;

    let mut svg_paths = vec![];
    let (paths, strokes) = swf_shape_to_paths(shape);
    for path in paths {
        let mut svg_path = SvgPath::new();

        svg_path = svg_path.set(
            "fill",
            match path.fill_style {
                FillStyle::Color(Color { r, g, b, a }) => {
                    format!("rgba({},{},{},{})", r, g, b, f32::from(a) / 255.0)
                }
                FillStyle::LinearGradient(gradient) => {
                    let matrix = Matrix::from(gradient.matrix);
                    let shift = Matrix {
                        a: 1638.4 / width,
                        d: 1638.4 / height,
                        tx: -819.2,
                        ty: -819.2,
                        ..Default::default()
                    };
                    let gradient_matrix = matrix * shift;

                    let mut svg_gradient = LinearGradient::new()
                        .set("id", format!("f{}", num_defs))
                        .set("gradientUnits", "userSpaceOnUse")
                        .set(
                            "gradientTransform",
                            format!(
                                "matrix({} {} {} {} {} {})",
                                gradient_matrix.a,
                                gradient_matrix.b,
                                gradient_matrix.c,
                                gradient_matrix.d,
                                gradient_matrix.tx,
                                gradient_matrix.ty
                            ),
                        );
                    for record in &gradient.records {
                        let stop = Stop::new()
                            .set("offset", format!("{}%", f32::from(record.ratio) / 2.55))
                            .set(
                                "stop-color",
                                format!(
                                    "rgba({},{},{},{})",
                                    record.color.r,
                                    record.color.g,
                                    record.color.b,
                                    f32::from(record.color.a) / 255.0
                                ),
                            );
                        svg_gradient = svg_gradient.add(stop);
                    }
                    defs = defs.add(svg_gradient);

                    let fill_id = format!("url(#f{})", num_defs);
                    num_defs += 1;
                    fill_id
                }
                FillStyle::RadialGradient(gradient) => {
                    let matrix = Matrix::from(gradient.matrix);
                    let shift = Matrix {
                        a: 1638.4 / width,
                        d: 1638.4 / height,
                        tx: -819.2,
                        ty: -819.2,
                        ..Default::default()
                    };
                    let gradient_matrix = matrix * shift;

                    let mut svg_gradient = RadialGradient::new()
                        .set("id", format!("f{}", num_defs))
                        .set("gradientUnits", "userSpaceOnUse")
                        .set(
                            "gradientTransform",
                            format!(
                                "matrix({} {} {} {} {} {})",
                                gradient_matrix.a,
                                gradient_matrix.b,
                                gradient_matrix.c,
                                gradient_matrix.d,
                                gradient_matrix.tx,
                                gradient_matrix.ty
                            ),
                        );
                    for record in &gradient.records {
                        let stop = Stop::new()
                            .set("offset", format!("{}%", f32::from(record.ratio) / 2.55))
                            .set(
                                "stop-color",
                                format!(
                                    "rgba({},{},{},{})",
                                    record.color.r, record.color.g, record.color.b, record.color.a
                                ),
                            );
                        svg_gradient = svg_gradient.add(stop);
                    }
                    defs = defs.add(svg_gradient);

                    let fill_id = format!("url(#f{})", num_defs);
                    num_defs += 1;
                    fill_id
                }
                FillStyle::FocalGradient {
                    gradient,
                    focal_point,
                } => {
                    let matrix = Matrix::from(gradient.matrix);
                    let shift = Matrix {
                        a: 1638.4 / width,
                        d: 1638.4 / height,
                        tx: -819.2,
                        ty: -819.2,
                        ..Default::default()
                    };
                    let gradient_matrix = matrix * shift;

                    let mut svg_gradient = RadialGradient::new()
                        .set("id", format!("f{}", num_defs))
                        .set("fx", -focal_point)
                        .set("gradientUnits", "userSpaceOnUse")
                        .set(
                            "gradientTransform",
                            format!(
                                "matrix({} {} {} {} {} {})",
                                gradient_matrix.a,
                                gradient_matrix.b,
                                gradient_matrix.c,
                                gradient_matrix.d,
                                gradient_matrix.tx,
                                gradient_matrix.ty
                            ),
                        );
                    for record in &gradient.records {
                        let stop = Stop::new()
                            .set("offset", format!("{}%", f32::from(record.ratio) / 2.55))
                            .set(
                                "stop-color",
                                format!(
                                    "rgba({},{},{},{})",
                                    record.color.r, record.color.g, record.color.b, record.color.a
                                ),
                            );
                        svg_gradient = svg_gradient.add(stop);
                    }
                    defs = defs.add(svg_gradient);

                    let fill_id = format!("url(#f{})", num_defs);
                    num_defs += 1;
                    fill_id
                }
                FillStyle::Bitmap { .. } => {
                    let svg_image = Image::new(); // TODO: .set("xlink:href", "");

                    let svg_pattern = Pattern::new()
                        .set("id", format!("f{}", num_defs))
                        .add(svg_image);

                    defs = defs.add(svg_pattern);

                    let fill_id = format!("url(#f{})", num_defs);
                    num_defs += 1;
                    fill_id
                }
            },
        );

        let mut data = Data::new();
        for subpath in &path.subpaths {
            //svg_paths.push_str(&format!("M{} {}", subpath.start.0, subpath.start.1));
            data = data.move_to(subpath.start);

            for edge in &subpath.edges {
                match edge {
                    SubpathEdge::Straight(x, y) => {
                        data = data.line_to((*x, *y));
                    }
                    SubpathEdge::Bezier(cx, cy, ax, ay) => {
                        data = data.quadratic_curve_to((*cx, *cy, *ax, *ay));
                    }
                }
            }
        }

        svg_path = svg_path.set("d", data);
        svg_paths.push(svg_path);
    }

    for stroke in strokes {
        let mut svg_path = SvgPath::new();
        let line_style = stroke.line_style.unwrap();
        svg_path = svg_path
            .set("fill", "none")
            .set(
                "stroke",
                format!(
                    "rgba({},{},{},{})",
                    line_style.color.r, line_style.color.g, line_style.color.b, line_style.color.a
                ),
            )
            .set("width", line_style.width as f32 / 20.0);

        let mut data = Data::new();
        for subpath in &stroke.subpaths {
            data = data.move_to(subpath.start);

            for edge in &subpath.edges {
                match edge {
                    SubpathEdge::Straight(x, y) => {
                        data = data.line_to((*x, *y));
                    }
                    SubpathEdge::Bezier(cx, cy, ax, ay) => {
                        data = data.quadratic_curve_to((*cx, *cy, *ax, *ay));
                    }
                }
            }
        }

        svg_path = svg_path.set("d", data);
        svg_paths.push(svg_path);
    }

    if num_defs > 0 {
        document = document.add(defs);
    }

    for svg_path in svg_paths {
        document = document.add(svg_path);
    }

    document.to_string()
}

struct Stroke {
    path: Path,
    line_style: LineStyle,
}

// TODO(Herschel): Iterater-ize this.
pub fn swf_shape_to_paths(shape: &Shape) -> (Vec<Path>, Vec<Path>) {
    let mut layers = vec![];
    let mut paths = HashMap::<u32, Path>::new();
    let mut stroke_paths = HashMap::<u32, Path>::new();

    let mut x = 0f32;
    let mut y = 0f32;

    let mut fill_style_0 = 0;
    let mut fill_style_1 = 0;
    let mut line_style = 0;
    let mut fill_styles = &shape.styles.fill_styles;
    let mut line_styles = &shape.styles.line_styles;
    for record in &shape.shape {
        use swf::ShapeRecord::*;
        match record {
            StyleChange(style_change) => {
                if let Some((move_x, move_y)) = style_change.move_to {
                    x = move_x;
                    y = move_y;
                }

                if let Some(i) = style_change.fill_style_0 {
                    fill_style_0 = i;
                }

                if let Some(i) = style_change.fill_style_1 {
                    fill_style_1 = i;
                }

                if let Some(i) = style_change.line_style {
                    line_style = i;
                }

                if let Some(ref new_styles) = style_change.new_styles {
                    // TODO
                    layers.push((paths, stroke_paths));
                    paths = HashMap::new();
                    stroke_paths = HashMap::new();
                    fill_styles = &new_styles.fill_styles;
                    line_styles = &new_styles.line_styles;
                }
            }

            StraightEdge { delta_x, delta_y } => {
                if fill_style_0 != 0 {
                    let path = paths.entry(fill_style_0).or_insert_with(|| {
                        Path::new(fill_styles[fill_style_0 as usize - 1].clone())
                    });
                    path.add_edge((x + delta_x, y + delta_y), SubpathEdge::Straight(x, y));
                }

                if fill_style_1 != 0 {
                    let path = paths.entry(fill_style_1).or_insert_with(|| {
                        Path::new(fill_styles[fill_style_1 as usize - 1].clone())
                    });
                    path.add_edge((x, y), SubpathEdge::Straight(x + delta_x, y + delta_y));
                }

                if line_style != 0 {
                    let path = stroke_paths.entry(line_style).or_insert_with(|| {
                        Path::new_stroke(line_styles[line_style as usize - 1].clone())
                    });
                    path.add_edge((x, y), SubpathEdge::Straight(x + delta_x, y + delta_y));
                }

                x += delta_x;
                y += delta_y;
            }

            CurvedEdge {
                control_delta_x,
                control_delta_y,
                anchor_delta_x,
                anchor_delta_y,
            } => {
                if fill_style_0 != 0 {
                    let path = paths.entry(fill_style_0).or_insert_with(|| {
                        Path::new(fill_styles[fill_style_0 as usize - 1].clone())
                    });
                    path.add_edge(
                        (
                            x + control_delta_x + anchor_delta_x,
                            y + control_delta_y + anchor_delta_y,
                        ),
                        SubpathEdge::Bezier(x + control_delta_x, y + control_delta_y, x, y),
                    );
                }

                if fill_style_1 != 0 {
                    let path = paths.entry(fill_style_1).or_insert_with(|| {
                        Path::new(fill_styles[fill_style_1 as usize - 1].clone())
                    });
                    path.add_edge(
                        (x, y),
                        SubpathEdge::Bezier(
                            x + control_delta_x,
                            y + control_delta_y,
                            x + control_delta_x + anchor_delta_x,
                            y + control_delta_y + anchor_delta_y,
                        ),
                    );
                }

                if line_style != 0 {
                    let path = stroke_paths.entry(line_style).or_insert_with(|| {
                        Path::new_stroke(line_styles[line_style as usize - 1].clone())
                    });
                    path.add_edge(
                        (x, y),
                        SubpathEdge::Bezier(
                            x + control_delta_x,
                            y + control_delta_y,
                            x + control_delta_x + anchor_delta_x,
                            y + control_delta_y + anchor_delta_y,
                        ),
                    );
                }

                x += control_delta_x + anchor_delta_x;
                y += control_delta_y + anchor_delta_y;
            }
        }
    }

    layers.push((paths, stroke_paths));

    let mut out_paths = vec![];
    let mut out_strokes = vec![];
    for (paths, strokes) in layers {
        for (_, path) in paths {
            out_paths.push(path);
        }

        for (_, stroke) in strokes {
            out_strokes.push(stroke);
        }
    }

    (out_paths, out_strokes)
}

pub struct Path {
    fill_style: FillStyle,
    line_style: Option<LineStyle>,
    subpaths: Vec<Subpath>,
}

impl Path {
    fn new(fill_style: FillStyle) -> Path {
        Path {
            fill_style,
            line_style: None,
            subpaths: vec![],
        }
    }

    fn new_stroke(line_style: LineStyle) -> Path {
        Path {
            fill_style: FillStyle::Color(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            }),
            line_style: Some(line_style),
            subpaths: vec![],
        }
    }

    fn add_edge(&mut self, start: (f32, f32), edge: SubpathEdge) {
        let new_subpath = Subpath {
            start,
            end: match edge {
                SubpathEdge::Straight(x, y) => (x, y),
                SubpathEdge::Bezier(_cx, _cy, ax, ay) => (ax, ay),
            },

            edges: {
                let mut edges = VecDeque::new();
                edges.push_back(edge);
                edges
            },
        };

        self.merge_subpath(new_subpath);
    }

    fn merge_subpath(&mut self, mut subpath: Subpath) {
        fn approx_eq(a: (f32, f32), b: (f32, f32)) -> bool {
            let dx = a.0 - b.0;
            let dy = a.1 - b.1;
            const EPSILON: f32 = 0.0001;
            dx.abs() < EPSILON && dy.abs() < EPSILON
        }

        let mut subpath_index = None;
        for (i, other) in self.subpaths.iter_mut().enumerate() {
            if approx_eq(subpath.end, other.start) {
                other.start = subpath.start;
                for edge in subpath.edges.iter().rev() {
                    other.edges.push_front(*edge);
                }
                subpath_index = Some(i);
                break;
            }

            if approx_eq(other.end, subpath.start) {
                other.end = subpath.end;
                other.edges.append(&mut subpath.edges);

                subpath_index = Some(i);
                break;
            }
        }

        if let Some(i) = subpath_index {
            let subpath = self.subpaths.swap_remove(i);
            self.merge_subpath(subpath);
        } else {
            self.subpaths.push(subpath);
        }
    }
}

struct Subpath {
    start: (f32, f32),
    end: (f32, f32),

    edges: VecDeque<SubpathEdge>,
}

#[derive(Copy, Clone)]
enum SubpathEdge {
    Straight(f32, f32),
    Bezier(f32, f32, f32, f32),
}
use plotters::prelude::*;

pub fn main(map: &Vec<Vec<i32>>, name: &str) {
    let root = BitMapBackend::new(name, (640, 480)).into_drawing_area();
    root.fill(&WHITE).expect("fail to fill");
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption("Tabla de clasificación", ("san-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(0i32..3i32, 0i32..3i32)
        .expect("error at build chart");

    chart
        .configure_mesh()
        .x_labels(4)
        .y_labels(4)
        .max_light_lines(4)
        .x_label_offset(25)
        .y_label_offset(25)
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .expect("help");

    chart.draw_series(
        map.iter()
            .enumerate()
            .map(|(y, l)| l.iter().enumerate().map(move |(x, v)| (x as i32, y as i32, v)))
            .flatten()
            .map(|(x, y, v)| {
                Rectangle::new(
                    [(x, y), (x + 1, y + 1)],
                    HSLColor(
                        240.0 / 360.0 - 240.0 / 360.0 * (*v as f64 / 20.0),
                        0.7,
                        0.1 + 0.4 * *v as f64 / 20.0,
                    )
                    .filled(),
                )
            }),
    ).expect("help");

    root.present().expect("error at present root");
}

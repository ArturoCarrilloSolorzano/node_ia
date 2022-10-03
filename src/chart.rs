use plotters::{
    prelude::*,
    style::{
        full_palette::{BLACK, RED},
        Color,
    },
};

pub fn main(scatter_1: Vec<(f32, f32)>, scatter_2: Vec<(f32, f32)>, name: &str) {
    let root = BitMapBackend::new(name, (640, 480)).into_drawing_area();
    root.fill(&WHITE).expect("fail to fill");
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption("Test calisification", ("san-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)
        .expect("");

    chart
        .configure_mesh()
        .max_light_lines(3)
        .draw()
        .expect("help");

    chart.draw_series(PointSeries::of_element(scatter_1, 5, &BLUE, &|c, s, st| {
        return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
    })).expect("help");

    chart.draw_series(PointSeries::of_element(scatter_2, 5, &RED, &|c, s, st| {
        return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
    })).expect("help");

    root.present().expect("dfad");
}

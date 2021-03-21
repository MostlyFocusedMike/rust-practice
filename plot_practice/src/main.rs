use plotters::prelude::*;

fn same_color(color: plotters::style::RGBColor) -> plotters::style::RGBColor{
    color
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("./images/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(-102f32..102f32, 0f32..102f32)?;

    chart.configure_mesh().draw()?;

    let m = 0.03;
    let b = 89.0;
    chart
        .draw_series(LineSeries::new(
            (-100..=100)
                .map(|x_p| {
                    let x = x_p as f32;
                    (x, m * x + b)
                }),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));


    let m = -0.4;
    let b = 50.0;
    chart
        .draw_series(LineSeries::new(
            (-100..=100)
                .map(|x_p| {
                    let x = x_p as f32;
                    (x, m * x + b)
                }),
            &BLUE,
        ))?
        .label("b=50, m=-0.4")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
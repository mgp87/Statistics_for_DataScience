use plotters::prelude::*;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("images/histogram.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Histogram Plot", ("sans-serif", 50))
        .margin(5)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..5u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .draw()?;
    let data = make_data(20);
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.filled())
            .data(data.iter().map(|x: &u32| (*x, 1))),
    )?;

    root.present().expect("Unable to create file");
    Ok(())
}

fn make_data(elems: u32) -> Vec<u32> {
    let mut data = vec![];
    let mut rng = rand::thread_rng();
    for _i in 0..elems {
        data.push(rng.gen_range(0..11));
    }
    data
}

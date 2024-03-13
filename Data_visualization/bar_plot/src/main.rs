use plotters::prelude::*;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = make_data(10);
    let root_area = BitMapBackend::new("images/bar-plot.png", (800, 600)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .caption("Bar Plot Demo", ("sans-serif", 50))
        .build_cartesian_2d((0..9).into_segmented(), 0..55)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();
    ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
        let x0 = SegmentValue::Exact(x);
        let x1 = SegmentValue::Exact(x + 1);
        let mut bar = Rectangle::new([(x0, 0), (x1, *y)], RED.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();
    root_area.present()?;
    Ok(())
}

fn make_data(elems: i32) -> Vec<i32> {
    let mut data = vec![];
    let mut rng = rand::thread_rng();
    for _i in 0..elems {
        data.push(rng.gen_range(0..50));
    }
    return data;
}

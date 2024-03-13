use plotters::prelude::*;
use plotters::style::full_palette::ORANGE;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("images/pie-chart.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let title = TextStyle::from(("sans-serif", 30).into_font()).color(&(BLACK));
    root.titled("Pie Chart Demo", title)?;

    let dimensions = root.dim_in_pixel();
    let center = (dimensions.0 as i32 / 2, dimensions.1 as i32 / 2);
    let radius = 300.0;
    let sizes = vec![70.0, 35.0];
    let colors = vec![RED, YELLOW];
    let labels = vec!["Rust", "Python"];

    let mut pie = Pie::new(&center, &radius, &sizes, &colors, &labels);
    pie.start_angle(70.0);
    pie.label_style((("sans-serif", 50).into_font()).color(&(ORANGE)));
    pie.percentages((("sans-serif", radius * 0.06).into_font()).color(&BLACK));
    root.draw(&pie)?;
    root.present()?;
    Ok(())
}

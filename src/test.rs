#[cfg(test)]
mod tests {
    use plotters::backend::SVGBackend;
    use plotters::prelude::*;
    use std::time::Duration;

    #[test]
    fn test_plotter() {
        // let drawing_area = SVGBackend::new("tmp.png", (3840, 2160)).into_drawing_area();
        // drawing_area.fill(&WHITE).unwrap();
        // let mut builder = ChartBuilder::on(&drawing_area);
        // builder.margin(100).set_left_and_bottom_label_area_size(300);
        // 
        // let mut chart_context = builder
        //     .build_cartesian_2d(Duration::default()..Duration::from_secs(100), (10, 000))
        //     .unwrap();
        // 
        // chart_context
        //     .draw_series(LineSeries::new(
        //         Duration::default()..Duration::from_secs(1),
        //         &RED,
        //     ))
        //     .unwrap();
        // chart_context.configure_mesh().draw().unwrap();
    }
}

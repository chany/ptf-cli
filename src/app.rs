use crate::{
    compute::{f64_iter, is_converge},
    config::Config,
    errors::Errcode,
};
use num::complex::Complex64;
use plotters::{
    backend::BitMapBackend,
    chart::{ChartBuilder, LabelAreaPosition},
    drawing::IntoDrawingArea,
    element::Pixel,
    style::{BLACK, WHITE},
};

pub fn start(cfg: Config) -> Result<(), Errcode> {
    let (left, right, bottom, up) = (
        cfg.center.re - cfg.plot_range,
        cfg.center.re + cfg.plot_range,
        cfg.center.im - cfg.plot_range,
        cfg.center.im + cfg.plot_range,
    );
    let step = (cfg.plot_range * 2.0) / (cfg.resolution as f64);

    let drawing_area =
        BitMapBackend::new(&cfg.filename, (cfg.resolution, cfg.resolution)).into_drawing_area();
    let _ = drawing_area.fill(&WHITE);

    let Ok(mut ctx) = ChartBuilder::on(&drawing_area)
        .caption("Power Tower Function", ("san-serif", 20))
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .build_cartesian_2d(left..right, bottom..up)
    else {
        return Err(Errcode::PlottersError(1));
    };
    let _ = ctx.configure_mesh().disable_mesh().draw();

    let mut pixels = Vec::<(f64, f64)>::new();
    // pixels.reserve(cfg.resolution as usize);

    for r in f64_iter(left, right, step) {
        for i in f64_iter(bottom, up, step) {
            let c = Complex64 { re: r, im: i };
            if is_converge(c, cfg.max_iter, cfg.escape_radius) {
                pixels.push((r, i));
            } else {
                continue;
            }
        }
    }

    let _ = ctx.draw_series(pixels.iter().map(|x| Pixel::new(*x, &BLACK)));

    Ok(())
}

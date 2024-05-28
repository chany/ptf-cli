use num::complex::Complex64;
use std::path::PathBuf;

pub struct Config {
    pub center: Complex64,
    pub plot_range: f64,
    pub max_iter: usize,
    pub escape_radius: f64,
    pub filename: PathBuf,
    pub resolution: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            center: Complex64 { re: 0.0, im: 0.0 },
            plot_range: 5.0,
            max_iter: 500,
            escape_radius: 1e+10,
            filename: PathBuf::from(format!("./mytetration_x_{}_y_{}_eps_{}.png", 0, 0, 5)),
            resolution: 512,
        }
    }
}

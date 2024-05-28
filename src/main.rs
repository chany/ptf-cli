mod app;
mod cli;
mod compute;
mod config;
mod errors;

use config::Config;
use errors::exit_with_retcode;
use std::{path::PathBuf, process::exit};

fn main() {
    let mut cfg = Config::default();
    match cli::parse_args() {
        Ok(args) => {
            (cfg.center.re, cfg.center.im) = (args.center.0, args.center.1);
            cfg.plot_range = args.delta;
            cfg.max_iter = args.max_iter;
            cfg.escape_radius = args.escape_radius;
            cfg.filename = PathBuf::from(format!(
                "./mytetration_x_{}_y_{}_eps_{}.png",
                cfg.center.re, cfg.center.im, cfg.plot_range
            ));
            exit_with_retcode(app::start(cfg));
        }
        Err(e) => exit(e.get_retcode()),
    }
}

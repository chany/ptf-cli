use crate::errors::Errcode;
use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
pub struct Args {
    /// 대상 범위의 중앙좌표: 공백없이 실수부와 허수부를 콤마로 구분하여 입력 (예: -c -1.2,0)
    #[arg(short = 'c', long="center", default_value = "0.0,0.0", value_parser = parse_tuple, allow_hyphen_values=true) ]
    pub center: (f64, f64),

    /// 중앙좌표로부터 그릴 ± x 범위: ± y는 ± x와 같음
    #[arg(short = 'd', long = "delta", default_value_t = 5.0)]
    pub delta: f64,

    /// 해상도: 512, 1024, 2048, 4096, ...
    #[arg(short = 'r', long = "resolution", default_value_t = 512)]
    pub res: u32,

    /// 최대 몇층까지 계산할 것인지
    #[arg(short = 'i', long = "max_iter", default_value_t = 500)]
    pub max_iter: usize,

    /// 복소수 크기가 escape_radius를 벗어나면 발산한 것으로 처리
    #[arg(short = 'e', long = "escape", default_value_t = 1e+10)]
    pub escape_radius: f64,
}

// Parse (f64, f64) tuple
fn parse_tuple(s: &str) -> Result<(f64, f64), Box<dyn Error + Send + Sync + 'static>> {
    let mut iter = s.splitn(2, ',').map(|x| x.trim());
    let a = iter.next().ok_or("Invalid Tuple")?.parse()?;
    let b = iter.next().ok_or("Invalid Tuple")?.parse()?;

    Ok((a, b))
}

pub fn parse_args() -> Result<Args, Errcode> {
    let args = Args::parse();

    Ok(args)
}

use std::str::FromStr;
use structopt::StructOpt;

pub enum Colorscheme {
    Default,
    DefaultDark,
    Monokai,
    Solarized,
    Vice,
    Custom(String),
}

impl FromStr for Colorscheme {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "default" => Colorscheme::Default,
            "default-dark" => Colorscheme::DefaultDark,
            "monokai" => Colorscheme::Monokai,
            "solarized" => Colorscheme::Solarized,
            "Vice" => Colorscheme::Vice,
            _ => Colorscheme::Custom(s.to_string()),
        })
    }
}

#[derive(StructOpt)]
pub struct Args {
    /// Set a colorscheme
    #[structopt(short = "c", default_value = "default")]
    pub colorscheme: Colorscheme,

    /// Only show CPU, Mem, and Process widgets
    #[structopt(short = "m")]
    pub minimal: bool,

    /// Number of times per second to update CPU and Mem widgets
    #[structopt(short = "r", default_value = "1")]
    pub rate: f64,

    /// Show each CPU in the CPU widget
    #[structopt(short = "p")]
    pub percpu: bool,

    /// Show average CPU in the CPU widget
    #[structopt(short = "a")]
    pub averagecpu: bool,

    /// Show temperatures in fahrenheit
    #[structopt(short = "f")]
    pub fahrenheit: bool,

    /// Show a statusbar with the time
    #[structopt(short = "s")]
    pub statusbar: bool,

    /// Show battery level widget ('minimal' turns off)
    #[structopt(short = "b")]
    pub battery: bool,
}

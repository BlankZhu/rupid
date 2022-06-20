use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author = "BlankZhu",
    version = "0.1.0",
    about = "Rupid, a WIP rust gateway",
    long_about = None
)]
pub struct Option {
    /// config filename of Rupid
    #[clap(short, long, default_value = "config.yaml", value_parser)]
    pub config_filename: String,
}

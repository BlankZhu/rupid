use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "BlankZhu")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Option {
    #[clap(
        short,
        long,
        default_value = "config.yaml",
        about = "rupid's config filename"
    )]
    pub config_filename: String,
}

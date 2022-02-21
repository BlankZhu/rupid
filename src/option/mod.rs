use clap::Parser;

#[derive(Debug, Parser)]
// #[clap(version = "0.1", author = "BlankZhu")]
#[clap(
    author = "BlankZhu",
    version = "0.1.0",
    about = "rupid, a WIP API gateway"
)]
pub struct Option {
    #[clap(
        short,
        long,
        default_value = "config.yaml",
        // about = "rupid's config filename"
    )]
    pub config_filename: String,
}

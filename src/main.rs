#[macro_use]
extern crate rocket;

mod args;
mod gen;
mod utils;

use args::*;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = match args.file_source {
        #[cfg(feature = "google_drive")]
        FileSource::GoogleDrive => gen::google_drive()?,
    };

    println!("{}", config);
    Ok(())
}

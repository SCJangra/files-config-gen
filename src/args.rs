use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
    #[clap(arg_enum)]
    pub file_source: FileSource,
}

#[derive(Clone, Copy, Debug, ArgEnum)]
pub enum FileSource {
    #[cfg(feature = "google_drive")]
    GoogleDrive,
}

use sli::SliFile;
use structopt::StructOpt;

use std::path::{Path, PathBuf};
use std::fs;

#[derive(StructOpt)]
#[structopt(about = "A tool for converting between Smash Ultimate sound label info files and yaml")]
struct Args {
    in_file: PathBuf,
    out_file: PathBuf,

    #[structopt(short, long, help = "newline-separated hash labels to use")]
    labels: Option<PathBuf>,
}

fn main() {
    let args = Args::from_args();

    match SliFile::open(&args.in_file) {
        Ok(bgm_prop_file) => {
            let _ = sli::set_labels(
                args.labels.as_deref().unwrap_or(Path::new("tone_labels.txt"))
            );

            fs::write(&args.out_file, serde_yaml::to_string(&bgm_prop_file).unwrap()).unwrap();
        }
        Err(sli::Error::BadMagic { .. }) => {
            // Magic doesn't match, should be yaml file

            let contents = fs::read_to_string(&args.in_file).unwrap();
            let bgm_prop_file: SliFile = serde_yaml::from_str(&contents).unwrap();

            bgm_prop_file.save(&args.out_file).unwrap();
        },
        Err(err) => {
            // Another error occurred, magic matches but failed to parse
            eprintln!("An error occurred: {}", err);
        }
    }
}

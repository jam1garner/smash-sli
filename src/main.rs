use bgm_property::BgmPropertyFile;
use structopt::StructOpt;

use std::path::{Path, PathBuf};
use std::fs;

#[derive(StructOpt)]
struct Args {
    in_file: PathBuf,
    out_file: PathBuf,

    #[structopt(short, long)]
    labels: Option<PathBuf>,
}

fn main() {
    let args = Args::from_args();

    match BgmPropertyFile::open(&args.in_file) {
        Ok(bgm_prop_file) => {
            let _ = bgm_property::set_labels(
                args.labels.as_deref().unwrap_or(Path::new("bgm_hashes.txt"))
            );

            fs::write(&args.out_file, serde_yaml::to_string(&bgm_prop_file).unwrap()).unwrap();
        }
        Err(bgm_property::Error::BadMagic { .. }) => {
            // Magic doesn't match, should be yaml file

            let contents = fs::read_to_string(&args.in_file).unwrap();
            let bgm_prop_file: BgmPropertyFile = serde_yaml::from_str(&contents).unwrap();

            bgm_prop_file.save(&args.out_file).unwrap();
        },
        Err(err) => {
            // Another error occurred, magic matches but failed to parse
            eprintln!("An error occurred: {}", err);
        }
    }
}

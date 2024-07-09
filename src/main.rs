use clap::Parser;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, metadata},
    os::unix::fs::MetadataExt,
};
use walkdir::WalkDir;

/// Simple program to check file integrity
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to start from
    #[arg(short, long)]
    path: String,

    /// Max file size in MB
    #[arg(short, long, default_value_t = 1)]
    max_file_size: u64,

    /// Verbose
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let wd_entry = WalkDir::new(args.path)
    .into_iter()
    .filter_map(|e| e.ok());

    wd_entry.for_each(|wd_entry|{
        let md = match metadata(&wd_entry.path()) {
            Ok(entry) => entry,
            Err(e) => {
                println!("Error '{}'", e);
                return
            },
        };

        if md.is_dir() {
            return;
        }

        if args.max_file_size != 0 && md.size() > (args.max_file_size * 1024 * 1024) {
            if args.verbose {
                println!(
                    "Skipping '{}' due to size",
                    wd_entry.path().to_str().unwrap()
                );
            }
            return;
        }

        let mut hash = Sha256::new();

        let contents = fs::read(&wd_entry.path()).expect("Should have been able to read the file");
        hash.update(contents);
        let result = hash.finalize();
        let hex_result = hex::encode(result);

        if args.verbose {
            println!("{} {}", hex_result, wd_entry.path().to_str().unwrap());
        }
    });
}

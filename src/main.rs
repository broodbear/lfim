use clap::Parser;
use lfim::hash::hash;
use std::io::Result;
use walkdir::WalkDir;

pub mod lfim;

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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let wd_entry = WalkDir::new(args.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path_is_symlink())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.metadata().unwrap().len() < (args.max_file_size * 1024 * 1024));

    wd_entry.for_each(|wd_entry| {
        tokio::spawn(async move {
            let path = wd_entry.path().to_str().unwrap();

            let file_hash = match hash(path) {
                Ok(h) => h,
                Err(e) => {
                    println!("error, {}", e);
                    return
                }
            };

            if args.verbose {
                println!("{} {}", file_hash, path);
            }
        });
    });

    Ok(())
}

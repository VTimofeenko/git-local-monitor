use clap::Parser;
use std::path::Path;
use git2::Repository;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    path: String,
}


fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    println!("Checking '{}'...", path.display());

    let repo = Repository::open(path).expect("Failed to open a repo");
    println!("repo: {} found", repo.path().display())
}

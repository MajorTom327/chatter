use clap::{Parser};
use chatter::run;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    path: std::path::PathBuf,
    output: std::path::PathBuf,
}


fn main() {
  let args = Cli::parse();

  let path = args.path;
  let output = args.output;

  run(path, output);
}

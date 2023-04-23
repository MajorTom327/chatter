use clap::{Parser};
use chatter::run;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short('p'), long)]
    path: std::path::PathBuf,
    #[clap(short('o'), long)]
    output: std::path::PathBuf,

    #[clap(short('m'), long)]
    prompt_modifier: Option<String>
}


fn main() {
  let args = Cli::parse();

  let path = args.path;
  let output = args.output;
  let prompt_modifier = args.prompt_modifier;

  run(path, output, prompt_modifier);
}

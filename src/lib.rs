mod prompt;
mod reader;

use std::io::Write;
use std::fs::OpenOptions;

pub fn run(path: std::path::PathBuf, output: std::path::PathBuf, prompt_modifier: Option<String>, as_code: bool) {
  let directories = reader::get_directories(path);


  let output = OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("{}.jsonl", output.to_string_lossy().to_string()));

  let mut output = match output {
    Ok(file) => file,
    Err(err) => panic!("{}", err)
  };

  for directory in &directories {
    let prompt = reader::get_prompt(directory, &prompt_modifier, as_code).unwrap();

    let prompt_str = prompt.to_json();
    let prompt_str = prompt_str + "\n";
    let prompt_buffer: &[u8] = prompt_str.as_bytes();

    output.write(prompt_buffer).unwrap();
  }

  println!("All data as been writed to output");
}

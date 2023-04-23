use std::fs;
use std::fs::{File};
use std::io::{Read};
use crate::prompt::{Prompt};


pub fn get_directories(path: std::path::PathBuf) -> Vec<String> {

  let mut directories = Vec::new();

  if let Ok(entries) = fs::read_dir(path) {
      for entry in entries {
        if let Ok(entry) = entry {
          if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
              let path = entry.path();
              let _ = &directories.push(path.to_string_lossy().to_string());
            }
          }
        }
      }
    }
  directories
}

pub fn get_prompt(directory: &String) -> Result<Prompt, String> {

  let prompt = File::open(format!("{}/prompt.md", directory));
    let completion = File::open(format!("{}/completion.md", directory));

    let mut prompt = match prompt {
      Ok(file) => file,
      Err(err) => {
        eprintln!("Error: {}", err);
        return Err(format!("Error: {}", err));
      }
    };

    let mut completion = match completion {
      Ok(file) => file,
      Err(err) => {
        eprintln!("Error: {}", err);
        return Err(format!("Error: {}", err));
      }
    };

    let mut prompt_string = String::new();
    match prompt.read_to_string(&mut prompt_string) {
      Ok(content) => content,
      Err(e) => panic!("Error on read prompt: {:?}", e),
    };

    let mut completion_string = String::new();
    match completion.read_to_string(&mut completion_string) {
      Ok(content) => content,
      Err(e) => panic!("Error on read completion: {:?}", e),
    };

    let prompt = Prompt::new(prompt_string, completion_string);
    println!("Generation of {} is done.", directory);
    Ok(prompt)
}

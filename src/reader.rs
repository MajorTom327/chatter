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

pub fn get_prompt(directory: &String, prompt_modifier: &Option<String>, as_code: bool) -> Result<Prompt, String> {


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
    let prompt_string = format!("{}\n\n###\n\n", prompt_string);

    // Use the formatter to replace the {} with the prompt string and have a common way to modify the prompt
    let prompt_string = match prompt_modifier {
      Some(modifier) => {
        let temp_string = modifier.clone();
        temp_string.replace("{}", &prompt_string).replace("\n", "")
      },
      None => prompt_string
    };


    let mut completion_string = String::new();
    match completion.read_to_string(&mut completion_string) {
      Ok(content) => content,
      Err(e) => panic!("Error on read completion: {:?}", e),
    };

    let suffix = ".";

    if as_code {
      completion_string = format!(" ```json\n{}\n```{}", completion_string, suffix);
    } else {
      completion_string = format!(" {}{}", completion_string, suffix);
    }

    let prompt = Prompt::new(prompt_string, completion_string);
    println!("Generation of {} is done.", directory);
    Ok(prompt)
}

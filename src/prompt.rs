use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Prompt {
  pub prompt: String,
  pub completion: String,
}

impl Prompt {
  pub fn new(prompt: String, completion: String) -> Prompt {
    Prompt {
      prompt,
      completion
    }
  }

  pub fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

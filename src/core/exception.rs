use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NotFoundException {
  pub message: &'static str,
  pub status_code: u16,
}

impl Default for NotFoundException {
  fn default() -> Self {
    NotFoundException {
      message: "Not Found",
      status_code: 404,
    }
  }
}

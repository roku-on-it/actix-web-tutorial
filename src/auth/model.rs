use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginInput {
  pub email: String,
  pub password: String,
}

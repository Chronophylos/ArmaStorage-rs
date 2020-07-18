use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Value {
    String(String),
}

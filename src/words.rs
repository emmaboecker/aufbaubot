use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Deletion {
    pub keywords: Vec<String>,
    pub response: String
}
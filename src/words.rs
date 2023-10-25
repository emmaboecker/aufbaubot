use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Deletion {
    pub keywords: Vec<String>,
    pub response: String
}
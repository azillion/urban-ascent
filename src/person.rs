use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    id: u32,
    name: String,
    age: u8,
    health: u8,
    last_sleep: u8,
    groceries: u8,
}

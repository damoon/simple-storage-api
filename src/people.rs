use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Role {
    ADMIN,
    USER,
    GREENHORN,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub role: Role,
}

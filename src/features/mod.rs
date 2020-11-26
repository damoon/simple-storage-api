pub mod db;

use serde::{Deserialize, Serialize};

//#[strum(serialize_all = "shouty_snake_case")]
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

impl Person {
    #[allow(dead_code)]
    pub fn role_name(&self) -> String {
        let s_age = self.age.to_string();
        let ret = self.name.clone() + &s_age;
        ret
    }
}

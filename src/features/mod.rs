pub mod output;
pub mod tools;

use strum_macros::{Display, EnumString};

//#[strum(serialize_all = "shouty_snake_case")]
#[derive(Debug, PartialEq, EnumString, Display)]
pub enum Role {
    Admin,
    Greenhorn,
}

pub struct Person {
    pub name: String,
    pub age: u8,
    pub role: Role,
}

impl Person {
    pub fn role_name(&self) -> String {
        let s_age = self.age.to_string();
        //let mut ret = self.name.clone();
        let ret = self.name.clone() + &s_age;
        //ret.push_str(&s_age);
        ret
    }
}

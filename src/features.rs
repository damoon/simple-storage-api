pub mod output;
pub mod tools;

#[derive(Debug)]
pub enum Role {
    Admin,
    Greenhorn,
}

pub struct Person {
    pub name: String,
    pub age: i8,
    pub role: Role,
}

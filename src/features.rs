pub mod output;

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

pub fn reverse(pair: (i32, &str)) -> (&str, i32) {
    let (i, s) = pair;
    (s, i)
}

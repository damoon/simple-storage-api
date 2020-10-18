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

pub fn show_person(person: Person) {
    println!("{}: {}, {:?}", person.name, person.age, person.role);
}

pub fn do_foo(n: i8) {
    println!("{}", n);
}

#[allow(dead_code)]
fn list(slice: &[i32]) {
    println!("{0} {1}", slice.len(), slice[0]);
}

pub fn reverse(pair: (i32, &str)) -> (&str, i32) {
    let (i, s) = pair;
    (s, i)
}

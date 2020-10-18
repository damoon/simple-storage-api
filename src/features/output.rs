pub fn show_person(person: super::Person) {
    println!("{}: {}, {:?}", person.name, person.age, person.role);
}

pub fn do_foo(n: i8) {
    println!("{}", n);
}

#[allow(dead_code)]
fn list(slice: &[i32]) {
    println!("{0} {1}", slice.len(), slice[0]);
}

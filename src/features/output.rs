pub fn show_person(person: super::Person) {
    println!(
        "[person> name: {}: age:{}, role:{}",
        person.name, person.age, person.role
    );
}

#[allow(dead_code)]
fn list(slice: &[i32]) {
    println!("{0} {1}", slice.len(), slice[0]);
}

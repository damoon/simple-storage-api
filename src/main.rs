// use ferris_says::say;

// use std::io::{stdout, BufWriter};
//use features;
mod features;

#[allow(dead_code)]
fn main() {
    //println!("{} hello world", 44);
    //let some_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    //list(&some_numbers);

    let some_tuple = (1, "stefan", [1, 2, 3]);
    let rev = features::reverse((some_tuple.0, some_tuple.1));
    println!("{:?}", rev.1);

    features::do_foo(3);

    let p_stefan = features::Person {
        name: String::from("Stefan"),
        age: 42,
        role: features::Role::Admin,
    };

    let p_basti = features::Person {
        name: String::from("basti"),
        role: features::Role::Greenhorn,
        ..p_stefan
    };

    features::show_person(p_stefan);
    features::show_person(p_basti);

    // let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans 23!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();
}

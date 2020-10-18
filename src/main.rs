// use ferris_says::say;

// use std::io::{stdout, BufWriter};
//use features;

mod features;

use std::env;
use std::str::FromStr;

#[allow(dead_code)]
fn main() {
    //println!("{} hello world", 44);
    //let some_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    //list(&some_numbers);

    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let role = features::Role::from_str(&args[3]).unwrap();
    //    let role = features::roleFromString();

    let person = features::Person {
        name: args[1].clone(),
        age: args[2].parse::<u8>().unwrap(),
        role: role,
    };

    let db = features::db::get_database();
    let key: i32 = 1;
    let val: [u8; 3] = [1, 2, 3];

    features::db::store(&db, &key, &val);

    let res = features::db::read(&db, &key);

    println!("{:#?}", res);

    // features::tools::do_foo(3);

    // let p_stefan = features::Person {
    //     name: String::from("Stefan"),
    //     age: 42,
    //     role: features::Role::Admin,
    // };

    // let p_basti = features::Person {
    //     name: String::from("basti"),
    //     role: features::Role::Greenhorn,
    //     ..p_stefan
    // };

    features::output::show_person(person);
    // features::output::show_person(p_basti);

    // let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans 23!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();
}

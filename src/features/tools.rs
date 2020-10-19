#[allow(dead_code)]
pub fn reverse(pair: (i32, &str)) -> (&str, i32) {
    let (i, s) = pair;
    (s, i)
}

#[allow(dead_code)]
pub fn do_foo(n: i8) {
    println!("{}", n);
}

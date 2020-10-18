pub fn tuples() {
    let some_tuple = (1, "stefan", [1, 2, 3]);
    let rev = features::tools::reverse((some_tuple.0, some_tuple.1));
    println!("{:?}", rev.1);
}

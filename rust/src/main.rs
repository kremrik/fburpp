use fburpp::reader;

fn main() {
    let col_names = vec!["foo", "bar", "baz"];
    let col_types = vec!["str", "int", "str"];
    let fburppframe = reader(
        "/home/kyle/projects/fburpp/rust/example.csv",
        &col_names,
        &col_types,
    ).unwrap();
    println!("{:?}", fburppframe);
}

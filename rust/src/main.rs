use fburpp::reader;

fn main() {
    let schema = vec!["str", "int", "str"];
    let fburppframe = reader(&schema).unwrap();
    println!("{:?}", fburppframe);
}

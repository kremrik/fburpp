use fburpp::{reader, writer};

fn main() {
    let col_names = vec!["foo", "bar", "baz"];
    let col_types = vec!["str", "int", "str"];
    let fburppframe = reader(
        "/home/kyle/projects/fburpp/rust/example.csv",
        &col_names,
        &col_types,
    ).unwrap();
    
    let _ = writer(
        "/home/kyle/projects/fburpp/rust/example.out.csv",
        fburppframe
    );
}

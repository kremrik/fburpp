use fburpp::execute;

use serde_json::json;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let j = json!({
        "input_path": "/home/kyle/projects/fburpp/rust/example.csv",
        "output_path": "/home/kyle/projects/fburpp/rust/example.out.csv",
        "col_names": ["foo", "bar", "baz"],
        "col_types": ["str", "int", "str"],
        "select": ["foo", "bar"]
    });

    execute(&j.to_string()).unwrap();

    Ok(())
}

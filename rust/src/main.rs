use fburpp::execute;
use fburpp::job::{Job};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let j = r#"
    {
        "input_path": "/home/kyle/projects/fburpp/rust/example.csv",
        "structure": {
            "col_names": ["foo", "bar", "baz"],
            "col_types": ["str", "int", "str"]
        },
        "output_path": "/home/kyle/projects/fburpp/rust/example.out.csv",
        "select": {
            "fields": ["foo", "bar"]
        },
        "filter": [
            {
                "field": "bar",
                "comparator": ">",
                "value": "1"
            }
        ]
    }
    "#;

    // let job: Job = serde_json::from_str(j)?;
    // println!("{:?}", job.input_path);

    execute(&j.to_string()).unwrap();

    Ok(())
}

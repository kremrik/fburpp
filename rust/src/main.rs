use fburpp::read_write::CSV;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "/home/kyle/projects/fburpp/rust/example.csv";
    let output_path = "/home/kyle/projects/fburpp/rust/example.out.csv";
    let col_names = vec!["foo", "bar", "baz"];
    let col_types = vec!["str", "int", "str"];
    let select = vec!["foo", "bar"];

    let job = CSV {
        input_path,
        output_path,
        col_names,
        col_types,
        select,
    };

    job.run()?;

    Ok(())
}

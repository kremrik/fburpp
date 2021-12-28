// use fburpp::execute;
// use fburpp::job::{Job};
use fburpp::csv::{
    make_reader,
    make_writer,
    row_to_record,
    CsvRows,
};
use fburpp::data::{select};

use std::error::Error;

fn make_fake_job() -> String {
    r#"
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
    "#.to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "/home/kyle/projects/fburpp/rust/example.csv";
    let output_path = "/home/kyle/projects/fburpp/rust/example.out.csv";
    let col_names: Vec<String> = vec![
        String::from("foo"), 
        String::from("bar"), 
        String::from("baz")
    ];
    let col_types: Vec<String> = vec![
        String::from("str"), 
        String::from("int"), 
        String::from("str")
    ];

    let select_cols = vec![
        String::from("foo"),
        String::from("baz"),
    ];

    let mut reader = make_reader(&input_path);
    let mut writer = make_writer(&output_path);
    let csvrows = CsvRows::new(&mut reader, &col_names, &col_types);

    for row in csvrows {
        let sel_row = select(row, &select_cols);
        let record = row_to_record(sel_row);
        writer.write_record(record)?;
    }

    Ok(())
}

use fburpp::job::{Job};
use fburpp::{csv, json};
use fburpp::data::{select};

use serde_json;

use std::error::Error;
use std::io::{prelude::*};

fn make_fake_job() -> Job {
    let jstr = r#"
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
    "#.to_string();

    let j: Job = serde_json::from_str(&jstr).unwrap();
    return j
}

fn test_csv() -> Result<(), Box<dyn Error>> {
    let job = make_fake_job();

    let mut reader = csv::make_reader(&job.input_path);
    // let mut writer = csv::make_writer(&job.output_path);
    let mut writer = json::make_writer(&job.output_path);
    let csvrows = csv::CsvRows::new(
        &mut reader,
        &job.structure.col_names,
        &job.structure.col_types
    );

    let sel = job.select;

    for row in csvrows {
        let sel_row = select(row, &sel);
        // let record = csv::row_to_record(sel_row);
        // writer.write_record(record)?;
        let record = json::row_to_object(sel_row);
        writer.write_all(record.as_bytes())?;
    }

    Ok(())
}

fn main() {
    test_csv().unwrap();
}

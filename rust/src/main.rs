#![allow(dead_code)]

use fburpp::job::{Job};
use fburpp::{csv, json};
use fburpp::data::{select};

use serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::io::{prelude::*};

fn fake_job() -> Job {
    let jstr = r#"
    {
        "input_path": "/home/kyle/projects/fburpp/rust/example.csv",
        "file_type": "csv",
        "schema": {
            "foo": "str",
            "bar": "int",
            "baz": "str"
        },
        "output_path": "/home/kyle/projects/fburpp/rust/csv_to_csv.out.csv",
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


fn test_job() -> Result<(), Box<dyn Error>> {
    let job = fake_job();

    let mut reader = csv::make_reader(&job.input_path);
    let mut writer = csv::make_writer(&job.output_path);
    let csvrows = csv::CsvRows::new(
        &mut reader,
        &job.schema,
    );

    let sel = job.select;

    for row in csvrows {
        let sel_row = select(row, &sel);
        let record = csv::row_to_record(sel_row);
        writer.write_record(record)?;
    }

    Ok(())
}

fn main() {
    test_job().unwrap();
}

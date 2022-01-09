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

fn main() -> Result<(), Box<dyn Error>> {
    let mut job = fake_job();
    job.execute()?;
    Ok(())
}

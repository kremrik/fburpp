#![allow(dead_code)]

use fburpp::job::{Job};
use fburpp::{csv, job, json};
use fburpp::data::{select};

use serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::io::{prelude::*};

fn fake_job() -> Job {
    let jstr = r#"
    {
        "input_path": "/home/kyle/projects/fburpp/rust/example.csv",
        "input_type": "csv",
        "schema": {
            "csv": {
                "col_names": ["foo", "bar", "baz"],
                "col_types": ["str", "int", "str"]
            }
        },
        "output_path": "/home/kyle/projects/fburpp/rust/csv_to_csv.out.csv",
        "output_type": "csv",
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
    let mut job = fake_job();
    job.execute();
    Ok(())
}

fn main() {
    test_job().unwrap()
}

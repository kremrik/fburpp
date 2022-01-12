#![allow(dead_code)]

use fburpp::job::{Job};
use fburpp::{csv, job, json};
use fburpp::data::{select};

use serde_json;

use std::error::Error;

fn job_csv_to_csv() -> Job {
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
        }
    }
    "#.to_string();

    let j: Job = serde_json::from_str(&jstr).unwrap();
    return j
}

fn job_json_to_csv() -> Job {
    let jstr = r#"
    {
        "input_path": "/home/kyle/projects/fburpp/rust/example.json",
        "input_type": "json",
        "schema": {
            "json": {
                "schema": {
                    "foo": "str",
                    "bar": "int",
                    "baz": "str"
                }
            }
        },
        "output_path": "/home/kyle/projects/fburpp/rust/json_to_csv.out.csv",
        "output_type": "csv",
        "select": {
            "fields": ["foo", "bar"]
        }
    }
    "#.to_string();

    let j: Job = serde_json::from_str(&jstr).unwrap();
    return j
}

fn job_csv_to_json() -> Job {
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
        "output_path": "/home/kyle/projects/fburpp/rust/csv_to_json.out.csv",
        "output_type": "json",
        "select": {
            "fields": ["foo", "bar"]
        }
    }
    "#.to_string();

    let j: Job = serde_json::from_str(&jstr).unwrap();
    return j
}

fn test_job() -> Result<(), Box<dyn Error>> {
    let mut csv_to_csv = job_csv_to_csv();
    let mut json_to_csv = job_json_to_csv();
    let mut csv_to_json = job_csv_to_json();

    csv_to_csv.execute();
    json_to_csv.execute();
    csv_to_json.execute();

    Ok(())
}

fn main() {
    test_job().unwrap()
}

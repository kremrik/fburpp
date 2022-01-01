#![allow(dead_code)]

use fburpp::job::{Job};
use fburpp::{csv, json};
use fburpp::data::{select};

use serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::io::{prelude::*};

fn fake_job_csv_to_csv() -> Job {
    let jstr = r#"
    {
        "input_path": "/home/kyle/projects/fburpp/rust/example.csv",
        "structure": {
            "col_names": ["foo", "bar", "baz"],
            "col_types": ["str", "int", "str"]
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

fn fake_job_csv_to_json() -> Job {
    let jstr = r#"
    {
        "input_path": "/home/kyle/projects/fburpp/rust/example.csv",
        "structure": {
            "col_names": ["foo", "bar", "baz"],
            "col_types": ["str", "int", "str"]
        },
        "output_path": "/home/kyle/projects/fburpp/rust/csv_to_json.out.json",
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

fn fake_job_json_to_csv() -> Job {
    let jstr = r#"
    {
        "input_path": "/home/kyle/projects/fburpp/rust/example.json",
        "structure": {
            "col_names": ["foo", "bar", "baz"],
            "col_types": ["str", "int", "str"]
        },
        "output_path": "/home/kyle/projects/fburpp/rust/json_to_csv.out.csv",
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

fn test_csv_to_csv() -> Result<(), Box<dyn Error>> {
    let job = fake_job_csv_to_csv();

    let mut reader = csv::make_reader(&job.input_path);
    let mut writer = csv::make_writer(&job.output_path);
    let csvrows = csv::CsvRows::new(
        &mut reader,
        &job.structure.col_names,
        &job.structure.col_types
    );

    let sel = job.select;

    for row in csvrows {
        let sel_row = select(row, &sel);
        let record = csv::row_to_record(sel_row);
        writer.write_record(record)?;
    }

    Ok(())
}

fn test_csv_to_json() -> Result<(), Box<dyn Error>> {
    let job = fake_job_csv_to_csv();

    let mut reader = csv::make_reader(&job.input_path);
    let mut writer = json::make_writer(&job.output_path);
    let csvrows = csv::CsvRows::new(
        &mut reader,
        &job.structure.col_names,
        &job.structure.col_types
    );

    let sel = job.select;

    for row in csvrows {
        let sel_row = select(row, &sel);
        let record = json::row_to_object(sel_row);
        writer.write_all(record.as_bytes())?;
    }

    Ok(())
}

fn test_json_to_csv() -> Result<(), Box<dyn Error>> {
    let job = fake_job_json_to_csv();
    let mut schema: HashMap<String, String> = HashMap::new();
    schema.insert("foo".to_string(), "str".to_string());
    schema.insert("bar".to_string(), "int".to_string());
    schema.insert("baz".to_string(), "str".to_string());

    let reader = json::make_reader(&job.input_path);
    let mut writer = csv::make_writer(&job.output_path);
    let jsonrows = json::JsonRows::new(
        reader,
        schema,
    );

    let sel = job.select;

    for row in jsonrows {
        let sel_row = select(row, &sel);
        let record = csv::row_to_record(sel_row);
        writer.write_record(record)?;
    }

    Ok(())
}

fn main() {
    test_json_to_csv().unwrap();
}

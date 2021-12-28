use fburpp::job::{Job};
use fburpp::csv::{
    make_reader,
    make_writer,
    row_to_record,
    CsvRows,
};
use fburpp::data::{select};

use serde_json;

use std::error::Error;

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

    let mut reader = make_reader(&job.input_path);
    let mut writer = make_writer(&job.output_path);
    let csvrows = CsvRows::new(
        &mut reader,
        &job.structure.col_names,
        &job.structure.col_types
    );

    let sel = job.select;

    for row in csvrows {
        let sel_row = select(row, &sel);
        let record = row_to_record(sel_row);
        writer.write_record(record)?;
    }

    Ok(())
}

fn main() {
    test_csv().unwrap();
}

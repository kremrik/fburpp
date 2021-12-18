use crate::read_write::{CSV};

use serde::{Deserialize, Serialize};
use serde_json::Result;

// This whole thing is terrible and only exists to get
// things started

#[derive(Serialize, Deserialize)]
pub struct Job {
    input_path: String,
    output_path: String,
    col_names: Vec<String>,
    col_types: Vec<String>,
    select: Vec<String>,
}

pub fn execute(json: &str) -> Result<()> {
    let job = serde_json::from_str(json)?;
    dispatch_job(job)?;
    Ok(())
}

fn dispatch_job(job: Job) -> Result<()> {
    let csv_job = CSV {
        input_path: &job.input_path,
        output_path: &job.output_path,
        col_names: &job.col_names,
        col_types: &job.col_types,
        select: &job.select
    };

    execute_csv_job(csv_job);
    Ok(())
}

fn execute_csv_job(job: CSV) {
    job.run().unwrap()
}

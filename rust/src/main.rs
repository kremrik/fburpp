use fburpp::read_write::{RowIterator, RowWriter};
use fburpp::data::{select};
use csv;

fn main() {
    let path = "/home/kyle/projects/fburpp/rust/example.csv";
    let o_path = "/home/kyle/projects/fburpp/rust/example.out.csv";
    let col_names = vec!["foo", "bar", "baz"];
    let col_types = vec!["str", "int", "str"];
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let wtr = csv::Writer::from_path(o_path).unwrap();
    let records = rdr.records();

    let rowiter = RowIterator { 
        col_names: &col_names, 
        col_types: &col_types, 
        records 
    };

    let mut rowwriter = RowWriter { writer: wtr };
    let keep_cols = vec!["foo"];

    for row in rowiter {
        let new_row = select(row, &keep_cols);
        rowwriter.write(new_row).unwrap();
    }

    rowwriter.flush().unwrap();
}

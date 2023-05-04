#![allow(non_snake_case)]
use marine_rs_sdk::marine;



// use arrow::array::{Int32Array, ArrayRef};

use arrow_array::{Int32Array, ArrayRef};
use arrow_array::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs::File;
use std::sync::Arc;

// use parquet::arrow::util::pretty::print_batches;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
// use parquet::errors::Result;

use polars_core::prelude::*;
use polars_lazy::prelude::*;
use polars_io::prelude::*;

pub fn main() {}

#[marine]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}

#[marine]
pub fn polars_ex() {
    let df = df! {
        "column_a" => &[1, 2, 3, 4, 5],
        "column_b" => &["a", "b", "c", "d", "e"]
   }.unwrap();
   println!("df: {}", df);
}

/*
#[marine]
pub fn read_parquet() {

    let testdata = arrow::util::test_util::parquet_test_data();
    let path = format!("{testdata}/alltypes_plain.parquet");
    let file = File::open(path).unwrap();

    // Create a sync parquet reader with batch_size.
    // batch_size is the number of rows to read up to buffer once from pages, defaults to 1024
    let parquet_reader = ParquetRecordBatchReaderBuilder::try_new(file)?
        .with_batch_size(8192)
        .build()?;

    let mut batches = Vec::new();

    for batch in parquet_reader {
        batches.push(batch?);
    }

    print_batches(&batches).unwrap();
    Ok(())
}
*/


pub fn parquet_ex() {
    let ids = Int32Array::from(vec![1, 2, 3, 4]);
    let vals = Int32Array::from(vec![5, 6, 7, 8]);
    let batch = RecordBatch::try_from_iter(vec![
    ("id", Arc::new(ids) as ArrayRef),
    ("val", Arc::new(vals) as ArrayRef),
    ]).unwrap();

    let file = File::create("data.parquet").unwrap();

    // Default writer properties
    let props = WriterProperties::builder().build();

    let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props)).unwrap();

    writer.write(&batch).expect("Writing batch");

    // writer must be closed to write footer
    writer.close().unwrap();
}
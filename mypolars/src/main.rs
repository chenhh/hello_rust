use polars::prelude::*;
use std::fs::File;
use chrono::prelude::*;

// fn create_chunked_array() {
//     // use iterators
//     let ca: UInt32Chunked = (0..10).map(Some).collect();
//     println!("{:?}", ca);
//
//     // from slices
//     let ca = UInt32Chunked::new("foo", &[1, 2, 3]);
//     println!("{:?}", ca);
//
//     // use builders
//     let mut builder = PrimitiveChunkedBuilder::<UInt32Type>::new("foo", 10);
//     for value in 0..10 {
//         builder.append_value(value);
//     }
//     let ca = builder.finish();
//     println!("{:?}", ca);
// }

// fn create_series() {
//     // use iterators
//     let s: Series = (0..10).map(Some).collect();
//     println!("{:?}", s);
//
//     // from slices
//     let s = Series::new("foo", &[1, 2, 3]);
//     println!("{:?}", s);
//
//     // from a chunked-array
//     let ca = UInt32Chunked::new("foo", &[Some(1), None, Some(3)]);
//     let s = ca.into_series();
//     println!("{:?}", s);
// }

fn create_dataframe()  -> Result<(), Box<dyn std::error::Error>> {
    // use macro
    let df = df![
        "names" => ["a", "b", "c"],
        "values" => [1, 2, 3],
        "values_nulls" => [Some(1), None, Some(3)]
    ];
    println!("{:?}", df);

    // from a Vec<Series>
    // let s1 = Series::new("names".into(), &["a", "b", "c"]);
    // let s2 = Series::new("values".into(), &[Some(1), None, Some(3)]);
    // let df = DataFrame::new(vec![Column::Series(s1), s2]);
    // println!("{:?}", df);

    // v3
    let mut df: DataFrame = df!(
    "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
    "birthdate" => [
        NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
        NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
        NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
        NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
    ],
    "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
    "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
)
        .unwrap();
    println!("{}", df);

    // write df to csv
    let mut file = File::create("output.csv").expect("could not create file");
    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df)?;
    let df_csv = CsvReadOptions::default()
        .with_infer_schema_length(None)
        .with_has_header(true)
        .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
        .try_into_reader_with_file_path(Some("output.csv".into()))?
        .finish()?;
    println!("{}", df_csv);
    Ok(())
}


fn main() {
    // create_series();
    create_dataframe();
}

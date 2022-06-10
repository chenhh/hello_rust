use polars::prelude::*;
use polars::df;

fn create_chunked_array() {
    // use iterators
    let ca: UInt32Chunked = (0..10).map(Some).collect();
    println!("{:?}", ca);

    // from slices
    let ca = UInt32Chunked::new("foo", &[1, 2, 3]);
    println!("{:?}", ca);

    // use builders
    let mut builder = PrimitiveChunkedBuilder::<UInt32Type>::new("foo", 10);
    for value in 0..10 {
        builder.append_value(value);
    }
    let ca = builder.finish();
    println!("{:?}", ca);
}

fn create_series() {
    // use iterators
    let s: Series = (0..10).map(Some).collect();
    println!("{:?}", s);

    // from slices
    let s = Series::new("foo", &[1, 2, 3]);
    println!("{:?}", s);

    // from a chunked-array
    let ca = UInt32Chunked::new("foo", &[Some(1), None, Some(3)]);
    let s = ca.into_series();
    println!("{:?}", s);
}

fn create_dataframe(){
    // use macro
    let df = df! [
        "names" => ["a", "b", "c"],
        "values" => [1, 2, 3],
        "values_nulls" => [Some(1), None, Some(3)]
    ];
    println!("{:?}", df);

    // from a Vec<Series>
    let s1 = Series::new("names", &["a", "b", "c"]);
    let s2 = Series::new("values", &[Some(1), None, Some(3)]);
    let df = DataFrame::new(vec![s1, s2]);
    println!("{:?}", df);
}



fn main() {
    // create_series();
    create_dataframe();
}

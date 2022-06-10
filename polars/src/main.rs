use polars::prelude::*;

fn build_chunked_array(){
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


fn main() {


}

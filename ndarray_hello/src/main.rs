extern crate ndarray;
use ndarray::Array3;

fn main() {
    let mut a3 = Array3::<f64>::zeros((3, 4, 5));
    a3[[0, 0, 0]] = 0.0;
    a3[[1, 1, 1]] = 1.0;
    a3[[2, 2, 2]] = 2.0;
    println!("The 3D array is {:?}", a3);
}

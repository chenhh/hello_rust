use ndarray::{array, arr2, Array};

fn main() {
    let arr = array![[1.,2.,3.],[4.,5.,6.]];
    println!("{:?}", arr);

    let arr = arr2(&[[1.,2.,1.], [4.,5.,4.]]);
    println!("{:?}", arr);

    let arr = Array::range(0., 10., 0.5);
    println!("{:?}", arr);

    let arr = Array::linspace(0., 10., 11);
    println!("{:?}", arr);

    let arr = Array::ones((3, 4, 5));
    println!("{:?}", arr);
}

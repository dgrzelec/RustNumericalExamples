use ndarray::prelude::*;

const N: usize = 10;

type Matrix = Array2<i32>;
fn main() {

    // let a = array![(1..-1)];
    let mut b = Matrix::from_shape_fn((N,N), ignore_boundary);

    println!("{:?}", b);

    let mut diag = b.diag_mut();

    diag[0] = 1;

    println!("{:?}", b);


}

fn ignore_boundary((x, y): (usize,usize)) -> i32 {
    if x == 0 || x==(N-1) {
        return 0;
    }
    if y == 0 || y==(N-1) {
        return 0;
    }
    return 1;

}
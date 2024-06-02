use ndarray::Array;
use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::OwnedRepr;
use ndarray_rand::rand_distr::Pert;
use ndarray_rand::RandomExt;

fn main() {
    println!("{:8.1}", gen_rand_num(10));
}

fn gen_rand_num(n: usize) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> {
    return Array::random((1,n), Pert::new(0., 10., 8.).unwrap());
}

// generate floating point
// generate integer
// generate timestamp
// write to csv
// config for generating data by day
// uniform dist each day is getting bigger

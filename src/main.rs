use rust_linalg::*;

fn main() {
    let m1 = Matrix::new(vec![vec![1, 2, 1], vec![4, 1, 3]]);
    let ones: Matrix<i32> = Matrix::ones(3, 4);
    let m2 = m1.dot(&ones);
    println!("{}", ones);
    println!("{}", m2);
    println!("{}", m2.dot_scalar(2));
    println!("{}", m2.transpose());
}

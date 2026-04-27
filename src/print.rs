use crate::matrix::Matrix;

pub fn print_matrix(matrix: Matrix) {
    for i in 0..matrix.rows() {
        print_row(matrix.data[i].clone());
    }
}

fn print_row(row: Vec<f64>) {
    println!("{:?}", row);
}

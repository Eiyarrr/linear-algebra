use crate::matrix::Matrix;

pub fn row_addition(matrix: &mut Matrix, i: usize, j: usize, factor: f64) {
    if i == j {
        println!("Cannot do row addition where i = j");
        return;
    }
    for index in 0..matrix.data[i].len() {
        matrix.data[i][index] += matrix.data[i][index] * factor;
    }
}

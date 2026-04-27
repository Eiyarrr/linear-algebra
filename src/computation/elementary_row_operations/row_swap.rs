use crate::matrix::Matrix;

pub fn row_swap(matrix: &mut Matrix, row1: usize, row2: usize) {
    if row1 == row2 {
        println!("Rows to swap are the same!");
        return;
    }
    matrix.data.swap(row1, row2);
}

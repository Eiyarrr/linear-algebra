use crate::matrix::Matrix;

pub fn row_swap(matrix: &mut Matrix, row1: usize, row2: usize) {
    matrix.data.swap(row1, row2);
}

use crate::computation::elementary_row_operations::row_multiplication::row_multiplication;
use crate::computation::elementary_row_operations::row_swap::row_swap;
use crate::matrix::Matrix;

pub fn rref(matrix: &mut Matrix) {
    let rows = matrix.rows();
    let cols = matrix.columns();

    let mut r = 0;
    let mut c = 0;

    while r < rows && c < cols {
        // find row with a non-zero entry in current column
        let mut tmp = None;
        for i in r..rows {
            if matrix.data[r][c] != 0.0 {
                tmp = Some(i);
                break;
            }
        }

        // if no non-zero entires exist in collumn, increment by one
        if tmp.is_none() {
            c += 1;
            continue;
        }

        // swap non-zero entry row into current row
        let tmp = tmp.unwrap();
        row_swap(matrix, r, tmp);

        // normalize pivot point
        normalize_pivot(matrix, tmp, c);

        // set column to 0
        eliminate_collumn(matrix, tmp, c);

        r += 1;
        c += 1;
    }
}

fn normalize_pivot(matrix: &mut Matrix, row: usize, col: usize) {
    let pivot = matrix.data[row][col];
    row_multiplication(&mut matrix.row_mut(row), 1.0 / pivot);
}

fn eliminate_collumn(matrix: &mut Matrix, row: usize, piv: usize) {
    for r in 0..matrix.rows() {
        if r == row {
            continue;
        }
        if matrix.data[r][piv] == 0.0 {
            continue;
        }
        let d = -matrix.data[r][piv] / matrix.data[row][piv];
        for c in 0..matrix.columns() {
            matrix.data[r][c] = matrix.data[r][c] + d * matrix.data[row][c];
        }
    }
}

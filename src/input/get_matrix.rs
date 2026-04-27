use std::io;

use crate::matrix::Matrix;

pub fn get_matrix() -> Matrix {
    let mut input: String = String::from("");
    let mut data: Vec<Vec<f64>> = Vec::new();

    loop {
        let row_data = get_row();
        if row_data == vec![] {
            break;
        }
        data.push(row_data);
    }

    return Matrix::new(data);
}

fn get_row() -> Vec<f64> {
    let mut data: Vec<f64> = Vec::new();

    return data;
}

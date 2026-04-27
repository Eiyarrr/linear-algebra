use std::io;

use crate::matrix::Matrix;

pub fn get_matrix() -> Matrix {
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
    let mut input: String = String::from("");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line.");

    if input.trim().is_empty() {
        return vec![];
    }

    let mut curr_string: String = String::new();
    for i in input.chars() {
        if i.is_whitespace() {
            match curr_string.parse::<f64>() {
                Ok(n) => data.push(n),
                Err(e) => {
                    println!("Could not parse \"{}\", err: {}", curr_string, e);
                    data = vec![];
                }
            }
            curr_string = String::new();
        } else {
            curr_string.push(i);
        }
    }

    return data;
}

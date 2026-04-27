pub fn row_multiplication(row: &mut Vec<f64>, factor: f64) {
    if factor == 0.0 {
        println!("Factor cannot be 0");
        return;
    }
    for i in row {
        *i *= factor;
    }
}

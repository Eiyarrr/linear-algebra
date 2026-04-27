#[derive(Debug)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        return Self { data };
    }
    pub fn rows(&self) -> usize {
        return self.data.len();
    }
    pub fn columns(&self) -> usize {
        return self.data[0].len();
    }
    pub fn row(&self, i: usize) -> &Vec<f64> {
        return &self.data[i];
    }
    pub fn row_mut(&mut self, i: usize) -> &mut Vec<f64> {
        return &mut self.data[i];
    }
}

mod computation;
mod input;
mod print;
mod matrix;

use crate::input::input_loop::input_loop;
use crate::computation::algorithms::rref::rref;
use crate::print::print_matrix;

fn main() {
    let mut matrix = input_loop();
    rref(&mut matrix);
    print_matrix(matrix);
}

use std::fmt::{self, Display, Formatter};

// Following struct is for the activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )",
               self.0, self.1, self.2, self.3)
    }
}

// Swaps two elements in given matrix
fn transpose(m: Matrix) -> Matrix {
    // Return new tuple, with inner elements swapped
    Matrix(m.0, m.2, m.1, m.3)
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix))
}

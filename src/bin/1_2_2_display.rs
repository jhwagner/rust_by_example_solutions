use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

// Implement Display for Complex
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Determine whether imaginary component is position/negative
        let sign = if self.imag < 0.0 { '-' } else { '+' };
        // Display using i notation
        write!(f, "{real} {sign} {imag}i",
               real=self.real,
               sign=sign,
               imag=self.imag.abs())
    }
}

fn main() {
    let poscomp = Complex { real: 3.3, imag: 7.2 };
    let negcomp = Complex { real: 3.3, imag: -7.2 };

    println!("Display: {}", poscomp);
    println!("Display: {}", negcomp);
    println!("{:?}", poscomp);
}

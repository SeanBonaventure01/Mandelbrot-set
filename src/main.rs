use num_complex::Complex;

fn is_in_mandlebrot_set(complex_num: Complex<f64>, max_steps: i32) -> Option<i32> {
    let mut z = Complex::new(0.0, 0.0);
    let mut z_1 : Complex<f64>;

    for iteration in 0..max_steps {
        z_1 = z*z + complex_num;
        z = z_1;

        if z_1.norm_sqr() < 2.0 {
            return Some(iteration)
        }
    }
    None
}

fn main() 
{
    let z1 = Complex::new(3.0, 4.0);
    let z2 = Complex::new(1.0, 2.0);

    let sum = z1 * z2;
    println!("Hello, world, value is: {}", sum);
}

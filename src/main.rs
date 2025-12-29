pub mod mandelbrot;
use num_complex::Complex;

fn main() 
{
    let z1 = Complex::new(0.1, 0.1);
    let z2 = Complex::new(1.0, 2.0);

    let res_1 = mandelbrot::is_in_mandlebrot_set(z1, 25); 
    let res_2 = mandelbrot::is_in_mandlebrot_set(z2, 25); 
    println!("Res 1: {:?}, Res 2: {:?}", res_1, res_2);
}

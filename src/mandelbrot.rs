use num_complex::Complex;

pub fn is_in_mandlebrot_set(complex_num: Complex<f64>, max_steps: i32) -> Option<i32> {
    let mut z = Complex::new(0.0, 0.0);
    let mut z_1 : Complex<f64>;

    for iteration in 0..max_steps {
        z_1 = z*z + complex_num;
        z = z_1;

        if z_1.norm_sqr() > 2.0 {
            return Some(iteration)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_test()
    {
        // In set
        let c_0 = Complex::new(0.1, -0.26);
        // Well out of set
        let c_1 = Complex::new(-0.38, -0.82);
        // Close to set
        let c_2 = Complex::new(-0.38745, -0.6825);

        let res_0 = is_in_mandlebrot_set(c_0, 200);
        let res_1 = is_in_mandlebrot_set(c_1, 200);
        let res_2 = is_in_mandlebrot_set(c_2, 200);

        assert_eq!(res_0, None);
        assert!(res_1.unwrap() > 1);
        assert!(res_2.unwrap() > res_1.unwrap());
    }
}

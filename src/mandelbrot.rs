use num_complex::Complex;
use std::thread;

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

pub struct MandelbrotRenderParams {
    pixel_width: u32, 
    pixel_height: u32,
    mandelbrot_width: f64,
    mandelbrot_height: f64,
    center_x: f64,
    center_y: f64,
    steps: i32
}

impl MandelbrotRenderParams {
    pub fn new(pixel_width: u32, pixel_height: u32, mandelbrot_width: f64, mandelbrot_height: f64,
        center_x: f64, center_y: f64, steps: i32) -> Self {
        MandelbrotRenderParams {
            pixel_width,
            pixel_height,
            mandelbrot_width,
            mandelbrot_height,
            center_x,
            center_y,
            steps}
    }
}

pub fn compute_set_vals_multithreaded(params: MandelbrotRenderParams, num_threads: u32) -> Vec<Option<i32>> {
    let num_rows_per_thread = params.pixel_height/num_threads;
    let mut handles = vec![];

    for i in 0..num_threads
    {
        let start_index = params.pixel_width*i*num_rows_per_thread;
        let size = num_rows_per_thread;
        //let new_center_y = (i - num_threads/2)
        let chunk_params = MandelbrotRenderParams {
            pixel_width: params.pixel_width,
            pixel_height: num_rows_per_thread,
            mandelbrot_width: params.mandelbrot_width,
            mandelbrot_height: params.mandelbrot_height/(num_threads as f64),
            center_x: params.center_x,
            center_y: params.center_y,
            steps: 100
        };

        let handle = thread::spawn(move || {
            compute_set_vals_naive(chunk_params)
        });

        handles.push(handle);
    }

    handles.into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect()
}

pub fn compute_set_vals_naive(params: MandelbrotRenderParams) -> Vec<Option<i32>> {
    let mut mandelbrot_out : Vec<Option<i32>> = vec![None; (params.pixel_width*params.pixel_height) as usize];
    let center_point_x : i32 = (params.pixel_width as i32)/2;
    let center_point_y : i32 = (params.pixel_height as i32)/2;
    let scale_per_pixel_x = params.mandelbrot_width/(params.pixel_width as f64);
    let scale_per_pixel_y = params.mandelbrot_height/(params.pixel_height as f64);

    for x in 0..params.pixel_width {
        for y in 0..params.pixel_height {
            let x_pos : f64 = (((x as i32 - center_point_x) as f64)*scale_per_pixel_x) + params.center_x;
            let y_pos : f64 = (((y as i32 - center_point_y) as f64)*scale_per_pixel_y) + params.center_y;
            let complex_val = Complex::new(x_pos, y_pos);
            mandelbrot_out[(x + y*params.pixel_width) as usize] = is_in_mandlebrot_set(complex_val, params.steps);
        }
    }
    mandelbrot_out
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

pub mod mandelbrot;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use winit::dpi::LogicalSize;
use pixels::Pixels;
use pixels::SurfaceTexture;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

struct MandelbrotApp<'a>{
    window: Option<Arc<Window>>,
    pixel_buff: Option<Pixels<'a>>,
    window_width: u32,
    window_height: u32,
    mandelbrot_values: Vec<Option<i32>>,
}

impl<'a> MandelbrotApp<'a> {
    pub fn new() -> Self
    {
        MandelbrotApp {
            window: None,
            pixel_buff: None,
            window_width: 1280,
            window_height: 960,
            mandelbrot_values: vec![None; 0]
        }
    }
    
    pub fn with_resolution(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;
    }

    pub fn update_mandelbrot_values(&mut self, new_values : Vec<Option<i32>>)
    {
        self.mandelbrot_values = new_values;
    }


    pub fn draw(&mut self) 
    {
        println!("Drawing");
        if self.mandelbrot_values.len() != (self.window_width*self.window_height) as usize
        {
            println!("Inalid length!");
        }

        if let Some(buff) = &mut self.pixel_buff {
                // Clear the pixel buffer
                //let frame = buff.frame_mut();
                for (index, pixel) in buff.frame_mut().chunks_exact_mut(4).enumerate() {
                    match self.mandelbrot_values[index as usize] {
                        None => {
                            pixel[0] = 0x00;
                            pixel[1] = 0x00;
                            pixel[2] = 0x00;
                        },
                        Some(value) => {
                            pixel[0] = 0x00;
                            pixel[1] = 0x00;
                            pixel[2] = 0xff;
                        }
                    }

                    pixel[3] = 0xff; // A
                }
                buff.render();
        }
    }
}

impl<'a> ApplicationHandler for MandelbrotApp<'a> {
    // Required methods
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        let inner_size = winit::dpi::PhysicalSize::new(self.window_width, self.window_height);
        // Todo: Figure out how dpi actually works
        let window_attributes = Window::default_attributes()
            .with_title("Mandelbrot Set Generator")
            .with_inner_size(inner_size)
            .with_resizable(false)
            .with_blur(true);
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.window = Some(Arc::clone(&window));
        self.pixel_buff = Some(Pixels::new(inner_size.width, inner_size.height, SurfaceTexture::new(inner_size.width, inner_size.height, window)).unwrap());
        self.draw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    )
    {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close was requested; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                println!("Redrawing!");
            }
            _ => {
                //println!("Unhandled event: {:?}", event);
            },
        }
    }
}
fn main() 
{
    let event_loop = EventLoop::new().unwrap();
    let mut mandelbrot_app = MandelbrotApp::new();
    let start = Instant::now();
    let params = mandelbrot::MandelbrotRenderParams::new(
         mandelbrot_app.window_width,
         mandelbrot_app.window_height,
         3.0,
         2.0,
         -0.5,
         0.0,
         50);

    //mandelbrot_app.update_mandelbrot_values(mandelbrot::compute_set_vals_multithreaded(params, 1));
    mandelbrot_app.update_mandelbrot_values(mandelbrot::compute_set_vals_naive(params));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    let _ = event_loop.run_app(&mut mandelbrot_app);
}

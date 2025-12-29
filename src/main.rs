//pub mod mandelbrot;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use winit::dpi::LogicalSize;
use pixels::Pixels;
use pixels::SurfaceTexture;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

struct MandelbrotApp<'a>{
    window: Option<Window>,
    pixel_buff: Option<Pixels<'a>>
}

impl<'a> MandelbrotApp<'a> {
    pub fn new() -> Self
    {
        MandelbrotApp {
            window: None,
            pixel_buff: None
        }
    }

    pub fn draw(&mut self) 
    {
        println!("Drawing");

        if let Some(buff) = &mut self.pixel_buff {
                // Clear the pixel buffer
                //let frame = buff.frame_mut();
                for pixel in buff.frame_mut().chunks_exact_mut(4) {
                    pixel[0] = 0x00; // R
                    pixel[1] = 0x00; // G
                    pixel[2] = 0x00; // B
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
        let window_attributes = Window::default_attributes()
            .with_title("Mandelbrot Set Generator")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
            .with_resizable(false)
            .with_blur(true);
        let window = event_loop.create_window(window_attributes).unwrap();
        
        self.pixel_buff = Some(Pixels::new(WIDTH, HEIGHT, SurfaceTexture::new(WIDTH, HEIGHT, window)).unwrap());
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
    let _ = event_loop.run_app(&mut mandelbrot_app);
}

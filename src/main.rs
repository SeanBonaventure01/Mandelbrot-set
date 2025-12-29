//pub mod mandelbrot;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use winit::dpi::LogicalSize;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

struct MandelbrotApp{
    window: Option<Window>
}

impl MandelbrotApp {
    pub fn new() -> Self
    {
        MandelbrotApp {
            window: None
        }
    }
}

impl ApplicationHandler for MandelbrotApp {
    // Required methods
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        let window_attributes = Window::default_attributes()
            .with_title("Mandelbrot Set Generator")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
            .with_resizable(false)
            .with_blur(true);
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
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
            _ => {
                println!("Unhandled event: {:?}", event);
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

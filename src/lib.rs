use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use winit_input_helper::WinitInputHelper;

struct App {
    window: Option<Window>,
    title: String,
    width: u32,
    height: u32,
    pixels: Option<Pixels>
}
impl App {
    fn new<T:Into<String>>(title: T,width:u32,height:u32) -> Self {
        App{window: None,pixels:None, title: title.into(),width,height}
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes().with_title(&self.title);
        let window = event_loop.create_window(attributes).unwrap();
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(self.width, self.height, surface_texture).expect("Pixels::new failed")
        };
        self.pixels = Some(pixels);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                match self.pixels.as_mut() {
                    None => {}
                    Some(pixels) => {pixels.resize_surface(size.width, size.height).expect("resize failed")}
                }
            }
            WindowEvent::KeyboardInput {event, .. }=> {
                if event.state == winit::event::ElementState::Pressed {
                    println!("{:?}", event.text);
                }
            }
            WindowEvent::RedrawRequested => {
                match self.pixels.as_mut() {
                    None => {}
                    Some(pixels) => {pixels.render().expect("pixels render failed")}
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

pub fn make_main<T:Into<String>>(polling: bool, window_title: T, width:u32, height: u32) -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()?;
    if polling {
        event_loop.set_control_flow(ControlFlow::Poll);
    } else {
        event_loop.set_control_flow(ControlFlow::Wait);
    }
    let mut app = App::new(window_title,width,height);
    event_loop.run_app(&mut app)
}

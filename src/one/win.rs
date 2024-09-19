use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, Size},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

pub fn draw(title: &str, w: usize, h: usize) {
    let eloop = EventLoop::new().unwrap();
    let mut app = App::new(title, w, h);
    eloop.run_app(&mut app).unwrap();
}

#[derive(Default)]
pub struct App {
    attr: WindowAttributes,
    window: Option<Window>,
}

impl App {
    fn new(title: &str, w: usize, h: usize) -> App {
        let attr = Window::default_attributes()
            .with_title(title)
            .with_inner_size(Size::Logical(LogicalSize::new(w as f64, h as f64)));
        App {
            attr: attr,
            window: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(self.attr.clone()).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

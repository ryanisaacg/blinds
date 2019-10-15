use mint::Vector2;
use winit::event_loop::EventLoop;
use winit::window::{Window as WinitWindow, WindowBuilder};


pub struct Settings {
    /// The size of the window
    pub size: Vector2<f32>,
    /// If the cursor should be visible over the application
    pub show_cursor: bool,
    /// If the application should be fullscreen
    pub fullscreen: bool,
    /// The icon on the window or the favicon on the tab
    pub icon_path: Option<&'static str>, 
    /// How many samples to do for MSAA
    ///
    /// By default it is None; if it is Some, it should be a non-zero power of two
    ///
    /// Does nothing on web currently
    pub multisampling: Option<u16>,
    /// If the window can be resized by the user
    ///
    /// Does nothing on web
    pub resizable: bool
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            size: Vector2 { x: 1024.0, y: 768.0 },
            show_cursor: true,
            fullscreen: false,
            icon_path: None,
            multisampling: None,
            resizable: false
        }
    }
}

pub struct Window {
    window: WinitWindow
}

impl Window {
    pub(crate) fn new(eventloop: &EventLoop<()>, _settings: Settings) -> Window {
        let window = WindowBuilder::new()
            .build(eventloop)
            .expect("TODO");
        Window {
            window,
        }
    }
}

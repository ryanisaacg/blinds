#[cfg(feature = "gl")]
use glow::Context;
#[cfg(all(feature = "gl", not(target_arch = "wasm32")))]
use glutin::{PossiblyCurrent, WindowedContext};
use mint::Vector2;
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;
use winit::window::{Fullscreen, Window as WinitWindow, WindowBuilder};

/// The various options to pass to the Window and/or GL context
pub struct Settings {
    /// The size of the window
    pub size: Vector2<f32>,
    /// If the cursor should be visible over the application, or if the cursor should be hidden
    pub cursor_icon: Option<CursorIcon>,
    /// If the application should be fullscreen
    pub fullscreen: bool,
    /// The icon on the window or the favicon on the tab
    #[cfg(feature = "image")]
    pub icon_path: Option<&'static str>,
    /// How many samples to do for MSAA
    ///
    /// By default it is None; if it is Some, it should be a non-zero power of two
    ///
    /// Does nothing on web currently
    pub multisampling: Option<u16>,
    /// Enable or disable vertical sync
    ///
    /// Does nothing on web
    pub vsync: bool,
    /// If the window can be resized by the user
    ///
    /// Does nothing on web
    pub resizable: bool,
    /// The title of your application
    pub title: &'static str,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            size: Vector2 {
                x: 1024.0,
                y: 768.0,
            },
            cursor_icon: Some(CursorIcon::Default),
            fullscreen: false,
            #[cfg(feature = "image")]
            icon_path: None,
            multisampling: None,
            vsync: false,
            resizable: false,
            title: "",
        }
    }
}

/// The Window for your blinds application
pub struct Window(pub(crate) Arc<WindowContents>);

pub(crate) struct WindowContents {
    #[cfg(any(target_arch = "wasm32", not(feature = "gl")))]
    window: WinitWindow,
    #[cfg(all(feature = "gl", not(target_arch = "wasm32")))]
    window: WindowedContext<PossiblyCurrent>,
}

fn fullscreen_convert(fullscreen: bool, monitor: MonitorHandle) -> Option<Fullscreen> {
    if fullscreen {
        Some(Fullscreen::Borderless(monitor))
    } else {
        None
    }
}

#[cfg(all(feature = "stdweb", target_arch = "wasm32"))]
fn insert_canvas(
    window: &WinitWindow,
    settings: &Settings,
) -> std_web::web::html_element::CanvasElement {
    use std_web::traits::*;
    use std_web::web::document;
    use winit::platform::web::WindowExtStdweb;

    let canvas = window.canvas();
    let document = document();
    document
        .body()
        .expect("Document has no body node")
        .append_child(&canvas);

    if let Some(path) = settings.icon_path {
        let head = document.head().expect("Failed to find head node");
        let element = document
            .create_element("link")
            .expect("Failed to create link element");
        element
            .set_attribute("rel", "shortcut icon")
            .expect("Failed to create favicon element");
        element
            .set_attribute("type", "image/png")
            .expect("Failed to create favicon element");
        element
            .set_attribute("href", path)
            .expect("Failed to create favicon element");
        head.append_child(&element);
    }

    canvas
}

#[cfg(all(feature = "web-sys", target_arch = "wasm32"))]
fn insert_canvas(window: &WinitWindow, settings: &Settings) -> web_sys::HtmlCanvasElement {
    use winit::platform::web::WindowExtWebSys;
    let canvas = window.canvas();
    let window = web_sys::window().expect("Failed to obtain window");
    let document = window.document().expect("Failed to obtain document");

    document
        .body()
        .expect("Document has no body node")
        .append_child(&canvas)
        .expect("Failed to insert canvas");

    if let Some(path) = settings.icon_path {
        let head = document.head().expect("Failed to find head node");
        let element = document
            .create_element("link")
            .expect("Failed to create link element");
        element
            .set_attribute("rel", "shortcut icon")
            .expect("Failed to create favicon element");
        element
            .set_attribute("type", "image/png")
            .expect("Failed to create favicon element");
        element
            .set_attribute("href", path)
            .expect("Failed to create favicon element");
        head.append_child(&element).expect("Failed to add favicon");
    }

    canvas
}

#[cfg(all(not(feature = "gl"), not(target_arch = "wasm32")))]
fn insert_canvas(_window: &WinitWindow, _settings: &Settings) {}

fn settings_to_wb(el: &EventLoop<()>, settings: &Settings) -> WindowBuilder {
    #[cfg(feature = "image")]
    let icon = settings.icon_path.map(|path| {
        let img = image::open(path).expect("Failed to load image");
        let rgba = img.to_rgba();
        let (width, height) = rgba.dimensions();
        let buffer = rgba.into_raw();

        winit::window::Icon::from_rgba(buffer, width, height).expect("Bad image data")
    });
    #[cfg(not(feature = "image"))]
    let icon = None;
    WindowBuilder::new()
        .with_inner_size(LogicalSize {
            width: settings.size.x as f64,
            height: settings.size.y as f64,
        })
        .with_resizable(settings.resizable)
        .with_fullscreen(fullscreen_convert(
            settings.fullscreen,
            el.primary_monitor(),
        ))
        .with_title(settings.title)
        .with_window_icon(icon)
}

impl WindowContents {
    pub(crate) fn new(el: &EventLoop<()>, settings: Settings) -> WindowContents {
        let wb = settings_to_wb(el, &settings);
        #[cfg(any(not(feature = "gl"), target_arch = "wasm32"))]
        let window = {
            let window = wb.build(el).expect("Failed to create window");
            insert_canvas(&window, &settings);
            WindowContents { window }
        };
        #[cfg(all(feature = "gl", not(target_arch = "wasm32")))]
        let window = {
            let mut cb = glutin::ContextBuilder::new().with_vsync(settings.vsync);
            if let Some(msaa) = settings.multisampling {
                cb = cb.with_multisampling(msaa);
            }
            let window = cb.build_windowed(wb, el).expect("Failed to create window");
            let window = unsafe { window.make_current().expect("Failed to acquire GL context") };
            WindowContents { window }
        };
        window.set_cursor_icon(settings.cursor_icon);

        window
    }

    #[cfg(feature = "gl")]
    pub(crate) fn new_gl(el: &EventLoop<()>, settings: Settings) -> (WindowContents, Context) {
        let window = WindowContents::new(el, settings);

        #[cfg(target_arch = "wasm32")]
        let ctx = {
            #[cfg(feature = "stdweb")]
            let ctx = {
                use webgl_stdweb::WebGLRenderingContext;
                use winit::platform::web::WindowExtStdweb;

                window
                    .window
                    .canvas()
                    .get_context::<WebGLRenderingContext>()
                    .expect("Failed to acquire a WebGL rendering context")
            };
            #[cfg(feature = "web-sys")]
            let ctx = {
                use wasm_bindgen::JsCast;
                use winit::platform::web::WindowExtWebSys;

                window
                    .window
                    .canvas()
                    .get_context("webgl")
                    .expect("Failed to acquire a WebGL rendering context")
                    .expect("Failed to acquire a WebGL rendering context")
                    .dyn_into::<web_sys::WebGlRenderingContext>()
                    .expect("WebGL context of unexpected type")
            };

            glow::Context::from_webgl1_context(ctx)
        };
        #[cfg(not(target_arch = "wasm32"))]
        let ctx = {
            glow::Context::from_loader_function(|s| window.window.get_proc_address(s) as *const _)
        };

        (window, ctx)
    }

    pub fn set_cursor_icon(&self, icon: Option<CursorIcon>) {
        match icon {
            Some(icon) => {
                self.window().set_cursor_visible(true);
                self.window().set_cursor_icon(icon.into());
            }
            None => {
                self.window().set_cursor_visible(false);
            }
        }
    }

    pub fn size(&self) -> Vector2<f32> {
        let size = self.window().inner_size();
        Vector2 {
            x: size.width as f32,
            y: size.height as f32,
        }
    }

    pub fn set_size(&self, size: Vector2<f32>) {
        self.window().set_inner_size(LogicalSize {
            width: size.x as f64,
            height: size.y as f64,
        });
    }

    pub fn set_title(&self, title: &str) {
        self.window().set_title(title);
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        self.window().set_fullscreen(fullscreen_convert(
            fullscreen,
            self.window().current_monitor(),
        ));
    }

    pub(crate) fn resize(&self, _size: &LogicalSize) {
        #[cfg(all(feature = "gl", not(target_arch = "wasm32")))]
        self.window
            .resize(_size.to_physical(self.window.window().hidpi_factor()));
    }

    #[cfg(feature = "gl")]
    pub fn present(&self) {
        #[cfg(all(feature = "gl", not(target_arch = "wasm32")))]
        self.window.swap_buffers().expect("Failed to swap buffers")
    }

    pub fn scale(&self) -> f32 {
        self.window().hidpi_factor() as f32
    }

    #[inline]
    fn window(&self) -> &WinitWindow {
        #[cfg(any(not(feature = "gl"), target_arch = "wasm32"))]
        return &self.window;
        #[cfg(all(feature = "gl", not(target_arch = "wasm32")))]
        return self.window.window();
    }
}

impl Window {
    /// Set the cursor icon to some value, or set it to invisible (None)
    pub fn set_cursor_icon(&self, icon: Option<CursorIcon>) {
        self.0.set_cursor_icon(icon);
    }

    /// Get the size of the window in logical units
    ///
    /// On a high-dpi display, this doesn't correspond to physical pixels and must be multiplied by
    /// [`scale`] when passing sizes to functions like `glViewport`.
    ///
    /// [`scale`]: Window::scale_factor
    pub fn size(&self) -> Vector2<f32> {
        self.0.size()
    }

    /// The DPI scale factor of the window
    ///
    /// For a good example of DPI scale factors, see the [`winit docs`] on the subject
    ///
    /// [`winit docs`]: winit::dpi
    pub fn scale_factor(&self) -> f32 {
        self.0.scale()
    }

    /// Set the size of the inside of the window in logical units
    pub fn set_size(&self, size: Vector2<f32>) {
        self.0.set_size(size);
    }

    /// Set the title of the window
    ///
    /// Currently does nothing on the web, but will set the browser tab title in a future release
    /// (see issue #5)
    pub fn set_title(&self, title: &str) {
        self.0.set_title(title);
    }

    /// Set if the window should be fullscreen or not
    ///
    /// On desktop, it will instantly become fullscreen (borderless windowed on Windows and Linux,
    /// and fullscreen on macOS). On web, it will become fullscreen after the next user
    /// interaction, due to browser API restrictions.
    pub fn set_fullscreen(&self, fullscreen: bool) {
        self.0.set_fullscreen(fullscreen);
    }

    #[cfg(feature = "gl")]
    /// Draw the OpenGL frame to the screen
    ///
    /// If vsync is enabled, this will block until the frame is completed on desktop. On web, there
    /// is no way to control vsync, or to manually control presentation, so this function is a
    /// no-op.
    pub fn present(&self) {
        self.0.present();
    }
}

/// The options for the cursor icon
pub enum CursorIcon {
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
}

impl Into<winit::window::CursorIcon> for CursorIcon {
    fn into(self) -> winit::window::CursorIcon {
        use CursorIcon::*;
        match self {
            Default => winit::window::CursorIcon::Default,
            Crosshair => winit::window::CursorIcon::Crosshair,
            Hand => winit::window::CursorIcon::Hand,
            Arrow => winit::window::CursorIcon::Arrow,
            Move => winit::window::CursorIcon::Move,
            Text => winit::window::CursorIcon::Text,
            Wait => winit::window::CursorIcon::Wait,
            Help => winit::window::CursorIcon::Help,
            Progress => winit::window::CursorIcon::Progress,
            NotAllowed => winit::window::CursorIcon::NotAllowed,
            ContextMenu => winit::window::CursorIcon::ContextMenu,
            Cell => winit::window::CursorIcon::Cell,
            VerticalText => winit::window::CursorIcon::VerticalText,
            Alias => winit::window::CursorIcon::Alias,
            Copy => winit::window::CursorIcon::Copy,
            NoDrop => winit::window::CursorIcon::NoDrop,
            Grab => winit::window::CursorIcon::Grab,
            Grabbing => winit::window::CursorIcon::Grabbing,
            AllScroll => winit::window::CursorIcon::AllScroll,
            ZoomIn => winit::window::CursorIcon::ZoomIn,
            ZoomOut => winit::window::CursorIcon::ZoomOut,
            EResize => winit::window::CursorIcon::EResize,
            NResize => winit::window::CursorIcon::NResize,
            NeResize => winit::window::CursorIcon::NeResize,
            NwResize => winit::window::CursorIcon::NwResize,
            SResize => winit::window::CursorIcon::SResize,
            SeResize => winit::window::CursorIcon::SeResize,
            SwResize => winit::window::CursorIcon::SwResize,
            WResize => winit::window::CursorIcon::WResize,
            EwResize => winit::window::CursorIcon::EwResize,
            NsResize => winit::window::CursorIcon::NsResize,
            NeswResize => winit::window::CursorIcon::NeswResize,
            NwseResize => winit::window::CursorIcon::NwseResize,
            ColResize => winit::window::CursorIcon::ColResize,
            RowResize => winit::window::CursorIcon::RowResize,
        }
    }
}

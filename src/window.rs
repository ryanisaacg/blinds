#[cfg(feature = "gl")]
use glow::Context;
#[cfg(feature = "gl")]
use glutin::{WindowContext};
use mint::Vector2;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;
use winit::window::{Fullscreen, Window as WinitWindow, WindowBuilder};

pub struct Settings {
    /// The size of the window
    pub size: Vector2<f32>,
    /// If the cursor should be visible over the application, or if the cursor should be hidden
    pub cursor_icon: Option<CursorIcon>,
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
    pub resizable: bool,
    /// The title of your application
    pub title: &'static str,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            size: Vector2 { x: 1024.0, y: 768.0 },
            cursor_icon: Some(CursorIcon::Default),
            fullscreen: false,
            icon_path: None,
            multisampling: None,
            resizable: false,
            title: "",
        }
    }
}

pub struct Window {
    #[cfg(not(feature="gl"))]
    window: WinitWindow,
    #[cfg(feature="gl")]
    window: WindowedContext<PossiblyCurrent>,
    #[cfg(feature="gl")]
    ctx: Context,
}

fn fullscreen_convert(fullscreen: bool, monitor: MonitorHandle) -> Option<Fullscreen> {
    if fullscreen {
        Some(Fullscreen::Borderless(monitor))
    } else {
        None
    }
}

impl Window {
    pub(crate) fn new(eventloop: &EventLoop<()>, settings: Settings) -> Window {
        let wb = WindowBuilder::new()
            .with_inner_size(LogicalSize {
                width: settings.size.x as f64,
                height: settings.size.y as f64
            })
            .with_resizable(settings.resizable)
            .with_fullscreen(fullscreen_convert(settings.fullscreen, eventloop.primary_monitor()))
            .with_title(settings.title);
        // TODO: respect window icons
        // TODO: insert the canvas
        #[cfg(not(feature="gl"))] let window = {
            let window = wb
                .build(eventloop)
                .expect("TODO");
            Window {
                window,
            }
        };
        #[cfg(feature="gl")] let window = {
            // TODO: initialize glow
            let window = WindowBuilder::new()
                .build(eventloop)
                .expect("TODO");
            Window {
                window,
            }
        };
        window.set_cursor_icon(settings.cursor_icon);

        window
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
        self.window().set_fullscreen(fullscreen_convert(fullscreen, self.window().current_monitor()));
    }

    #[cfg(feature="gl")]
    pub fn present(&self) {

    }

    #[cfg(feature="gl")]
    pub fn ctx(&mut self) -> &mut Context {
        &mut self.ctx
    }

    #[inline]
    fn window(&self) -> &WinitWindow {
        #[cfg(not(feature="gl"))]
        return &self.window;
        #[cfg(feature="gl")]
        return window.window();
    }
}

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

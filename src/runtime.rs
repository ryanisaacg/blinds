use crate::event::*;
use crate::{EventStream, Window, WindowBuilder};
use futures_util::task::LocalSpawnExt;
use futures_executor::LocalPool;
use mint::Vector2;
use std::future::Future;
use winit::event::Event as WinitEvent;
use winit::event_loop::EventLoop;

// TODO: add gilrs events
// TODO: add timing handling
// TODO: provide custom windowbuilder


pub struct Runtime {
    stream: EventStream,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            stream: EventStream::new(),
        }
    }

    pub fn init(self, wb: WindowBuilder) -> (Window, EventStream) {
        (Window::new(wb), self.stream)
    }

    pub fn run<F, T>(app: F) -> !
            where T: 'static + Future<Output = ()>, F: FnOnce(Runtime) -> T {
        let runtime = Runtime::new();
        let buffer = runtime.stream.buffer();


        let mut pool = LocalPool::new();
        let mut spawner = pool.spawner();
        spawner.spawn_local(app(runtime)).expect("Failed to start application");

        let event_loop = EventLoop::new::<>();

        event_loop.run(move |event, _, _| {
            match event {
                WinitEvent::WindowEvent { event, .. } => {
                    // TODO: convert Winit event to quick-lifecycle event
                    //buffer.borrow_mut().push(Event::Input(event));
                    if let Some(event) = convert(event) {
                        buffer.borrow_mut().push(event);
                    }
                }
                WinitEvent::LoopDestroyed => {
                    buffer.borrow_mut().push(Event::Close);
                    pool.run_until_stalled();
                }
                WinitEvent::EventsCleared => {
                    buffer.borrow_mut().push(Event::Update);
                    pool.run_until_stalled();
                }
                _ => ()
            }
        })
    }
}

fn convert(event: winit::event::WindowEvent) -> Option<Event> {
    use winit::event::WindowEvent::*;
    Some(match event {
        RedrawRequested => Event::Draw,
        Resized(ls) => Event::Window(WindowEvent::Resized(ls_to_vec(ls))),
        ReceivedCharacter(c) => Event::Input(InputEvent::ReceivedCharacter(c)),
        Focused(f) => Event::Window(WindowEvent::Focused(f)),
        KeyboardInput { input: winit::event::KeyboardInput {
            state, virtual_keycode: Some(key), modifiers, ..
        }, .. } => Event::Input(InputEvent::KeyboardInput {
            key: key.into(),
            modifiers: modifiers.into(),
            state: state.into(),
        }),
        CursorMoved { device_id, position, modifiers } => Event::Input(
            InputEvent::MouseMoved {
                pointer: Pointer(device_id),
                position: lp_to_vec(position),
                modifiers: modifiers.into(),
            }
        ),
        CursorEntered { device_id } => Event::Input(InputEvent::MouseEntered {
            pointer: Pointer(device_id),
        }),
        CursorLeft { device_id } => Event::Input(InputEvent::MouseLeft {
            pointer: Pointer(device_id),
        }),
        MouseWheel { device_id, delta, modifiers, .. } => Event::Input(
            InputEvent::MouseWheel {
                pointer: Pointer(device_id),
                delta: delta.into(),
                modifiers: modifiers.into()
            }
        ),
        MouseInput { device_id, button, state, modifiers, ..} => Event::Input(
            InputEvent::MouseInput {
                pointer: Pointer(device_id),
                state: state.into(),
                button: button.into(),
                modifiers: modifiers.into(),
            }
        ),
        Destroyed => Event::Close,
        _ => return None
    })
}

fn ls_to_vec(ls: winit::dpi::LogicalSize) -> Vector2<f32> {
    Vector2 {
        x: ls.width as f32,
        y: ls.height as f32
    }
}

fn lp_to_vec(ls: winit::dpi::LogicalPosition) -> Vector2<f32> {
    Vector2 {
        x: ls.x as f32,
        y: ls.y as f32,
    }
}

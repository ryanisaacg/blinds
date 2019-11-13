use crate::event::*;
use crate::{EventBuffer, EventStream, Window, Settings};
use futures_util::task::LocalSpawnExt;
use futures_executor::LocalPool;
use mint::Vector2;
use std::cell::RefCell;
use std::sync::Arc;
use std::future::Future;
use winit::event::{Event as WinitEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run<F, T>(settings: Settings, app: F) -> !
        where T: 'static + Future<Output = ()>, F: 'static + FnOnce(Window, EventStream) -> T {
    let stream = EventStream::new();
    let buffer = stream.buffer();

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop, settings);
    let pool = LocalPool::new();
    pool.spawner().spawn_local(app(window, stream)).expect("Failed to start application");

    do_run(event_loop, pool, buffer)
}

#[cfg(feature = "gl")]
use glow::Context;

#[cfg(feature = "gl")]
pub fn run_gl<T, F>(settings: Settings, app: F) -> !
        where T: 'static + Future<Output = ()>, F: 'static + FnOnce(Window, Context, EventStream) -> T {
    let stream = EventStream::new();
    let buffer = stream.buffer();

    let event_loop = EventLoop::new();
    let (window, ctx) = Window::new_gl(&event_loop, settings);
    let pool = LocalPool::new();
    pool.spawner().spawn_local(app(window, ctx, stream)).expect("Failed to start application");

    do_run(event_loop, pool, buffer)
}

fn do_run(event_loop: EventLoop<()>, mut pool: LocalPool, buffer: Arc<RefCell<EventBuffer>>) -> ! {
    event_loop.run(move |event, _, ctrl| {
        match event {
            WinitEvent::WindowEvent { event, .. } => {
                if let winit::event::WindowEvent::CloseRequested = &event {
                    *ctrl = ControlFlow::Exit;
                }
                if let Some(event) = convert(event) {
                    buffer.borrow_mut().push(event);
                }
            }
            WinitEvent::LoopDestroyed | WinitEvent::EventsCleared => {
                if pool.try_run_one() {
                    *ctrl = ControlFlow::Exit;
                }
            }
            _ => ()
        }
    })
}

fn convert(event: winit::event::WindowEvent) -> Option<Event> {
    use winit::event::WindowEvent::*;
    Some(match event {
        Resized(ls) => Event::Resized(ls_to_vec(ls)),
        ReceivedCharacter(c) => Event::ReceivedCharacter(c),
        Focused(f) => Event::Focused(f),
        KeyboardInput { input: winit::event::KeyboardInput {
            state, virtual_keycode: Some(key), modifiers, ..
        }, .. } => Event::KeyboardInput {
            key: key.into(),
            modifiers: modifiers.into(),
            state: state.into(),
        },
        CursorMoved { device_id, position, modifiers } => Event::MouseMoved {
            pointer: Pointer(device_id),
            position: lp_to_vec(position),
            modifiers: modifiers.into(),
        },
        CursorEntered { device_id } => Event::MouseEntered {
            pointer: Pointer(device_id),
        },
        CursorLeft { device_id } => Event::MouseLeft {
            pointer: Pointer(device_id),
        },
        MouseWheel { device_id, delta, modifiers, .. } => Event::MouseWheel {
            pointer: Pointer(device_id),
            delta: delta.into(),
            modifiers: modifiers.into()
        },
        MouseInput { device_id, button, state, modifiers, ..} => Event::MouseInput {
            pointer: Pointer(device_id),
            state: state.into(),
            button: button.into(),
            modifiers: modifiers.into(),
        },
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

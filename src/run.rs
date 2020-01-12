use crate::event::*;
use crate::{EventBuffer, EventContext, EventStream, Settings, Window, WindowContents};
use futures_executor::LocalPool;
use futures_util::task::LocalSpawnExt;
use mint::Vector2;
use std::cell::RefCell;
use std::future::Future;
use std::sync::Arc;
use winit::event::Event as WinitEvent;
use winit::event_loop::{ControlFlow, EventLoop};

/// The entry point for a blinds-based application
///
/// `run` acts as the executor for your async application, and it handles your event loop on both
/// desktop and web. It is a single-threaded executor, because wasm doesn't support thread at the
/// moment.
///
/// Currently blinds only supports one window, and `settings` determines how it will be
/// constructed.
///
/// If you want a GL context, use [`run_gl`] instead. GL contexts require additional work when
/// creating a window, and therefore are not created by defualt.
///
/// [`run_gl`]: run_gl
pub fn run<F, T>(settings: Settings, app: F) -> !
where
    T: 'static + Future<Output = ()>,
    F: 'static + FnOnce(Window, EventStream<Event>) -> T,
{
    let stream = EventStream::new();
    let buffer = stream.buffer();

    let event_loop = EventLoop::new();
    let window = Arc::new(WindowContents::new(&event_loop, settings));
    let pool = LocalPool::new();
    pool.spawner()
        .spawn_local(app(Window(window.clone()), stream))
        .expect("Failed to start application");

    do_run(event_loop, window, pool, buffer)
}

// Prototype entry point supporting custom events and tasks
pub fn run_custom<F, T, E>(settings: Settings, app: F) -> !
where
    T: 'static + Future<Output = ()>,
    F: 'static + FnOnce(Window, EventContext<E>) -> T,
    E: 'static + From<Event>,
{
    let stream = EventStream::new();
    let buffer = stream.buffer();

    let event_loop = EventLoop::new();
    let window = Arc::new(WindowContents::new(&event_loop, settings));
    let pool = LocalPool::new();

    // FIXME: setup a new() function
    let spawner = pool.spawner();
    let context = EventContext { stream, spawner };
    pool.spawner()
        .spawn_local(app(Window(window.clone()), context))
        .expect("Failed to start application");
    
    do_run(event_loop, window, pool, buffer)
}

#[cfg(feature = "gl")]
use glow::Context;

#[cfg(feature = "gl")]
/// The entry point for a blinds-based application using OpenGL
///
/// `run_gl` acts the same as [`run`] except it provides a [`glow`] context
///
/// [`run`]: run
/// [`glow`]: glow
pub fn run_gl<T, F>(settings: Settings, app: F) -> !
where
    T: 'static + Future<Output = ()>,
    F: 'static + FnOnce(Window, Context, EventStream) -> T,
{
    let stream = EventStream::new();
    let buffer = stream.buffer();

    let event_loop = EventLoop::new();
    let (window, ctx) = WindowContents::new_gl(&event_loop, settings);
    let window = Arc::new(window);
    let pool = LocalPool::new();
    pool.spawner()
        .spawn_local(app(Window(window.clone()), ctx, stream))
        .expect("Failed to start application");

    do_run(event_loop, window, pool, buffer)
}

fn do_run<E>(
    event_loop: EventLoop<()>,
    window: Arc<WindowContents>,
    mut pool: LocalPool,
    buffer: Arc<RefCell<EventBuffer<E>>>,
) -> ! where E: 'static + From<Event> {
    #[cfg(feature = "gilrs")]
    let mut gilrs = gilrs::Gilrs::new();

    let mut finished = pool.try_run_one();

    event_loop.run(move |event, _, ctrl| {
        match event {
            WinitEvent::NewEvents(winit::event::StartCause::Init) => {
                *ctrl = ControlFlow::Poll;
            }
            WinitEvent::WindowEvent { event, .. } => {
                if let winit::event::WindowEvent::CloseRequested = &event {
                    *ctrl = ControlFlow::Exit;
                }
                if let winit::event::WindowEvent::Resized(size) = &event {
                    window.resize(*size);
                }
                if let Some(event) = convert_winit_window(event) {
                    buffer.borrow_mut().push(event.into());
                }
            }
            WinitEvent::DeviceEvent { event, .. } => {
                if let Some(event) = convert_winit_device(event) {
                    buffer.borrow_mut().push(event.into());
                }
            }
            WinitEvent::LoopDestroyed | WinitEvent::MainEventsCleared => {
                buffer.borrow_mut().mark_ready();
                #[cfg(feature = "gilrs")]
                process_gilrs_events(&mut gilrs, &buffer);
                finished = pool.try_run_one();
            }
            _ => (),
        }
        if finished {
            *ctrl = ControlFlow::Exit;
        }
    })
}

#[cfg(feature = "gilrs")]
fn process_gilrs_events<E>(
    gilrs: &mut Result<gilrs::Gilrs, gilrs::Error>,
    buffer: &Arc<RefCell<EventBuffer<E>>>,
) where E: From<Event> {
    if let Ok(gilrs) = gilrs.as_mut() {
        while let Some(ev) = gilrs.next_event() {
            if let Some(ev) = convert_gilrs(ev) {
                buffer.borrow_mut().push(ev.into());
            }
        }
    }
}

fn convert_winit_device(event: winit::event::DeviceEvent) -> Option<Event> {
    use winit::event::DeviceEvent::*;
    Some(match event {
        ModifiersChanged(state) => Event::ModifiersChanged {
            modifiers: state.into(),
        },
        _ => return None,
    })
}

fn convert_winit_window(event: winit::event::WindowEvent) -> Option<Event> {
    use winit::event::WindowEvent::*;
    Some(match event {
        Resized(ls) => Event::Resized(ps_to_vec(ls)),
        ScaleFactorChanged { scale_factor, .. } => Event::ScaleFactorChanged(scale_factor as f32),
        ReceivedCharacter(c) => Event::ReceivedCharacter(c),
        Focused(f) => Event::Focused(f),
        KeyboardInput {
            input:
                winit::event::KeyboardInput {
                    state,
                    virtual_keycode: Some(key),
                    ..
                },
            ..
        } => Event::KeyboardInput {
            key: key.into(),
            state: state.into(),
        },
        CursorMoved {
            device_id,
            position,
            ..
        } => Event::MouseMoved {
            pointer: Pointer(device_id),
            position: pp_to_vec(position),
        },
        CursorEntered { device_id } => Event::MouseEntered {
            pointer: Pointer(device_id),
        },
        CursorLeft { device_id } => Event::MouseLeft {
            pointer: Pointer(device_id),
        },
        MouseWheel {
            device_id, delta, ..
        } => Event::MouseWheel {
            pointer: Pointer(device_id),
            delta: delta.into(),
        },
        MouseInput {
            device_id,
            button,
            state,
            ..
        } => Event::MouseInput {
            pointer: Pointer(device_id),
            state: state.into(),
            button: button.into(),
        },
        _ => return None,
    })
}

#[cfg(feature = "gilrs")]
fn convert_gilrs(event: gilrs::Event) -> Option<Event> {
    use gilrs::ev::EventType::*;
    let gilrs::Event { id, event, .. } = event;
    let event = match event {
        ButtonPressed(btn, _) => convert_gilrs_button(btn).map(|button| GamepadEvent::Button {
            button,
            state: ElementState::Pressed,
        }),
        ButtonRepeated(_, _) => None,
        ButtonReleased(btn, _) => convert_gilrs_button(btn).map(|button| GamepadEvent::Button {
            button,
            state: ElementState::Released,
        }),
        ButtonChanged(_, _, _) => None,
        AxisChanged(axis, value, _) => {
            convert_gilrs_axis(axis).map(|axis| GamepadEvent::Axis { axis, value })
        }
        Connected => Some(GamepadEvent::Connected),
        Disconnected => Some(GamepadEvent::Disconnected),
        Dropped => None,
    };
    event.map(|event| Event::GamepadEvent {
        id: GamepadId(id),
        event,
    })
}

#[cfg(feature = "gilrs")]
fn convert_gilrs_button(event: gilrs::ev::Button) -> Option<GamepadButton> {
    use gilrs::ev::Button::*;
    Some(match event {
        South => GamepadButton::South,
        East => GamepadButton::East,
        North => GamepadButton::North,
        West => GamepadButton::West,
        LeftTrigger => GamepadButton::LeftShoulder,
        LeftTrigger2 => GamepadButton::LeftShoulder,
        RightTrigger => GamepadButton::RightShoulder,
        RightTrigger2 => GamepadButton::RightTrigger,
        Select => GamepadButton::Select,
        Start => GamepadButton::Start,
        LeftThumb => GamepadButton::LeftStick,
        RightThumb => GamepadButton::RightStick,
        DPadUp => GamepadButton::DPadUp,
        DPadDown => GamepadButton::DPadDown,
        DPadLeft => GamepadButton::DPadLeft,
        DPadRight => GamepadButton::DPadRight,

        C | Z | Unknown | Mode => return None,
    })
}

#[cfg(feature = "gilrs")]
fn convert_gilrs_axis(axis: gilrs::ev::Axis) -> Option<GamepadAxis> {
    use gilrs::ev::Axis::*;

    Some(match axis {
        LeftStickX => GamepadAxis::LeftStickX,
        LeftStickY => GamepadAxis::LeftStickY,
        RightStickX => GamepadAxis::RightStickX,
        RightStickY => GamepadAxis::RightStickY,

        LeftZ | RightZ | DPadX | DPadY | Unknown => return None,
    })
}

fn ps_to_vec<P: winit::dpi::Pixel>(ls: winit::dpi::PhysicalSize<P>) -> Vector2<f32> {
    Vector2 {
        x: ls.width.cast(),
        y: ls.height.cast(),
    }
}

fn pp_to_vec<P: winit::dpi::Pixel>(ls: winit::dpi::PhysicalPosition<P>) -> Vector2<f32> {
    Vector2 {
        x: ls.x.cast(),
        y: ls.y.cast(),
    }
}

use crate::event::*;
use crate::{EventBuffer, EventStream, Window, WindowContents, Settings};
use futures_util::task::LocalSpawnExt;
use futures_executor::LocalPool;
use mint::Vector2;
use std::cell::RefCell; use std::sync::Arc; use std::future::Future;
use winit::event::{Event as WinitEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run<F, T>(settings: Settings, app: F) -> !
        where T: 'static + Future<Output = ()>, F: 'static + FnOnce(Window, EventStream) -> T {
    let stream = EventStream::new();
    let buffer = stream.buffer();

    let event_loop = EventLoop::new();
    let window = Arc::new(WindowContents::new(&event_loop, settings));
    let pool = LocalPool::new();
    pool.spawner().spawn_local(app(Window(window.clone()), stream)).expect("Failed to start application");

    do_run(event_loop, window, pool, buffer)
}

#[cfg(feature = "gl")]
use glow::Context;

#[cfg(feature = "gl")]
pub fn run_gl<T, F>(settings: Settings, app: F) -> !
        where T: 'static + Future<Output = ()>, F: 'static + FnOnce(Window, Context, EventStream) -> T {
    let stream = EventStream::new();
    let buffer = stream.buffer();

    let event_loop = EventLoop::new();
    let (window, ctx) = WindowContents::new_gl(&event_loop, settings);
    let window = Arc::new(window);
    let pool = LocalPool::new();
    pool.spawner().spawn_local(app(Window(window.clone()), ctx, stream)).expect("Failed to start application");

    do_run(event_loop, window, pool, buffer)
}

fn do_run(event_loop: EventLoop<()>, window: Arc<WindowContents>, mut pool: LocalPool, buffer: Arc<RefCell<EventBuffer>>) -> ! {
    #[cfg(feature = "gilrs")]
    let mut gilrs = gilrs::Gilrs::new().unwrap();

    event_loop.run(move |event, _, ctrl| {
        match event {
            WinitEvent::WindowEvent { event, .. } => {
                if let winit::event::WindowEvent::CloseRequested = &event {
                    *ctrl = ControlFlow::Exit;
                }
                if let winit::event::WindowEvent::Resized(size) = &event {
                    window.resize(size);
                }
                if let Some(event) = convert_winit(event) {
                    buffer.borrow_mut().push(event);
                }
            }
            WinitEvent::LoopDestroyed | WinitEvent::EventsCleared => {
                #[cfg(feature = "gilrs")]
                while let Some(ev) = gilrs.next_event() {
                    if let Some(ev) = convert_gilrs(ev) {
                        buffer.borrow_mut().push(ev);
                    }
                }
                if pool.try_run_one() {
                    *ctrl = ControlFlow::Exit;
                } else {
                    window.present();
                }
            }
            _ => ()
        }
    })
}

fn convert_winit(event: winit::event::WindowEvent) -> Option<Event> {
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

#[cfg(feature = "gilrs")]
fn convert_gilrs(event: gilrs::Event) -> Option<Event> {
    use gilrs::ev::EventType::*;
    let gilrs::Event { id, event, .. } = event;
    let event = match event {
        ButtonPressed(btn, _) => convert_gilrs_button(btn)
            .map(|button| GamepadEvent::Button {
                button,
                state: ElementState::Pressed
            }),
        ButtonRepeated(_, _) => None,
        ButtonReleased(btn, _) => convert_gilrs_button(btn)
            .map(|button| GamepadEvent::Button {
                button,
                state: ElementState::Released
            }),
        ButtonChanged(_, _, _) => None,
        AxisChanged(axis, value, _) => convert_gilrs_axis(axis)
            .map(|axis| GamepadEvent::Axis {
                axis,
                value
            }),
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

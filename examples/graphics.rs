/*use quick_lifecycle::traits::*;
use quick_lifecycle::{Event, Runtime, WindowBuilder, WindowEvent};

async fn app(env: Runtime) {
    let (window, gl) = env.init_gl(WindowBuilder::new());

    while let Some(event) = window.next().await {
        if let Event::Input(WindowEvent::RedrawRequested) = event {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
        }
        println!("{:?}", event);
    }
}

fn main() {
    Runtime::run(app);
}
TODO: add GL context loading support
*/

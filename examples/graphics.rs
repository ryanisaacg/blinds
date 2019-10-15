use quick_lifecycle::traits::*;
use quick_lifecycle::{EventStream, Window, Settings, run_gl};

async fn app(window: Window, mut events: EventStream) {
    while let Some(event) = events.next().await {
        if let Event::Draw = event {
            unsafe {
                window.ctx().clear_color(0.5, 0.5, 0.5, 1.0);
            }
        }
    }
}

fn main() {
    run(Settings::default(), app);
}

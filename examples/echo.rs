use quick_lifecycle::traits::*;
use quick_lifecycle::{Runtime, WindowBuilder};

async fn app(env: Runtime) {
    let (_window, mut events) = env.init(WindowBuilder::default());

    while let Some(event) = events.next().await {
        println!("{:?}", event);
    }
}

fn main() {
    Runtime::run(app);
}

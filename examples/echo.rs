#![feature(async_closure)]

use blinds::traits::*;
use blinds::{Settings, run};


fn main() {
    run(Settings::default(), async move |_window, mut events| {
        while let Some(frame) = events.next().await {
            for event in frame {
                println!("{:?}", event);
            }
        }
    });
}

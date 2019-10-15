#![feature(async_closure)]

use quick_lifecycle::traits::*;
use quick_lifecycle::{EventStream, Window, Settings, run};


fn main() {
    run(Settings::default(), async move |window, mut events| {
        while let Some(event) = events.next().await {
            //println!("{:?}", event);
        }
    });
}

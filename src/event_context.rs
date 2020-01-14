use crate::EventStream;
use futures_executor::LocalSpawner;
use futures_util::task::LocalSpawnExt;
use std::future::Future;

// #[derive(Clone)]
pub struct EventContext<E> {
    spawner: LocalSpawner,
    stream: EventStream<E>,
}

// All of these custom Clone are smelly
impl<E> Clone for EventContext<E> {
    fn clone(&self) -> Self {
        EventContext {
            spawner: self.spawner.clone(),
            stream: self.stream.clone(),
        }
    }
}

impl<E> EventContext<E> {
    pub fn new(spawner: LocalSpawner, stream: EventStream<E>) -> Self {
        EventContext { spawner, stream }
    }

    pub fn spawn<F, T>(&self, task: F)
    where
        T: 'static + Future<Output = ()>,
        F: 'static + FnOnce(EventContext<E>) -> T,
    {
        let context_copy: EventContext<E> = self.clone();
        self.spawner
            .spawn_local(task(context_copy))
            .expect("Failed to start application");
    }

    pub fn stream(&mut self) -> &mut EventStream<E> {
        &mut self.stream
    }

    pub fn dispatch(&self, event: E) {
        self.stream.buffer().borrow_mut().push(event)
    }
}

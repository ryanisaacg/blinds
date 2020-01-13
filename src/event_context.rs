
use crate::EventStream;
use futures_executor::LocalSpawner;
use std::future::Future;
use futures_util::task::LocalSpawnExt;

// FIXME: struct fields shouldn't be public
// #[derive(Clone)]
pub struct EventContext<E> {
    pub spawner: LocalSpawner,
    pub stream: EventStream<E>,
}

// All of these custom Clone are smelly
impl <E> Clone for EventContext<E> {
    fn clone(&self) -> Self {
        EventContext {
            spawner: self.spawner.clone(),
            stream: self.stream.clone(),
        }
    }
}

// TODO: should probably start using Result more frquently?
impl <'a, E> EventContext<E> {
    pub fn spawn<F, T>(&'a self, task: F) 
    where 
        T: 'static + Future<Output = ()>,
        F: 'static + FnOnce(EventContext<E>) -> T {
            let context_copy: EventContext<E> = self.clone();
            self.spawner
            .spawn_local(task(context_copy))
            .expect("Failed to start application");
    }

    pub fn dispatch(&'a self, event: E) {
        self.stream.buffer().borrow_mut().push(event)
    }
}
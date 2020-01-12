
use crate::EventStream;
use futures_executor::LocalSpawner;

// FIXME: struct fields shouldn't be public
pub struct EventContext<E> {
    pub spawner: LocalSpawner,
    pub stream: EventStream<E>,
}
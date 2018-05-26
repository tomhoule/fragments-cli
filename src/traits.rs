use failure::Error;
use prost::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait Context {
    fn persist<A: Aggregate>(event: Event<Aggregate = A>);
}

pub trait Aggregate: Default {
    type Event;
    const NAME: &'static str;

    fn apply(self, event: Self::Event) -> Self;
}

pub trait Event {
    type Aggregate;
}

#[derive(Debug)]
pub struct CommandResponse {
    pub stream_id: Uuid,
    pub seq_no: u32,
}

pub trait Command: Serialize + Default + Message {
    fn handle(self, context: &::Context) -> Result<CommandResponse, Error>;
}

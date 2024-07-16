use std::fmt::Debug;

pub struct AgentDetail {
    pub name: String,
    pub id: Option<String>,
}


#[async_trait::async_trait]
pub trait AgentBase {
    async fn run_(&self, inputs: Vec<u8>);
}

#[async_trait::async_trait]
pub trait MessageHandler: Send + Sync + Debug {
    async fn on_message(&self, agent_id: String, data: Vec<u8>, time: u64);
}

#[async_trait::async_trait]
pub trait Processor: Send + Sync + Debug {
    async fn run(&self, input: Vec<u8>) -> ();
}
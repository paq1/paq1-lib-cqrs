use async_trait::async_trait;
use paq1_lib_error_handler::prelude::ResultErr;

pub enum CommandHandlers<STATE, COMMAND, EVENT, CONTEXT> {
    Create(Box<dyn CommandHandlerCreate<STATE, COMMAND, EVENT, CONTEXT>>),
    Update(Box<dyn CommandHandlerUpdate<STATE, COMMAND, EVENT, CONTEXT>>),
}

#[async_trait]
pub trait CommandHandlerCreate<STATE, COMMAND, EVENT, CONTEXT>: Send + Sync {
    fn name(&self) -> String;
    async fn on_command(
        &self,
        id: &String,
        command: &COMMAND,
        context: &CONTEXT,
    ) -> ResultErr<EVENT>;
}

#[async_trait]
pub trait CommandHandlerUpdate<STATE, COMMAND, EVENT, CONTEXT>: Send + Sync {
    fn name(&self) -> String;
    async fn on_command(
        &self,
        id: &String,
        state: &STATE,
        command: &COMMAND,
        context: &CONTEXT,
    ) -> ResultErr<EVENT>;
}

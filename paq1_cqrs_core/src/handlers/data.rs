use async_trait::async_trait;
use paq1_authn_core::data::context::ContextCore;
use paq1_lib_error_handler::prelude::ResultErr;

pub enum CommandHandler<STATE, COMMAND, EVENT> {
    Create(Box<dyn CommandHandlerCreate<STATE, COMMAND, EVENT>>),
    Update(Box<dyn CommandHandlerUpdate<STATE, COMMAND, EVENT>>),
}

impl<STATE, COMMAND, EVENT> CommandHandler<STATE, COMMAND, EVENT> {
    pub fn get_name(&self) -> String {
        match self {
            CommandHandler::Create(h) => h.name(),
            CommandHandler::Update(h) => h.name(),
        }
    }
}

#[async_trait]
pub trait CommandHandlerCreate<STATE, COMMAND, EVENT>: Send + Sync {
    fn name(&self) -> String;
    async fn on_command(&self, entity_id: &String, command: &COMMAND, ctx: &ContextCore) -> ResultErr<EVENT>;
}

#[async_trait]
pub trait CommandHandlerUpdate<STATE, COMMAND, EVENT>: Send + Sync {
    fn name(&self) -> String;
    async fn on_command(&self, entity_id: &String, state: &STATE, command: &COMMAND, ctx: &ContextCore) -> ResultErr<EVENT>;
}

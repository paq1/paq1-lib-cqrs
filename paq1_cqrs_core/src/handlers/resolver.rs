use crate::handlers::data::CommandHandler;

pub trait CommandHandlerResolver<STATE, COMMAND, EVENT>: Send + Sync {

    fn resolve(&self, handler_name: &str) -> Option<&CommandHandler<STATE, COMMAND, EVENT>>;
}

pub struct DefaultCommandHandlerResolver<STATE, COMMAND, EVENT> {
    pub handlers: Vec<CommandHandler<STATE, COMMAND, EVENT>>,
}

impl<STATE, COMMAND, EVENT> DefaultCommandHandlerResolver<STATE, COMMAND, EVENT> {
    pub fn new() -> Self {
        Self {
            handlers: vec![],
        }
    }

    pub fn add_handler(&mut self, handler: CommandHandler<STATE, COMMAND, EVENT>) -> &mut Self {
        self.handlers.push(handler);
        self
    }
}

impl<STATE, COMMAND, EVENT> CommandHandlerResolver<STATE, COMMAND, EVENT> for DefaultCommandHandlerResolver<STATE, COMMAND, EVENT> {
    fn resolve(&self, handler_name: &str) -> Option<&CommandHandler<STATE, COMMAND, EVENT>> {
        self
            .handlers
            .iter()
            .find(|handler| {
                handler.get_name().as_str() == handler_name
            })
    }
}

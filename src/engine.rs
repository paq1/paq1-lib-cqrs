use std::sync::Arc;
use crate::command_handlers::CommandHandlers;
use crate::reducer::Reducer;
use paq1_lib_storage_core::repositories::repository::Repository;

pub struct Engine<STATE: Clone, COMMAND, EVENT, CONTEXT> {
    pub handlers: Vec<CommandHandlers<STATE, COMMAND, EVENT, CONTEXT>>,
    pub reducer: Reducer<EVENT, STATE>,
    pub store: Arc<dyn Repository<STATE, String>>, // TODO : wrap into entity
    pub event: Arc<dyn Repository<EVENT, String>>, // TODO : wrap into event
}

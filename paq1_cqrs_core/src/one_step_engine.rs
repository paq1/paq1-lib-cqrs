use async_trait::async_trait;
use paq1_authn_core::data::context::ContextCore;
use paq1_lib_error_handler::prelude::*;

#[async_trait]
pub trait OneStepEngine<STATE, CMD, EVT>: Send + Sync {
    async fn execute(&self, entity_id: &str, handler_name: &str, cmd: &CMD, ctx: &ContextCore) -> ResultErr<(EVT, STATE, String, String)>;
}

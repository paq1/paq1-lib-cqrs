use async_trait::async_trait;
use std::sync::Arc;
use paq1_authn_core::data::context::ContextCore;
use paq1_lib_error_handler::prelude::{Error, ErrorWithCode, ResultErr};
use paq1_storage_core::prelude::entity::Entity;
use paq1_storage_core::prelude::event::Event;
use paq1_storage_core::prelude::repository::Repository;
use uuid::Uuid;
use paq1_cqrs_core::handlers::data::CommandHandler;
use paq1_cqrs_core::handlers::resolver::CommandHandlerResolver;
use paq1_cqrs_core::one_step_engine::OneStepEngine;
use paq1_cqrs_core::reducer::Reducer;

pub struct DefaultOneStepEngine<STATE, CMD, EVT> {
    pub store: Arc<dyn Repository<Entity<STATE>, String>>,
    pub journal: Arc<dyn Repository<Event<EVT>, String>>,
    pub handler_fetcher: Arc<dyn CommandHandlerResolver<STATE, CMD, EVT>>,
    pub reducer: Arc<dyn Reducer<EVT, STATE>>,
}

#[async_trait]
impl<STATE, CMD, EVT> OneStepEngine<STATE, CMD, EVT> for DefaultOneStepEngine<STATE, CMD, EVT>
where
    EVT: Clone + Send + Sync,
    STATE: Clone + Send + Sync,
    CMD: Send + Sync,
{
    async fn execute(
        &self,
        entity_id: &str,
        handler_name: &str,
        cmd: &CMD,
        ctx: &ContextCore
    ) -> ResultErr<(EVT, STATE, String, String)> {
        let entity_id_string = entity_id.to_string();

        let handler = self
            .handler_fetcher
            .resolve(handler_name)
            .ok_or_else(|| Error::Failure(ErrorWithCode::new("NOHAND", 400, "Handler not found")))?;


        let (evt, new_state) = match handler {
            CommandHandler::Create(e) => {
                let evt = e.on_command(&entity_id_string.clone(), cmd, ctx).await?;
                let new_state = self.reducer.reduce(None, &evt).ok_or(Error::Failure(ErrorWithCode {
                    code: "ERRRSTC".to_string(),
                    status: 400,
                    title: "bad transition for event".to_string(),
                    description: None,
                    problems: vec![]
                }))?;
                let _ = self.store.insert(&Entity {
                    data: new_state.clone(),
                    version: 1,
                    id: entity_id_string.clone(),
                }).await?;
                (evt, new_state)

            },
            CommandHandler::Update(e) => {
                let old_state = self.store
                    .fetch_one(&entity_id_string).await?
                    .ok_or_else(|| Error::Failure(ErrorWithCode::new("CMPF01", 404, "pas de data trouvé")))?;
                let evt = e.on_command(&entity_id_string.clone(), &old_state.data, cmd, ctx).await?;
                let new_state = self.reducer.reduce(Some(&old_state.data), &evt).ok_or(Error::Failure(ErrorWithCode {
                    code: "ERRRSTU".to_string(),
                    status: 400,
                    title: "bad transition for event".to_string(),
                    description: None,
                    problems: vec![]
                }))?;

                let _ = self.store.update(&Entity {
                    data: new_state.clone(),
                    version: old_state.version + 1,
                    ..old_state
                }).await?;
                (evt, new_state)

            }
        };

        let evt_id = Uuid::new_v4().to_string();

        let event_journal: Event<EVT> = Event {
            data: evt.clone(),
            id: evt_id.clone(),
            entity_id: entity_id_string.clone(),
            version: 1
        };

        let _ = self.journal.insert(&event_journal).await?;

        Ok((evt, new_state, evt_id, entity_id.to_string()))
    }
}

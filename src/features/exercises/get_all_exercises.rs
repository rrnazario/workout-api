use crate::connect_async;
use crate::models::model::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use mediator::{async_trait, AsyncRequestHandler, DefaultAsyncMediator, Request};

pub struct GetAllExercisesQuery();
impl Request<Vec<Cliente>> for GetAllExercisesQuery {}

pub struct GetAllExercisesQueryHandler(DefaultAsyncMediator);

impl GetAllExercisesQueryHandler {
    pub fn new(mediator: DefaultAsyncMediator) -> GetAllExercisesQueryHandler {
        Self(mediator)
    }
}

#[async_trait]
impl AsyncRequestHandler<GetAllExercisesQuery, Vec<Cliente>> for GetAllExercisesQueryHandler {
    async fn handle(&mut self, _req: GetAllExercisesQuery) -> Vec<Cliente> {
        let connection = &mut connect_async().await;

        let client = cliente
            .filter(excluido.eq(0))
            .select(Cliente::as_select())
            .load::<Cliente>(connection)
            .await
            .expect("GetAllOrcamentosQueryHandler_clientes");

        client
    }
}

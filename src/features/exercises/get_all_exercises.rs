use crate::schema::exercise::deleted;
use crate::{infrastructure::persistence::connect_async, schema::exercise};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use mediator::{async_trait, AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};

pub struct GetAllExercisesQuery;
#[derive(Queryable, Serialize, Deserialize, Selectable, Default)]
#[diesel(table_name = exercise)]
pub struct Exercise {
    id: i32,
    name: String,
}
#[derive(Serialize, Deserialize, Default)]
pub struct ExerciseResponse {
    exercises: Option<Vec<Exercise>>,
}

impl Request<ExerciseResponse> for GetAllExercisesQuery {}

pub struct GetAllExercisesQueryHandler(DefaultAsyncMediator);

impl GetAllExercisesQueryHandler {
    pub fn new(mediator: DefaultAsyncMediator) -> GetAllExercisesQueryHandler {
        Self(mediator)
    }
}

#[async_trait]
impl AsyncRequestHandler<GetAllExercisesQuery, ExerciseResponse> for GetAllExercisesQueryHandler {
    async fn handle(&mut self, _req: GetAllExercisesQuery) -> ExerciseResponse {
        let connection = &mut connect_async().await;

        let exercises = exercise::table
            .filter(deleted.eq(0))
            .select(Exercise::as_select())
            .load::<Exercise>(connection)
            .await
            .expect("GetAllExercisesQuery");

        ExerciseResponse {
            exercises: Some(exercises),
        }
    }
}

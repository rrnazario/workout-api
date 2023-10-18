use rocket::{
    get,
    serde::json::{serde_json::json, Value},
};

use mediator::AsyncMediator;

use crate::{infrastructure::mediator::get_mediator, models::dtos::NetworkResponse};

use super::get_all_exercises::GetAllExercisesQuery;

#[get("/exercises")]
pub async fn get_all_exercises() -> Result<Value, NetworkResponse> {
    let mut mediator = get_mediator();
    let result = mediator
        .send(GetAllExercisesQuery {})
        .await
        .unwrap_or_default();

    Ok(json!(result))
}

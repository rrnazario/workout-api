use mediator::DefaultAsyncMediator;

use crate::features::exercises::get_all_exercises::GetAllExercisesQueryHandler;

pub fn get_mediator() -> DefaultAsyncMediator {
    let mediator = DefaultAsyncMediator::builder()
        //clientes
        .add_handler_deferred(|m| GetAllExercisesQueryHandler::new(m))        
        .build();

    mediator
}
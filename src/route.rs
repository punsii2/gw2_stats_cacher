use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        todo_list_handler,
        todo_create_handler,
    },
    model,
};

pub fn create_router() -> Router {
    let db = model::todo_db();

    Router::new()
        .route(
            "/api/todos",
            get(todo_list_handler).post(todo_create_handler),
        )
        .with_state(db)
}

mod api;
mod question_list;

use api::*;
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use question_list::{QuestionList, Question};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // Initialize the question list with pre-loaded data
    let question_list = QuestionList::new();

    // Wrap the question list in Arc and RwLock for thread-safe access across async tasks
    let shared_question_list = Arc::new(RwLock::new(question_list));

    // Configure the Axum router with routes for CRUD operations
    let app = Router::new()
        .route("/questions", get(fetch_all_questions).post(create_question))
        .route("/questions/:id", get(fetch_question).put(update_question).delete(remove_question))
        .layer(axum::extract::Extension(shared_question_list));

    // Define the address and port to serve on localhost
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Run the Axum server
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn fallback_not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Endpoint does not exist")
}

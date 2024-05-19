use crate::*;
use axum::{
    extract::{Path, Extension},
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn fetch_all_questions(
    Extension(question_list): Extension<Arc<RwLock<QuestionList>>>
) -> impl IntoResponse {
    let questions = question_list.read().await.get_all_questions();
    (StatusCode::OK, Json(questions))
}

pub async fn create_question(
    Extension(question_list): Extension<Arc<RwLock<QuestionList>>>,
    Json(new_question): Json<Question>
) -> impl IntoResponse {
    question_list.write().await.add_question(new_question);
    (StatusCode::CREATED, "Question added successfully")
}

pub async fn fetch_question(
    Extension(question_list): Extension<Arc<RwLock<QuestionList>>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    match question_list.read().await.find_question(&id) {
        Some(question) => (StatusCode::OK, Json(question)).into_response(),
        None => (StatusCode::NOT_FOUND, "Question not found").into_response(),
    }
}

pub async fn update_question(
    Extension(question_list): Extension<Arc<RwLock<QuestionList>>>,
    Path(id): Path<String>,
    Json(updated_question): Json<Question>
) -> impl IntoResponse {
    match question_list.write().await.update_question(&id, updated_question) {
        Ok(_) => (StatusCode::OK, "Question updated successfully").into_response(),
        Err(err) => (StatusCode::NOT_FOUND, err).into_response(),
    }
}

pub async fn remove_question(
    Extension(question_list): Extension<Arc<RwLock<QuestionList>>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    if question_list.write().await.remove_question(&id).is_some() {
        (StatusCode::OK, "Question deleted successfully").into_response()
    } else {
        (StatusCode::NOT_FOUND, "Question not found").into_response()
    }
}

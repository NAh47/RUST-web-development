use crate::*;

/// Fetch all questions from the database.
pub async fn fetch_all_questions(
    State(question_list): State<Arc<RwLock<QuestionList>>>
) -> Response {
    let questions = question_list.read().await.get_all_questions();
    (StatusCode::OK, Json(questions)).into_response()
}

/// Create a new question using data provided in the JSON body.
pub async fn create_question(
    State(question_list): State<Arc<RwLock<QuestionList>>>,
    Json(new_question): Json<Question>
) -> Response {
    question_list.write().await.add_question(new_question);
    (StatusCode::CREATED, "Question added successfully").into_response()
}

/// Retrieve a specific question by its ID.
pub async fn fetch_question(
    State(question_list): State<Arc<RwLock<QuestionList>>>,
    Path(id): Path<String>
) -> Response {
    match question_list.read().await.find_question(&id) {
        Some(question) => (StatusCode::OK, Json(question)).into_response(),
        None => (StatusCode::NOT_FOUND, "Question not found").into_response(),
    }
}

/// Update a question identified by its ID with new data provided in the JSON body.
pub async fn update_question(
    State(question_list): State<Arc<RwLock<QuestionList>>>,
    Path(id): Path<String>,
    Json(updated_question): Json<Question>
) -> Response {
    match question_list.write().await.update_question(&id, updated_question) {
        Ok(_) => (StatusCode::OK, "Question updated successfully").into_response(),
        Err(err) => (StatusCode::NOT_FOUND, err).into_response(),
    }
}

/// Delete a question by its ID.
pub async fn remove_question(
    State(question_list): State<Arc<RwLock<QuestionList>>>,
    Path(id): Path<String>
) -> Response {
    if question_list.write().await.remove_question(&id).is_some() {
        (StatusCode::OK, "Question deleted successfully").into_response()
    } else {
        (StatusCode::NOT_FOUND, "Question not found").into_response()
    }
}

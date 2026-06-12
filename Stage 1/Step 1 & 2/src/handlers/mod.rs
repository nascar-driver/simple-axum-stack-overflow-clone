use crate::models::*;
use axum::{http::StatusCode, response::IntoResponse, Json};

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    let data = Question {
        title: question.title,
        description: question.description,
    };
    (StatusCode::CREATED, Json(data))
}

pub async fn read_questions() -> impl IntoResponse {
    let questions = vec![
        Question {
            title: "Question 1".to_string(),
            description: "Description 1".to_string(),
        },
        Question {
            title: "Question 2".to_string(),
            description: "Description 2".to_string(),
        },
    ];
    (StatusCode::OK, Json(questions))
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) -> impl IntoResponse {
    (StatusCode::OK, Json(question_uuid))
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above
pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    let data = Answer {
        question_uuid: answer.question_uuid,
        content: answer.content,
    };
    (StatusCode::CREATED, Json(data))
}

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above
pub async fn read_answers(Json(question_uuid): Json<QuestionId>) -> impl IntoResponse {
    let answers = vec![
        AnswerDetail {
            answer_uuid: "answer_uuid_1".to_string(),
            question_uuid: question_uuid.question_uuid.clone(),
            content: "content_1".to_string(),
            created_at: "created_at_1".to_string(),
        },
        AnswerDetail {
            answer_uuid: "answer_uuid_2".to_string(),
            question_uuid: question_uuid.question_uuid.clone(),
            content: "content_2".to_string(),
            created_at: "created_at_2".to_string(),
        },
    ];
    (StatusCode::OK, Json(answers))
}

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above
pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>) -> impl IntoResponse {
    (StatusCode::OK, Json(answer_uuid))
}
